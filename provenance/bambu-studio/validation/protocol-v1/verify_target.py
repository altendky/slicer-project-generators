#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
"""AGPL-3.0-only; authored by Codex for Kyle Altendorf, 2026-10-08.

Read-only verification of synthetic generator candidates and Bambu saves.
"""
from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
import sys
import xml.etree.ElementTree as ET
import zipfile

CORE = "http://schemas.microsoft.com/3dmanufacturing/core/2015/02"
PRODUCTION = "http://schemas.microsoft.com/3dmanufacturing/production/2015/06"
I = (1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.)
MM = {"micron": .001, "millimeter": 1., "centimeter": 10., "inch": 25.4, "foot": 304.8, "meter": 1000.}
TOLERANCE_MM = 1e-9
BASE = Path(sys.argv[1]).resolve()

def sha(data):
    return hashlib.sha256(data).hexdigest()

def tag(name):
    return f"{{{CORE}}}{name}"

def transform(value):
    if value is None:
        return I
    values = tuple(map(float, value.split()))
    assert len(values) == 12 and all(map(math.isfinite, values))
    result = list(I)
    for col in range(4):
        for row in range(3):
            result[row * 4 + col] = values[col * 3 + row]
    return tuple(result)

def multiply(a, b):
    return tuple(sum(a[r * 4 + k] * b[k * 4 + c] for k in range(4)) for r in range(4) for c in range(4))

def apply(matrix, point):
    return tuple(sum(matrix[r * 4 + c] * point[c] for c in range(3)) + matrix[r * 4 + 3] for r in range(3))

def determinant(m):
    return m[0] * (m[5] * m[10] - m[6] * m[9]) - m[1] * (m[4] * m[10] - m[6] * m[8]) + m[2] * (m[4] * m[9] - m[5] * m[8])

class Archive:
    def __init__(self, path):
        self.path = path
        self.bytes = path.read_bytes()
        with zipfile.ZipFile(path) as z:
            assert len(z.namelist()) == len(set(z.namelist()))
            self.files = {name: z.read(name) for name in z.namelist() if not name.endswith("/")}
        rels = ET.fromstring(self.files["_rels/.rels"])
        primary = [n.attrib["Target"].lstrip("/") for n in rels if n.attrib.get("Type") == "http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"]
        assert len(primary) == 1
        self.primary = primary[0]
        self.models = {name: ET.fromstring(data) for name, data in self.files.items() if name.endswith(".model")}
        self.resources = {}
        for name, root in self.models.items():
            assert root.tag == tag("model")
            for obj in root.find(tag("resources")):
                key = (name, obj.attrib["id"])
                assert key not in self.resources
                self.resources[key] = obj
        self.settings = ET.fromstring(self.files["Metadata/model_settings.config"]) if "Metadata/model_settings.config" in self.files else None
        self.settings_objects = {} if self.settings is None else {n.attrib["id"]: n for n in self.settings.findall("object")}

    def expand(self, model, object_id, matrix=I, stack=()):
        key = model, object_id
        assert key not in stack and len(stack) < 32
        obj = self.resources[key]
        mesh = obj.find(tag("mesh"))
        if mesh is not None:
            unit = MM[self.models[model].attrib.get("unit", "millimeter")]
            vertices = [apply(matrix, tuple(float(v.attrib[a]) for a in ("x", "y", "z"))) for v in mesh.find(tag("vertices"))]
            vertices = [tuple(x * unit for x in point) for point in vertices]
            triangles = [tuple(vertices[int(t.attrib[a])] for a in ("v1", "v2", "v3")) for t in mesh.find(tag("triangles"))]
            # Preserve world-space orientation of stored triangles. In these
            # fixtures all archive transforms have nonnegative determinant.
            return [{"key": key, "type": obj.attrib.get("type", "model"), "name": obj.attrib.get("name"), "triangles": triangles}]
        result = []
        for component in obj.find(tag("components")):
            target = component.attrib.get(f"{{{PRODUCTION}}}path", model).lstrip("/")
            result += self.expand(target, component.attrib["objectid"], multiply(matrix, transform(component.attrib.get("transform"))), stack + (key,))
        return result

    def builds(self):
        result = []
        root = self.models[self.primary]
        for item in root.find(tag("build")):
            assert item.attrib.get("printable", "1") in ("1", "true")
            object_id = item.attrib["objectid"]
            model = item.attrib.get(f"{{{PRODUCTION}}}path", self.primary).lstrip("/")
            leaves = self.expand(model, object_id, transform(item.attrib.get("transform")))
            settings = self.settings_objects.get(object_id)
            if settings is None:
                name = self.resources[model, object_id].attrib.get("name")
                for leaf in leaves:
                    leaf["role"] = "normal_part"
            else:
                name = self.name(settings)
                parts = settings.findall("part")
                assert len(parts) == len(leaves)
                for leaf, part in zip(leaves, parts, strict=True):
                    assert leaf["key"][1] == part.attrib["id"]
                    leaf["name"] = self.name(part)
                    leaf["role"] = part.attrib["subtype"]
            result.append({"name": name, "leaves": leaves})
        return result

    @staticmethod
    def name(node):
        names = [n.attrib["value"] for n in node.findall("metadata") if n.attrib.get("key") == "name"]
        assert len(names) <= 1
        return names[0] if names else None

def compare_triangles(expected, actual):
    assert len(expected) == len(actual), "triangle count changed"
    # Order is verified independently. Cyclic rotations preserve orientation;
    # swapping two corners does not, and is deliberately rejected.
    maximum = 0.
    for a, b in zip(expected, actual, strict=True):
        delta = min(max(abs(a[i][axis] - b[(i + shift) % 3][axis]) for i in range(3) for axis in range(3)) for shift in range(3))
        assert delta <= TOLERANCE_MM, f"world-space oriented triangle changed: {a} vs {b}; delta={delta}"
        maximum = max(maximum, delta)
    return maximum

def verify_fixture(name):
    root = BASE / name
    manifest = json.loads((root / "inputs/manifest.json").read_bytes())
    settings = json.loads((root / "inputs/settings.json").read_bytes())
    placements = settings["placements"]
    assert len(placements) == len(manifest["objects"])
    realized = {}
    inputs = []
    for obj, placement in zip(manifest["objects"], placements, strict=True):
        identity = obj["objectIdentity"]
        assert identity == placement["objectIdentity"]
        path = root / obj["retainedContent"]["path"]
        archive = Archive(path)
        assert sha(archive.bytes) == obj["retainedContent"]["sha256"]
        assert len(archive.bytes) == obj["retainedContent"]["byteLength"]
        triangles = [triangle for build in archive.builds() for leaf in build["leaves"] for triangle in leaf["triangles"]]
        matrix = placement["matrix"]
        triangles = [tuple(tuple(x * 1000 for x in apply(matrix, tuple(v / 1000 for v in point))) for point in triangle) for triangle in triangles]
        if determinant(matrix) < 0:
            triangles = [(t[0], t[2], t[1]) for t in triangles]
        realized[identity] = triangles
        inputs.append({"identity": identity, "path": obj["retainedContent"]["path"], "sha256": sha(archive.bytes), "byteLength": len(archive.bytes)})
    blockers = {b["objectIdentity"]: b["targets"] for b in settings["blockers"]}
    candidate = Archive(root / "outputs/project.3mf")
    native = Archive(root / "preset-roundtrip.3mf")
    generated_builds = candidate.builds()
    saved_builds = native.builds()
    printable = [o for o in manifest["objects"] if o["role"] == "rawGeometry"]
    assert len(generated_builds) == len(saved_builds) == len(printable)
    max_input_delta = max_save_delta = 0.
    associations = []
    for model, generated, saved in zip(printable, generated_builds, saved_builds, strict=True):
        assert generated["name"] == saved["name"] == model.get("displayName"), "printable name/order changed"
        expected = [o for o in manifest["objects"] if o["objectIdentity"] == model["objectIdentity"] or model["objectIdentity"] in blockers.get(o["objectIdentity"], [])]
        assert len(generated["leaves"]) == len(saved["leaves"]) == len(expected)
        members = []
        for obj, generated_leaf, saved_leaf in zip(expected, generated["leaves"], saved["leaves"], strict=True):
            role = "normal_part" if obj["role"] == "rawGeometry" else "support_blocker"
            assert generated_leaf["role"] == saved_leaf["role"] == role, "blocker role/association/order changed"
            assert generated_leaf["name"] == saved_leaf["name"] == obj.get("displayName"), "volume name changed"
            assert generated_leaf["type"] == saved_leaf["type"] == ("model" if role == "normal_part" else "other")
            max_input_delta = max(max_input_delta, compare_triangles(realized[obj["objectIdentity"]], generated_leaf["triangles"]))
            max_save_delta = max(max_save_delta, compare_triangles(generated_leaf["triangles"], saved_leaf["triangles"]))
            members.append({"identity": obj["objectIdentity"], "role": role, "triangles": len(saved_leaf["triangles"])})
        associations.append({"printableIdentity": model["objectIdentity"], "members": members})
    return {
        "fixture": name, "passed": True, "inputs": inputs,
        "candidateSha256": sha(candidate.bytes), "nativeSavedSha256": sha(native.bytes),
        "candidateModelSettingsSha256": sha(candidate.files["Metadata/model_settings.config"]) if "Metadata/model_settings.config" in candidate.files else None,
        "nativeModelSettingsSha256": sha(native.files["Metadata/model_settings.config"]),
        "nativeProjectSettingsSha256": sha(native.files["Metadata/project_settings.config"]),
        "maxInputToCandidateDeltaMm": max_input_delta, "maxCandidateToNativeDeltaMm": max_save_delta,
        "associations": associations,
    }

def main():
    results = []
    failures = []
    for name in ("configured", "assembly", "configured-blockers", "assembly-blockers", "assembly-internal-blockers"):
        try:
            results.append(verify_fixture(name))
        except Exception as error:
            failures.append({"fixture": name, "error": repr(error)})
    report = {
        "scriptSha256": sha(Path(__file__).read_bytes()), "pythonVersion": sys.version,
        "comparisonToleranceMm": TOLERANCE_MM, "results": results, "failures": failures,
        "earlierNoPresetReload": "Separate known failure; this script verifies preset-roundtrip.3mf only",
    }
    print(json.dumps(report, indent=2))
    return bool(failures)

if __name__ == "__main__":
    raise SystemExit(main())
