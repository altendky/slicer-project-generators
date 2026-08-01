# Support-Blocking Volumes Target Validation

This directory records the disposable Ubuntu 24.04 runtime used for
`BBL-SUPPORT-BLOCKING-VOLUMES-VERIFICATION-v1`. It does not contain or
redistribute Bambu Studio. Obtain the exact official AppImage named in the terms
review, verify its SHA-256, and extract it with `--appimage-extract` into
temporary storage.

Generate the source-authored fixture from a clean checkout of implementation
commit `2113110348d6f45f495b52481e8a5d0b9139dd7d` with Rust 1.94.1:

```console
test "$(git rev-parse HEAD)" = 2113110348d6f45f495b52481e8a5d0b9139dd7d
cargo +1.94.1 run --locked --quiet --example support_blocking_validation_fixture \
  -- support-blocking-input.3mf
sha256sum support-blocking-input.3mf
```

The expected fixture SHA-256 is
`6910a5bcc6e1301637829079d03807edef5e6a7239b6e67a8388564ae97f2780`.
Verify and extract the exact AppImage before starting the isolated target:

```console
sha256sum BambuStudio_ubuntu24.04-v02.07.01.62-20260616195227.AppImage
./BambuStudio_ubuntu24.04-v02.07.01.62-20260616195227.AppImage \
  --appimage-extract
```

The expected AppImage SHA-256 is
`fa98b608532dfbbbb2b0931483aac41e57fb19c175a2cc7bd7d528d5e0fbb287`;
use the resulting `squashfs-root` as `APPDIR`.

Build the local tag from this directory. The Dockerfile pins the Ubuntu base by
digest and resolves the CA bundle and runtime packages through Ubuntu snapshot
`20260731T230600Z`. The minimal base lacks a CA bundle, so TLS peer verification
is disabled only for snapshot transport during that signed apt transaction;
Ubuntu archive signature verification remains enabled:

```console
docker build --no-cache --platform linux/amd64 \
  --tag bambu-support-validation:2.7.1.62 .
```

Run the extracted target offline with read-only inputs:

```console
docker run --rm --platform linux/amd64 --network none --cap-drop ALL \
  --security-opt no-new-privileges \
  --user "$(id -u):$(id -g)" \
  --env HOME=/tmp --env LIBGL_ALWAYS_SOFTWARE=1 \
  --mount type=bind,src="$APPDIR",dst=/app,readonly \
  --mount type=bind,src="$INPUT",dst=/input.3mf,readonly \
  --mount type=bind,src="$OUTPUT_DIR",dst=/output \
  bambu-support-validation:2.7.1.62 \
  /app/AppRun --debug 5 \
  --export-3mf /output/roundtrip-1.3mf /input.3mf
```

Extract and hash `Metadata/model_settings.config` from the target-produced
archive, then inspect it for model A with one normal part and blockers A1 and A2
and model B with one normal part and blocker B:

```console
sha256sum "$INPUT" "$ROUNDTRIP"
unzip -p "$ROUNDTRIP" Metadata/model_settings.config \
  > "$OUTPUT_DIR/model_settings.config"
sha256sum "$OUTPUT_DIR/model_settings.config"
```

The expected target output SHA-256 is
`f9ed8a21ea4129b3ef9a9a0d7d21924661d3e98d8ebaf48d0a1da4cdcd2cee14`;
the expected settings-member SHA-256 is
`4d464f1eb19483b0f880d0c089226c9f7a5d8c543b4edac499f9fd8ea0ec358c`.
Reload the target-produced archive offline:

```console
docker run --rm --platform linux/amd64 --network none --cap-drop ALL \
  --security-opt no-new-privileges \
  --user "$(id -u):$(id -g)" \
  --env HOME=/tmp --env LIBGL_ALWAYS_SOFTWARE=1 \
  --mount type=bind,src="$APPDIR",dst=/app,readonly \
  --mount type=bind,src="$ROUNDTRIP",dst=/input.3mf,readonly \
  bambu-support-validation:2.7.1.62 \
  /app/AppRun --debug 5 --info /input.3mf
```

Reproduce the package-manifest hash with the container's default C locale:

```console
docker run --rm --platform linux/amd64 --network none \
  bambu-support-validation:2.7.1.62 \
  dpkg-query -W -f='${Package}\t${Version}\n' | sha256sum
```

The verification record contains exact artifact, image, fixture, output,
member, and package-manifest hashes and observed associations. Do not install
the networking plugin or commit, publish, or redistribute the AppImage or
target-produced archive.
