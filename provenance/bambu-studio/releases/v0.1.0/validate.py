#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
"""Bind synthetic protocol fixtures to an installed release; test its CLI.

Run on fresh fixtures from protocol_validation_fixture. Existing native saves
may accompany them when the generated candidate bytes remain identical.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile

sys.dont_write_bytecode = True
from build import BINARY, json_bytes, sha

GOLDEN = {
    "configured": "4eb23becc267a87ba0459307ecd58d53b0ecd35d2edff92ed130e044def1aefa",
    "assembly": "878d7d45d4f212d2054b09f11dfaba483c67e571b2b5f6ca7bd93c0423bca642",
    "configured-blockers": "84733edad4bad78236e2b23a78ca542b765891095f7d8977b77b1bf5c383c44c",
    "assembly-blockers": "9e483c26b6c6ac011b161ad7ee417d2ffb8c2e7ef567d8c23cd1ce6f67881d00",
    "assembly-internal-blockers": "755c23d43e93092125af6d661da8b6821805b13f15e9eed751bc5e3a92933918",
}
ERRORS = {
    "malformed-request": ("malformedRequest", "malformed_request"),
    "oversized-request": ("malformedRequest", "malformed_request"),
    "package-mismatch": ("unsupportedRequest", "unsupported_generator_identity"),
    "unsupported-materials": ("unsupportedRequest", "unsupported_geometry_profile"),
    "hash-mismatch": ("invalidInput", "raw_input_file_invalid"),
    "missing-input": ("invalidInput", "raw_input_file_invalid"),
    "input-symlink": ("invalidInput", "raw_input_file_invalid"),
    "output-limit": ("generationFailed", "output_byte_limit"),
    "candidate-conflict": ("generationFailed", "candidate_write_failed"),
    "wrong-sidecar": ("internalError", "package_identity_invalid"),
}


def identity(domain, value):
    # These fixture hash payloads have ASCII keys, strings and safe integers,
    # never floats. Python's compact sorted encoding equals JCS for this subset.
    def check(item):
        if isinstance(item, dict):
            for key, child in item.items():
                assert key.isascii()
                check(child)
        elif isinstance(item, list):
            for child in item:
                check(child)
        elif isinstance(item, int):
            assert abs(item) <= 2**53 - 1
        else:
            assert isinstance(item, (str, bool)) or item is None
    check(value)
    return hashlib.sha256(json_bytes({"domain": domain, "payload": value})).hexdigest()


def bind(request, package):
    request["expectedIdentities"].update(package)
    request["expectedIdentities"]["packageIdentity"] = identity("bambu-generator-package-v1", package)
    request["invocationIdentity"] = identity("generator-invocation-v1", {k: v for k, v in request.items() if k != "invocationIdentity"})


def write_request(root, request, package):
    bind(request, package)
    (root / "request.json").write_bytes(json_bytes(request))


def rebind_manifest(root, request):
    path = root / request["inputManifest"]["path"]
    manifest = json.loads(path.read_text())
    content = manifest["objects"][0]["retainedContent"]
    content["sha256"] = sha(root / content["path"])
    content["byteLength"] = (root / content["path"]).stat().st_size
    manifest["inputSetIdentity"] = identity("generator-input-set-v1", {k: manifest[k] for k in ["protocolVersion", "manifestVersion", "objects"]})
    manifest["manifestIdentity"] = identity("generator-input-manifest-v1", {k: v for k, v in manifest.items() if k != "manifestIdentity"})
    for key in ["inputSetIdentity", "manifestIdentity"]:
        request["inputManifest"][key] = manifest[key]
    path.write_bytes(json_bytes(manifest))


def invoke(binary, root):
    outcome = subprocess.run([str(binary), "--request", "request.json", "--result", "result.json"], cwd=root, capture_output=True)
    result = json.loads((root / "result.json").read_text()) if (root / "result.json").exists() else None
    return outcome, result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("package", type=Path)
    parser.add_argument("fixtures", type=Path)
    parser.add_argument("report", type=Path)
    args = parser.parse_args()
    binary = (args.package / BINARY).resolve()
    package = json.loads((args.package / "package.json").read_text())
    assert sha(binary) == package["binaryIdentity"]
    report = {"package": package, "success": {}, "failures": {}}
    for name, golden in GOLDEN.items():
        root = args.fixtures / name
        assert not (root / "outputs/project.3mf").exists()
        request = json.loads((root / "request.json").read_text())
        write_request(root, request, package)
        outcome, result = invoke(binary, root)
        assert outcome.returncode == 0 and result["status"] == "success", (name, outcome.stderr, result)
        assert result["invocationIdentity"] == request["invocationIdentity"]
        assert result["reportedIdentities"] == request["expectedIdentities"]
        for key in ["outputIdentity", "role", "path", "mediaType"]:
            assert result["output"][key] == request["output"][key]
        candidate = root / result["output"]["path"]
        assert sha(candidate) == golden == result["output"]["sha256"]
        assert candidate.stat().st_size == result["output"]["byteLength"]
        assert not list(root.rglob("generator-*.tmp"))
        report["success"][name] = {"candidateSha256": golden, "candidateByteLength": candidate.stat().st_size, "invocationIdentity": request["invocationIdentity"], "settingsIdentity": request["settings"]["settingsIdentity"], "resultSha256": sha(root / "result.json")}
    trials = ["malformed-request", "oversized-request", "package-mismatch", "unsupported-materials", "hash-mismatch", "missing-input", "input-symlink", "output-limit", "candidate-conflict", "result-conflict", "wrong-sidecar"]
    failures = args.fixtures.with_name(args.fixtures.name + "-negative")
    failures.mkdir()
    for name in trials:
        root = failures / name
        root.mkdir()
        shutil.copytree(args.fixtures / "configured/inputs", root / "inputs")
        request = json.loads((args.fixtures / "configured/request.json").read_text())
        write_request(root, request, package)
        trial_binary = binary
        marker = None
        if name == "malformed-request":
            (root / "request.json").write_bytes(b"{")
        elif name == "oversized-request":
            (root / "request.json").write_bytes(b" " * (1024 * 1024 + 1))
        elif name == "package-mismatch":
            request["expectedIdentities"]["packageIdentity"] = "wrong-package"
            request["invocationIdentity"] = identity("generator-invocation-v1", {k: v for k, v in request.items() if k != "invocationIdentity"})
            (root / "request.json").write_bytes(json_bytes(request))
        elif name == "unsupported-materials":
            raw = root / "inputs/geometry-0.3mf"
            with zipfile.ZipFile(raw) as archive:
                files = {n: archive.read(n) for n in archive.namelist()}
            files["3D/3dmodel.model"] = files["3D/3dmodel.model"].replace(b"<resources>", b'<resources><basematerials id="3"/>')
            with zipfile.ZipFile(raw, "w", compression=zipfile.ZIP_STORED) as archive:
                for path, content in files.items():
                    archive.writestr(zipfile.ZipInfo(path, date_time=(1980, 1, 1, 0, 0, 0)), content)
            rebind_manifest(root, request)
            write_request(root, request, package)
        elif name == "hash-mismatch":
            with (root / "inputs/geometry-0.3mf").open("ab") as file:
                file.write(b"changed")
        elif name == "missing-input":
            (root / "inputs/geometry-0.3mf").unlink()
        elif name == "input-symlink":
            (root / "inputs/geometry-0.3mf").unlink()
            (root / "inputs/geometry-0.3mf").symlink_to("geometry-1.3mf")
        elif name == "output-limit":
            request["output"]["maxByteLength"] = 1
            write_request(root, request, package)
        elif name in ["candidate-conflict", "result-conflict"]:
            destination = root / ("outputs/project.3mf" if name == "candidate-conflict" else "result.json")
            destination.parent.mkdir(parents=True, exist_ok=True)
            marker = destination.with_name("generator-" + hashlib.sha256(destination.name.encode()).hexdigest() + ".tmp")
            marker.write_bytes(b"existing-marker")
        elif name == "wrong-sidecar":
            installation = root / "installation"
            installation.mkdir()
            trial_binary = installation / BINARY
            shutil.copyfile(binary, trial_binary)
            trial_binary.chmod(0o755)
            (installation / "package.json").write_bytes(b"{}")
        outcome, result = invoke(trial_binary, root)
        assert outcome.returncode == 1, (name, outcome.returncode, result)
        assert not (root / "outputs/project.3mf").exists(), name
        if marker:
            assert marker.read_bytes() == b"existing-marker"
        if name == "result-conflict":
            assert result is None
        else:
            assert result["status"] == "failure" and len(result["errors"]) == 1, (name, result)
            error = result["errors"][0]
            assert (error["category"], error["code"]) == ERRORS[name], (name, error)
            assert "output" not in result
            assert (root / "result.json").stat().st_size <= 256 * 1024
        report["failures"][name] = {"exitCode": outcome.returncode, "result": result, "candidateAbsent": True, "existingMarkerPreserved": marker is not None}
    args.report.write_bytes(json_bytes(report) + b"\n")
    print(f"Validated {len(GOLDEN)} successful and {len(trials)} failed release CLI invocations")


if __name__ == "__main__":
    main()
