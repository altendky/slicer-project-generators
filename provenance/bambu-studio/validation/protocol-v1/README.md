# Protocol v1 Exact-Target Validation

Use the implementation commits and exact artifact identities in the
[verification supplement](../../verification/protocol-v1.md). These commands
run synthetic neutral fixtures only. The target and target-produced archives
remain temporary and are not redistributed.

Obtain the official Bambu Studio 2.7.1.62 Ubuntu 24.04 AppImage reviewed in the
[upstream terms record](../../reviews/bambu-studio-v02.07.01.62.md), verify SHA-256
`fa98b608532dfbbbb2b0931483aac41e57fb19c175a2cc7bd7d528d5e0fbb287`, and
extract it using `--appimage-extract` in a session temporary directory.
Build the existing [snapshot-pinned validation Dockerfile](../support-blocking-volumes-v1/Dockerfile).
The realized image used in the record is `bambu-validation-rebuild:2.7.1.62`.

Generate fresh fixtures from the reviewed source:

```console
cargo +1.94.1 build --locked --bin slicer-project-generator-bambu-studio
cargo +1.94.1 run --locked --example protocol_validation_fixture -- \
  "$BBL_FIXTURES" "$BBL_BINARY"
for name in configured assembly configured-blockers assembly-blockers \
  assembly-internal-blockers; do
  (cd "$BBL_FIXTURES/$name" && \
    ./slicer-project-generator-bambu-studio \
      --request request.json --result result.json) || exit
done
uv run provenance/bambu-studio/validation/protocol-v1/validate_neutral.py \
  "$BBL_FIXTURES"
```

`BBL_FIXTURES` is a fresh session temporary directory and `BBL_BINARY` is the
absolute path to the reviewed generator binary. The example copies the binary
and creates explicitly development-only hash-bound descriptors. No release
approval is implied. Its five cases exercise configured and rigid assembly
placements, duplicate names, equal bytes at separate paths, multiple blockers,
a shared blocker, and units/nested internal transforms.

For each case, run target save and reload offline. `BBL_APPDIR` is the extracted
hash-verified AppImage tree. Disable arrangement and supply the exact bundled
presets; they belong to the compatibility environment and are not generator
settings or distributed package inputs:

```console
for name in configured assembly configured-blockers assembly-blockers \
  assembly-internal-blockers; do
  docker run --rm --platform linux/amd64 --network none --cap-drop ALL \
    --security-opt no-new-privileges --user "$(id -u):$(id -g)" \
    --env HOME=/tmp --env LIBGL_ALWAYS_SOFTWARE=1 \
    --mount type=bind,src="$BBL_APPDIR",dst=/app,readonly \
    --mount type=bind,src="$BBL_FIXTURES/$name/outputs/project.3mf",dst=/input.3mf,readonly \
    --mount type=bind,src="$BBL_FIXTURES/$name",dst=/output \
    bambu-validation-rebuild:2.7.1.62 /app/AppRun --debug 5 --arrange 0 \
    --load-settings '/app/resources/profiles/BBL/machine/Bambu Lab X1 Carbon 0.4 nozzle.json;/app/resources/profiles/BBL/process/0.20mm Standard @BBL X1C.json' \
    --load-filaments '/app/resources/profiles/BBL/filament/Bambu PLA Basic @BBL X1C.json' \
    --export-3mf /output/preset-roundtrip.3mf /input.3mf || exit
  docker run --rm --platform linux/amd64 --network none --cap-drop ALL \
    --security-opt no-new-privileges --user "$(id -u):$(id -g)" \
    --env HOME=/tmp --env LIBGL_ALWAYS_SOFTWARE=1 \
    --mount type=bind,src="$BBL_APPDIR",dst=/app,readonly \
    --mount type=bind,src="$BBL_FIXTURES/$name/preset-roundtrip.3mf",dst=/input.3mf,readonly \
    bambu-validation-rebuild:2.7.1.62 /app/AppRun --debug 5 --info /input.3mf || exit
done
uv run provenance/bambu-studio/validation/protocol-v1/verify_target.py \
  "$BBL_FIXTURES"
```

The separate Python verifier checks raw-unit/internal-transform/neutral-placement
realization against generated geometry, then follows the target's split model
paths and component/build transformations. It compares oriented triangles,
names, model/volume order and every blocker association. Its tolerance is
`1e-9` millimeters; this is a fixture-specific observation, not a universal
target precision guarantee. Native target ZIP timestamps and metadata are not
claimed byte-reproducible. Generator candidates are deterministic.

The recorded first no-preset native reload crashed after completed import because
the target saved default config without required preset identifiers. The
successful preset-based commands above retain the original geometry and roles
and avoid that diagnosed target CLI condition. Do not edit saved archive bytes
or describe the earlier run as successful.
