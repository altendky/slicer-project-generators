# Named Objects Target Validation

This directory records the disposable Ubuntu 24.04 runtime used for
`BBL-NAMED-OBJECTS-VERIFICATION-v1`. It does not contain or redistribute Bambu
Studio. Obtain the exact official AppImage named in the terms review, verify its
SHA-256 before use, and extract it with `--appimage-extract` into temporary
storage.

Build the environment from this directory as `bambu-validation:2.7.1.62`, then
generate `named-objects-input.3mf` with the crate's `validation_fixture` example.
Run the extracted `AppRun` with the AppImage tree and input mounted read-only,
an output directory mounted writable, and these Docker controls:

```console
docker run --rm --network none --cap-drop ALL \
  --security-opt no-new-privileges \
  --user "$(id -u):$(id -g)" \
  --env HOME=/tmp --env LIBGL_ALWAYS_SOFTWARE=1 \
  --mount type=bind,src="$APPDIR",dst=/app,readonly \
  --mount type=bind,src="$INPUT",dst=/input.3mf,readonly \
  --mount type=bind,src="$OUTPUT_DIR",dst=/output \
  bambu-validation:2.7.1.62 \
  /app/AppRun --debug 5 \
  --export-3mf /output/roundtrip-1.3mf /input.3mf
```

Reload the target-produced archive in a second `--network none` invocation with
`--info`. The pinned target reaches `IMPORT_STAGE_FINISH` after parsing
`Metadata/model_settings.config` and assembling four objects. Inspect that XML
from the saved archive and compare all four name records to the source-authored
fixture. The verification record contains the observed artifact and member
hashes. Do not install the networking plugin or commit, publish, or redistribute
the AppImage or target-generated archives.
