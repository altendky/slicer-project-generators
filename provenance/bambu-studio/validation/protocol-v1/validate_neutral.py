#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.12"
# dependencies = ["jsonschema==4.25.1"]
# ///
"""Validate positive fixture documents using the unchanged service schemas."""
import json
from pathlib import Path
import sys

from jsonschema import Draft202012Validator

neutral = Path(__file__).resolve().parents[4] / "crates/bambu-studio/neutral"
protocol = Draft202012Validator(json.loads((neutral / "generator-protocol-v1.schema.json").read_text()))
settings = Draft202012Validator(json.loads((neutral / "generator-settings-v2.schema.json").read_text()))
for schema in (protocol.schema, settings.schema):
    Draft202012Validator.check_schema(schema)
count = 0
for root in sorted(Path(sys.argv[1]).iterdir()):
    for path in ("request.json", "inputs/manifest.json", "result.json"):
        protocol.validate(json.loads((root / path).read_text()))
    settings.validate(json.loads((root / "inputs/settings.json").read_text()))
    count += 1
print(f"{count} requests, manifests, settings, and results pass official Draft 2020-12 schemas.")
