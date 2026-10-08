#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
"""Freeze source, vendored dependencies and notices for the reviewed release.

Inputs are an existing full repository, its populated Cargo cache, the exact
Rust installation, the verified rust-src archive and a build environment record.
No target-produced fixture or AppImage is included.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tomllib

sys.dont_write_bytecode = True
from build import archive, json_bytes, sha

COMMIT = "54a0fa1635f75039d6b858a95bbe803bb61103cf"
RUST_SOURCE_HASH = "cb3756156fe6d2d6cedad327c94ad3721b612c4cd20dfb226d7543250788b66c"


def main():
    parser = argparse.ArgumentParser()
    for name in ["repo", "rust", "rust-source", "system-notices", "environment", "output"]:
        parser.add_argument("--" + name, required=True, type=Path)
    args = parser.parse_args()
    assert sha(args.rust_source) == RUST_SOURCE_HASH
    args.output.mkdir(parents=True, exist_ok=False)
    source = args.output / "source"
    source.mkdir()
    with subprocess.Popen(["git", "-C", str(args.repo), "archive", COMMIT], stdout=subprocess.PIPE) as proc:
        with tarfile.open(fileobj=proc.stdout, mode="r|") as tar:
            tar.extractall(source, filter="data")
        assert proc.wait() == 0
    subprocess.run(["cargo", "+1.94.1", "vendor", "--locked", "--offline", "--versioned-dirs", "vendor"], cwd=source, check=True, stdout=subprocess.DEVNULL)
    (source / ".cargo").mkdir()
    (source / ".cargo/config.toml").write_text('[source.crates-io]\nreplace-with = "vendored-sources"\n[source.vendored-sources]\ndirectory = "vendor"\n')
    release = source / "release"
    release.mkdir()
    for name in ["build.py", "prepare.py", "validate.py", "Dockerfile", "README.md"]:
        shutil.copyfile(Path(__file__).parent / name, release / name)
    shutil.copyfile(args.rust_source, release / "rust-src-1.94.1.tar.xz")
    notices = release / "notices"
    notices.mkdir()
    shutil.copytree(args.system_notices, notices / "system-runtime")
    with tarfile.open(args.rust_source) as tar:
        for member in tar.getmembers():
            path = Path(member.name)
            if member.isfile() and path.name.upper().startswith(("LICENSE", "COPYING", "COPYRIGHT", "NOTICE")):
                assert not path.is_absolute() and ".." not in path.parts
                destination = notices / "rust-library-source" / path
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(tar.extractfile(member).read())
    dependencies = []
    lock = tomllib.loads((source / "Cargo.lock").read_text())
    for package in lock["package"]:
        if "source" not in package:
            continue
        name = f'{package["name"]}-{package["version"]}'
        vendor = source / "vendor" / name
        checksum = json.loads((vendor / ".cargo-checksum.json").read_text())
        assert checksum["package"] == package["checksum"]
        for path, digest in checksum["files"].items():
            assert sha(vendor / path) == digest, (name, path)
        manifest = tomllib.loads((vendor / "Cargo.toml").read_text())["package"]
        retained = {}
        for path in sorted(vendor.rglob("*")):
            if path.is_file() and path.name.upper().startswith(("LICENSE", "COPYING", "COPYRIGHT", "NOTICE")):
                destination = notices / name / path.relative_to(vendor)
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(path, destination)
                retained[path.relative_to(vendor).as_posix()] = sha(path)
        assert retained, f"No notices found for {name}"
        dependencies.append({**package, "declaredLicense": manifest["license"], "notices": retained})
    rust_notices = notices / "rust-1.94.1"
    rust_notices.mkdir()
    for name in ["COPYRIGHT-library.html", "COPYRIGHT.html", "licenses"]:
        path = args.rust / "share/doc/rust" / name
        if path.is_dir():
            shutil.copytree(path, rust_notices / name)
        else:
            shutil.copyfile(path, rust_notices / name)
    toolchain_files = {}
    for base in ["bin", "lib"]:
        for path in sorted((args.rust / base).rglob("*")):
            if path.is_file():
                toolchain_files[path.relative_to(args.rust).as_posix()] = sha(path)
    (release / "dependencies.json").write_bytes(json_bytes(dependencies) + b"\n")
    inputs = {
        "sourceCommit": COMMIT,
        "sourceEpoch": int(subprocess.check_output(["git", "-C", str(args.repo), "show", "-s", "--format=%ct", COMMIT])),
        "lockSha256": sha(source / "Cargo.lock"),
        "dependenciesSha256": sha(release / "dependencies.json"),
        "recipeSha256": sha(release / "build.py"),
        "prepareRecipeSha256": sha(release / "prepare.py"),
        "validationRecipeSha256": sha(release / "validate.py"),
        "readmeSha256": sha(release / "README.md"),
        "dockerfileSha256": sha(release / "Dockerfile"),
        "rustSourceSha256": RUST_SOURCE_HASH,
        "rustVersion": "1.94.1",
        "target": "x86_64-unknown-linux-gnu",
        "command": "cargo build --release --locked --offline --bin slicer-project-generator-bambu-studio",
        "cargoIncremental": "0",
        "rustFlags": "--remap-path-prefix=/src=/source --remap-path-prefix=/build=/build",
        "environment": json.loads(args.environment.read_text()),
        "packagingInterpreterSha256": sha(Path(sys.executable)),
        "notices": {p.relative_to(notices).as_posix(): sha(p) for p in sorted(notices.rglob("*")) if p.is_file()},
        "toolchainFiles": toolchain_files,
    }
    (release / "build-inputs.json").write_bytes(json_bytes(inputs) + b"\n")
    modes = {p.relative_to(source).as_posix(): (0o755 if p.stat().st_mode & 0o111 else 0o644) for p in sorted(source.rglob("*")) if p.is_file()}
    (release / "source-modes.json").write_bytes(json_bytes(modes) + b"\n")
    inventory = {p.relative_to(source).as_posix(): sha(p) for p in sorted(source.rglob("*")) if p.is_file()}
    (release / "source-files.sha256.json").write_bytes(json_bytes(inventory) + b"\n")
    artifact = args.output / "bambu-studio-v0.1.0-corresponding-source.tar.gz"
    archive(source, artifact, inputs["sourceEpoch"], preserve_executable=True)
    print(json.dumps({"sourceSha256": sha(artifact), "sourceByteLength": artifact.stat().st_size, "sourceFiles": len(inventory)}, indent=2))


if __name__ == "__main__":
    main()
