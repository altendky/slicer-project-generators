#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.12"
# dependencies = ["jsonschema==4.25.1", "rfc8785==0.1.4"]
# ///
"""Verify the neutral handoff, then optionally exercise the published binary.

AGPL-3.0-only; authored 2026-10-12. Neutral identity payloads adapt the MIT
Onshape Export contract; see public LICENSE-MIT-service and local provenance.
Fixtures and target bytes are used only in this generator-local experiment.
"""
import argparse
import copy
from datetime import datetime, timezone
import hashlib
from importlib.metadata import version
import json
from pathlib import Path
import shutil
import stat
import subprocess
import sys

from jsonschema import Draft202012Validator
import rfc8785

REPO = Path(__file__).resolve().parents[4]
PUBLIC = REPO / "handoffs/bambu-studio/v0.1.0-v1"
RELEASE = REPO / "provenance/bambu-studio/releases/v0.1.0"
NEUTRAL = REPO / "crates/bambu-studio/neutral"
PROTOCOL = Draft202012Validator(json.loads((NEUTRAL / "generator-protocol-v1.schema.json").read_bytes()))
SETTINGS = Draft202012Validator(json.loads((NEUTRAL / "generator-settings-v2.schema.json").read_bytes()))


def reject_constant(value):
    raise ValueError(f"Non-JSON number: {value}")


def unique_members(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"Duplicate JSON member: {key}")
        result[key] = value
    return result


def decode(data):
    return json.loads(data, object_pairs_hook=unique_members, parse_constant=reject_constant)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def file_sha(path):
    with path.open("rb") as file:
        return hashlib.file_digest(file, "sha256").hexdigest()


def identity(domain, payload):
    return sha(rfc8785.dumps({"domain": domain, "payload": payload}))


def static_identity(deployment):
    return sha(b"onshape-export-deployed-generator-v1\0" + rfc8785.dumps(deployment))


def expected_bindings(deployment, settings_identity):
    result = {k: v for k, v in deployment.items() if k not in {"packageSha256", "binarySha256", "protocolVersion"}}
    result["settingsIdentity"] = settings_identity
    return result


def bind_request(request, deployment):
    request["protocolVersion"] = deployment["protocolVersion"]
    request["expectedIdentities"] = expected_bindings(deployment, request["settings"]["settingsIdentity"])
    payload = {k: v for k, v in request.items() if k != "invocationIdentity"}
    request["invocationIdentity"] = identity("generator-invocation-v1", payload)


def check_handoff():
    data = (PUBLIC / "handoff.json").read_bytes()
    handoff = decode(data)
    if data != rfc8785.dumps(handoff):
        raise ValueError("Handoff must be exact JCS UTF-8 without newline or BOM")
    digest_line = (PUBLIC / "handoff.sha256").read_text()
    if digest_line != file_sha(PUBLIC / "handoff.json") + "  handoff.json\n":
        raise ValueError("Detached handoff digest mismatch")
    schema = decode((PUBLIC / "handoff.schema.json").read_bytes())
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)
    validator.validate(handoff)
    deployment = handoff["immutableDeployment"]
    capabilities = deployment["capabilityIdentities"]
    if capabilities != sorted(set(capabilities)):
        raise ValueError("Capabilities must be sorted and unique")
    service_schema_bytes = (PUBLIC / "deployed-generator.schema.json").read_bytes()
    if sha(service_schema_bytes) != handoff["neutralContracts"]["deployedGeneratorSchema"]["sha256"]:
        raise ValueError("Pinned service deployment schema mismatch")
    service_schema = decode(service_schema_bytes)
    service_validator = Draft202012Validator(service_schema)
    if set(deployment) != set(service_schema["required"]) - {"executablePath"}:
        raise ValueError("Handoff must supply exactly the service immutable fields")
    # These paths are synthetic examples for contract testing, never metadata.
    for executable in ["/example/first/generator", "/example/relocated/generator"]:
        config = {**deployment, "executablePath": executable}
        service_validator.validate(config)
        if static_identity({k: v for k, v in config.items() if k != "executablePath"}) != static_identity(deployment):
            raise ValueError("Relocation changed immutable deployment identity")
    changed_field_count = 0
    for field, value in deployment.items():
        altered = copy.deepcopy(deployment)
        altered[field] = value + "-changed" if isinstance(value, str) else value + ["z.synthetic@1"] if isinstance(value, list) else value + 1
        if static_identity(altered) == static_identity(deployment):
            raise ValueError(f"Immutable identity omitted field {field}")
        if validator.is_valid({**handoff, "immutableDeployment": altered}):
            raise ValueError(f"Schema accepted unreviewed binding {field}")
        changed_field_count += 1
    # Regression checks cover strict decoding and nested object closure.
    for invalid in [b'{"a":1,"a":2}', b'{"a":NaN}', b'{} {}']:
        try:
            decode(invalid)
        except ValueError:
            pass
        else:
            raise ValueError("Strict JSON decoding accepted invalid document")
    for route in [[], ["interface"], ["release", "package"], ["immutableDeployment"]]:
        altered = copy.deepcopy(handoff)
        current = altered
        for key in route:
            current = current[key]
        current["unexpected"] = True
        if validator.is_valid(altered):
            raise ValueError("Schema accepted an unknown field")
    for name in ["protocolSchema", "settingsSchema"]:
        path = NEUTRAL / ("generator-protocol-v1.schema.json" if name == "protocolSchema" else "generator-settings-v2.schema.json")
        if file_sha(path) != handoff["neutralContracts"][name]["sha256"]:
            raise ValueError("Neutral schema bytes differ from pinned service reference")
    settings_schema_identity = identity("onshape-export-generator-settings-schema-v2", decode((NEUTRAL / "generator-settings-v2.schema.json").read_bytes()))
    if deployment["settingsSchemaIdentity"] != settings_schema_identity:
        raise ValueError("Settings schema identity mismatch")
    original = decode((RELEASE / "identities.json").read_bytes())
    for key in ["packageIdentity", "buildIdentity", "binaryIdentity", "provenanceSetIdentity"]:
        if deployment[key] != original[key]:
            raise ValueError("Original release identity changed")
    if deployment["packageSha256"] != original["archiveSha256"]:
        raise ValueError("Package digest is not the approved archive digest")
    metadata_path = PUBLIC / "release-metadata.json"
    if metadata_path.exists():
        metadata = decode(metadata_path.read_bytes())
        for name, info in metadata["files"].items():
            if file_sha(PUBLIC / name) != info["sha256"] or (PUBLIC / name).stat().st_size != info["byteLength"]:
                raise ValueError("Supplemental release metadata mismatch")
        if metadata["staticDeployedGeneratorIdentity"] != static_identity(deployment):
            raise ValueError("Supplemental static identity mismatch")
    return handoff, {"handoffSha256": sha(data), "staticDeployedGeneratorIdentity": static_identity(deployment), "immutableFieldMutationChecksPassed": changed_field_count, "serviceConfigurationSchemaAccepted": True, "relocationPreservesIdentity": True, "strictJsonAndClosedFieldsPassed": True, "settingsSchemaIdentityVerified": True}


def measure_installation(handoff, release_dir, installation):
    measured = {}
    for kind in ["package", "correspondingSource"]:
        artifact = handoff["release"][kind]
        path = release_dir / artifact["name"]
        if file_sha(path) != artifact["sha256"] or path.stat().st_size != artifact["byteLength"]:
            raise ValueError("Published artifact digest or length mismatch")
        measured[kind] = {"sha256": file_sha(path), "byteLength": path.stat().st_size}
    package = handoff["release"]["package"]
    descriptor_path = installation / package["descriptorMember"]
    descriptor = decode(descriptor_path.read_bytes())
    deployment = handoff["immutableDeployment"]
    if set(descriptor) != {"binaryIdentity", "buildIdentity", "provenanceSetIdentity"}:
        raise ValueError("Unexpected adjacent descriptor fields")
    if file_sha(descriptor_path) != package["descriptorSha256"]:
        raise ValueError("Adjacent descriptor digest mismatch")
    if identity("bambu-generator-package-v1", descriptor) != deployment["packageIdentity"]:
        raise ValueError("Adjacent descriptor identity mismatch")
    if any(descriptor[k] != deployment[k] for k in descriptor):
        raise ValueError("Adjacent descriptor bindings mismatch")
    binary = installation / package["executableMember"]
    mode = binary.stat().st_mode
    if not stat.S_ISREG(mode) or not mode & 0o111 or binary.is_symlink():
        raise ValueError("Published executable not regular executable bytes")
    if file_sha(binary) != deployment["binarySha256"]:
        raise ValueError("Published executable digest mismatch")
    inventory_bytes = (installation / "files.sha256.json").read_bytes()
    if inventory_bytes != (RELEASE / "package-files.json").read_bytes():
        raise ValueError("Installed inventory differs from approved release inventory")
    inventory = decode(inventory_bytes)
    actual = {p.relative_to(installation).as_posix() for p in installation.rglob("*") if p.is_file()}
    if actual != set(inventory) | {"files.sha256.json"}:
        raise ValueError("Package file set mismatch")
    for name, digest in inventory.items():
        if file_sha(installation / name) != digest:
            raise ValueError("Package file digest mismatch")
    measured.update({"binarySha256": file_sha(binary), "descriptorSha256": file_sha(descriptor_path), "descriptorIdentity": identity("bambu-generator-package-v1", descriptor), "packageFileCount": len(actual), "regularExecutableConfirmed": True})
    return binary.resolve(), measured


def exercise(handoff, binary, fixtures, work):
    work.mkdir(parents=True, exist_ok=False)
    successes = []
    deployment = handoff["immutableDeployment"]
    expected_candidates = decode((RELEASE / "cli-results.json").read_bytes())["success"]
    for name in expected_candidates:
        root = work / name
        root.mkdir()
        shutil.copytree(fixtures / name / "inputs", root / "inputs")
        request = decode((fixtures / name / "request.json").read_bytes())
        bind_request(request, deployment)
        PROTOCOL.validate(request)
        manifest = decode((root / request["inputManifest"]["path"]).read_bytes())
        PROTOCOL.validate(manifest)
        SETTINGS.validate(decode((root / request["settings"]["content"]["path"]).read_bytes()))
        if manifest["export"]["kindIdentity"] != deployment["inputKindIdentity"] or manifest["export"]["schemaIdentity"] != deployment["inputSchemaIdentity"]:
            raise ValueError("Fixture declarations do not match handoff")
        (root / "request.json").write_bytes(rfc8785.dumps(request))
        outcome = subprocess.run([str(binary), *handoff["interface"]["argv"]], cwd=root, capture_output=True)
        result_bytes = (root / "result.json").read_bytes()
        if len(result_bytes) > handoff["interface"]["result"]["maxByteLength"]:
            raise ValueError("Result exceeds declared bound")
        result = decode(result_bytes)
        PROTOCOL.validate(result)
        if outcome.returncode != 0 or result["status"] != "success":
            raise ValueError("Published binary did not accept complete handoff bindings")
        if result["protocolVersion"] != deployment["protocolVersion"] or result["invocationIdentity"] != request["invocationIdentity"] or result["reportedIdentities"] != request["expectedIdentities"]:
            raise ValueError("Result identity bindings mismatch")
        output = result["output"]
        if any(output[k] != request["output"][k] for k in ["outputIdentity", "role", "path", "mediaType"]):
            raise ValueError("Result output declaration mismatch")
        candidate = root / output["path"]
        if file_sha(candidate) != output["sha256"] or candidate.stat().st_size != output["byteLength"] or output["byteLength"] > min(request["output"]["maxByteLength"], handoff["interface"]["output"]["maxByteLength"]):
            raise ValueError("Measured candidate differs from declared output")
        if file_sha(candidate) != expected_candidates[name]["candidateSha256"] or list(root.rglob("generator-*.tmp")):
            raise ValueError("Candidate differs from original release or temporary output remains")
        successes.append({"case": name, "exitCode": outcome.returncode, "protocolVersion": result["protocolVersion"], "invocationIdentity": result["invocationIdentity"], "settingsIdentity": request["settings"]["settingsIdentity"], "expectedIdentities": request["expectedIdentities"], "reportedIdentities": result["reportedIdentities"], "output": output, "requestSha256": file_sha(root / "request.json"), "resultSha256": sha(result_bytes), "candidateMeasuredAndMatching": True, "temporaryOutputsAbsent": True})
    failures = []
    for case, category, code in [("malformed-request", "malformedRequest", "malformed_request"), ("identity-mismatch", "unsupportedRequest", "unsupported_generator_identity"), ("output-limit", "generationFailed", "output_byte_limit")]:
        root = work / case
        root.mkdir()
        shutil.copytree(fixtures / "configured" / "inputs", root / "inputs")
        request = decode((fixtures / "configured" / "request.json").read_bytes())
        bind_request(request, deployment)
        if case == "identity-mismatch":
            request["expectedIdentities"]["dialectIdentity"] = "synthetic-unmatched-identity"
        elif case == "output-limit":
            request["output"]["maxByteLength"] = 1
        request["invocationIdentity"] = identity("generator-invocation-v1", {k: v for k, v in request.items() if k != "invocationIdentity"})
        (root / "request.json").write_bytes(b"{" if case == "malformed-request" else rfc8785.dumps(request))
        outcome = subprocess.run([str(binary), *handoff["interface"]["argv"]], cwd=root, capture_output=True)
        data = (root / "result.json").read_bytes()
        result = decode(data)
        PROTOCOL.validate(result)
        if outcome.returncode != 1 or result["status"] != "failure" or len(data) > handoff["interface"]["result"]["maxByteLength"] or "output" in result or (root / "outputs/project.3mf").exists():
            raise ValueError("Ordinary failure behavior differs from handoff")
        if len(result["errors"]) != 1 or (result["errors"][0]["category"], result["errors"][0]["code"]) != (category, code):
            raise ValueError("Failure category/code differs from expected bounded outcome")
        if case != "malformed-request":
            if result.get("invocationIdentity") != request["invocationIdentity"]:
                raise ValueError("Failure invocation identity differs from parsed request")
            if "reportedIdentities" in result and result["reportedIdentities"] != request["expectedIdentities"]:
                raise ValueError("Failure reported identities differ from request")
        if list(root.rglob("generator-*.tmp")):
            raise ValueError("Failure left temporary output")
        failures.append({"case": case, "exitCode": outcome.returncode, "result": result, "resultByteLength": len(data), "candidateAbsent": True, "temporaryOutputsAbsent": True})
    return successes, failures


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--release-dir", type=Path)
    parser.add_argument("--installation", type=Path)
    parser.add_argument("--fixtures", type=Path)
    parser.add_argument("--work", type=Path)
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    handoff, report = check_handoff()
    if any(v is not None for v in [args.release_dir, args.installation, args.fixtures, args.work, args.report]):
        if any(v is None for v in [args.release_dir, args.installation, args.fixtures, args.work, args.report]):
            parser.error("All five artifact-experiment arguments are required together")
        binary, measured = measure_installation(handoff, args.release_dir, args.installation)
        successes, failures = exercise(handoff, binary, args.fixtures, args.work)
        _, after = measure_installation(handoff, args.release_dir, args.installation)
        if after != measured:
            raise ValueError("Released installation changed during experiment")
        report.update({"date": datetime.now(timezone.utc).date().isoformat(), "pythonVersion": sys.version, "verificationRecipeSha256": file_sha(Path(__file__)), "tools": {"jsonschema": version("jsonschema"), "rfc8785": version("rfc8785")}, "measuredArtifacts": measured, "installationUnchanged": True, "success": successes, "failures": failures})
        args.report.write_bytes(rfc8785.dumps(report))
        print(f"{len(successes)} success and {len(failures)} failure cases passed using the unchanged published executable")
    print(json.dumps({k: v for k, v in report.items() if k not in {"success", "failures"}}, indent=2))


if __name__ == "__main__":
    main()
