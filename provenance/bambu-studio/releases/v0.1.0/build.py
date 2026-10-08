#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
"""Package an offline, reviewed source tree after its release build.

Run inside an extracted corresponding-source tree, after the documented
container command builds /build/target/release. Only Python's standard library
is used. JSON descriptors contain strings only, so sorted compact JSON is JCS.
"""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path
import shutil
import sys
import tarfile
import zlib

BINARY = "slicer-project-generator-bambu-studio"
NAME = "bambu-studio-v0.1.0-x86_64-unknown-linux-gnu"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def json_bytes(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def archive(tree, destination, epoch, preserve_executable=False):
    """Fixed paths, permissions, ownership, order, mtime and gzip header."""
    with destination.open("wb") as output:
        with gzip.GzipFile(filename="", mode="wb", fileobj=output, mtime=0, compresslevel=9) as compressed:
            with tarfile.open(fileobj=compressed, mode="w", format=tarfile.GNU_FORMAT) as tar:
                for path in sorted(tree.rglob("*")):
                    if not path.is_file():
                        continue
                    if path.is_symlink():
                        raise ValueError(f"Unexpected symbolic link: {path}")
                    data = path.read_bytes()
                    info = tarfile.TarInfo(path.relative_to(tree).as_posix())
                    info.size = len(data)
                    executable = path.stat().st_mode & 0o111 if preserve_executable else path.name == BINARY
                    info.mode = 0o755 if executable else 0o644
                    info.mtime = epoch
                    tar.addfile(info, io.BytesIO(data))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    parser.add_argument("binary", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    source = args.source.resolve()
    inputs = json.loads((source / "release/build-inputs.json").read_text())
    assert sys.version == inputs["environment"]["packagingPython"]
    assert zlib.ZLIB_RUNTIME_VERSION == inputs["environment"]["packagingZlib"]
    inventory = json.loads((source / "release/source-files.sha256.json").read_text())
    actual = {p.relative_to(source).as_posix() for p in source.rglob("*") if p.is_file()}
    assert actual == set(inventory) | {"release/source-files.sha256.json"}
    for path, digest in inventory.items():
        assert sha(source / path) == digest, path
    modes = json.loads((source / "release/source-modes.json").read_text())
    for path, mode in modes.items():
        assert (0o755 if (source / path).stat().st_mode & 0o111 else 0o644) == mode, path
    args.output.mkdir(parents=True, exist_ok=False)
    package = args.output / NAME
    package.mkdir()
    shutil.copyfile(args.binary, package / BINARY)
    (package / BINARY).chmod(0o755)
    descriptor = {
        "binaryIdentity": sha(args.binary),
        "buildIdentity": hashlib.sha256(json_bytes(inputs)).hexdigest(),
        "provenanceSetIdentity": "BBL-MVP-PROVENANCE-SET-v1",
    }
    (package / "package.json").write_bytes(json_bytes(descriptor) + b"\n")
    for name in ["LICENSE", "NOTICE"]:
        shutil.copyfile(source / name, package / name)
    shutil.copyfile(source / "crates/bambu-studio/neutral/LICENSE-MIT", package / "LICENSE-neutral-MIT")
    shutil.copytree(source / "release/notices", package / "third-party-notices")
    for name in ["build-inputs.json", "dependencies.json", "README.md"]:
        shutil.copyfile(source / "release" / name, package / name)
    inventory = {p.relative_to(package).as_posix(): sha(p) for p in sorted(package.rglob("*")) if p.is_file()}
    (package / "files.sha256.json").write_bytes(json_bytes(inventory) + b"\n")
    artifact = args.output / (NAME + ".tar.gz")
    archive(package, artifact, inputs["sourceEpoch"])
    identities = {
        **descriptor,
        "packageIdentity": hashlib.sha256(json_bytes({"domain": "bambu-generator-package-v1", "payload": descriptor})).hexdigest(),
        "archiveSha256": sha(artifact),
        "archiveByteLength": artifact.stat().st_size,
        "descriptorSha256": sha(package / "package.json"),
    }
    (args.output / "identities.json").write_bytes(json_bytes(identities) + b"\n")
    print(json.dumps(identities, indent=2))


if __name__ == "__main__":
    main()
