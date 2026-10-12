# Generator-Local Handoff Verification v1

- Record ID: `BBL-DEPLOYMENT-HANDOFF-VERIFICATION-v1`
- Author: Codex operating for Kyle Altendorf; date: 2026-10-12
- Destination: neutral metadata supplement for existing Bambu 0.1.0
- Implementation baseline: `09dc4068e950a03eaf5c50fe56f5367be45cd79c`
- Normative generator policy SHA-256 at that baseline:
  `06fb380685154b0544b541f628210579d12cb24d8a11e1d120cad6a5f1fbc85a`

The [public packet](../../../../handoffs/bambu-studio/v0.1.0-v1/README.md)
contains only neutral opaque metadata and bounded conclusions. This directory
owns generator-local evidence and its reproduction. It supplies no service
installation, configuration write or production authorization.

## Evidence Basis And Terms

This is a release-only metadata supplement. It preserves every file under the
original `releases/v0.1.0/`, every original provenance set, the immutable release
tag and all package/source/descriptor/executable bytes. Original capability,
target source influence, terms, notice and compatibility reviews remain those
at the baseline; this work introduces no target behavior or new compatibility
claim. Opaque identity strings and neutral transport limits were consulted in
the baseline runtime declarations and interface document, then checked against
actual successful execution of the released binary. No target implementation,
layout, fixture or source-informed summary is included in the public handoff.

The unchanged service deployment schema and its adapted handoff-schema fragment
are from `altendky/onshape-export`, commit
`9961f6f1b56067c452447e90b63ad969e8d1db46`, declared MIT OR Apache-2.0; the MIT
choice is retained with its full copyright/permission/disclaimer. The exact
schema byte hash is
`2af98b181a53c1a8c26b51d95bc974561871c2f913bb37e8275f5c3fe47f2b90`.
The neutral protocol/settings schemas remain identical to the approved release
copies; their official immutable references and hashes are in the handoff.

The full consulted service `docs/src/project/deployed-generator.md` at that
commit has SHA-256
`45e5dea07988bf6ff904a79cd48c21f69271de5d9261befb3b3dac66aa8caa9f`,
including the static identity algorithm. Exact release-local consulted-file
SHA-256 values at `54a0fa1635f75039d6b858a95bbe803bb61103cf` are:

| Repository-relative file | SHA-256 |
| --- | --- |
| `crates/bambu-studio/src/cache_key.rs` | 638e9c31372ab7f7a3344eb33e6ce4d510c0c3cc809ac11716be421083e8738d |
| `crates/bambu-studio/src/generator_protocol.rs` | 3ed70d2822cfed7f07c63b2fcda399cfb1d6134f5646a221eb5dba95a21540f8 |
| `crates/bambu-studio/src/runtime.rs` | 6672894d20a841ca47799dcba84855433266ed473d1eb17b1cd5b7ab689a366f |
| `crates/bambu-studio/src/main.rs` | 8b7133275a2733c1bff15361ba05adec24987719e51bafdb0e3f212e7d068b7d |
| `docs/src/project/bambu-protocol-v1.md` | cafd8e6cd8fe0942d23e897340568ce797e7be2393071b1a611ec654cfd8d9d0 |

| Relationship | Classification | Evidence kind and source |
| --- | --- | --- |
| Public copied service schema and MIT notice | direct_source_reuse | source_informed: pinned service `config/deployed-generator/v1/deployed-generator.schema.json` and `LICENSE-MIT` |
| Closed handoff schema's deployment block | adapted_algorithm | source_informed: same pinned service schema, removing only operational executablePath and adding exact reviewed-value constraints; adaptation authored 2026-10-12 |
| Verifier invocation/descriptor/settings-schema identity calculations | adapted_algorithm | source_informed: released neutral `crates/bambu-studio/src/cache_key.rs` lines 10–35 and `generator_protocol.rs` lines 402–439, 742–758 at implementation `54a0fa1635f75039d6b858a95bbe803bb61103cf`; inherited protocol MIT source-influence and terms records apply |
| Static deployment identity verification | adapted_algorithm | source_informed: pinned service deployed-generator contract's NUL-prefix/JCS scope; neutral MIT terms apply |
| Opaque release binding association and neutral interface declarations | schema_fact | source_informed_schema_fact: baseline runtime `runtime.rs` lines 18–54, `main.rs` lines 4–19, interface document and original release records; interpreted only generator-locally |
| Installed published-binary acceptance, unchanged-byte measurements, repeated reports | independently_derived_behavior | independently_derived_behavior: original experiment below with explicit prior generator-local source use; no clean-room claim |

Official consulted release-local files are fixed at
`54a0fa1635f75039d6b858a95bbe803bb61103cf` or the baseline commit. Their source
hashes and notices remain in the original source inventory. Service reference:

https://github.com/altendky/onshape-export/blob/9961f6f1b56067c452447e90b63ad969e8d1db46/config/deployed-generator/v1/deployed-generator.schema.json

https://github.com/altendky/onshape-export/blob/9961f6f1b56067c452447e90b63ad969e8d1db46/docs/src/project/deployed-generator.md#L13-L42

https://github.com/altendky/onshape-export/blob/9961f6f1b56067c452447e90b63ad969e8d1db46/docs/src/project/deployed-generator.md#L70-L85

https://github.com/altendky/slicer-project-generators/blob/54a0fa1635f75039d6b858a95bbe803bb61103cf/crates/bambu-studio/src/cache_key.rs#L10-L35

https://github.com/altendky/slicer-project-generators/blob/54a0fa1635f75039d6b858a95bbe803bb61103cf/crates/bambu-studio/src/runtime.rs#L18-L54

The reproduction tool uses jsonschema 4.25.1 and rfc8785 0.1.4, with full resolved
dependency versions/archive checksums in `verify.py.lock`. These are external
verification tools, not shipped generator dependencies or a change to the
original generator graph. No third-party implementation text is copied from
these libraries. rfc8785's upstream distribution uses Apache-2.0; jsonschema's
uses MIT. Repository-authored tooling remains AGPL-3.0-only, with retained MIT
attribution for adapted neutral contracts. No library or target executable is
redistributed by this supplement. New named reviews bind the actual metadata
and report hashes in the [review](review.md).

## Reproduction

Use a session temporary directory for `BBL_RELEASE`, `BBL_INSTALLATION`,
`BBL_FIXTURES`, `BBL_WORK` and `BBL_REPORT`. Download and verify the unchanged
published assets before extracting the package into a fresh installation:

```console
gh release download bambu-studio-v0.1.0 --dir "$BBL_RELEASE"
gh release verify bambu-studio-v0.1.0
gh release verify-asset bambu-studio-v0.1.0 \
  "$BBL_RELEASE/bambu-studio-v0.1.0-x86_64-unknown-linux-gnu.tar.gz"
gh release verify-asset bambu-studio-v0.1.0 \
  "$BBL_RELEASE/bambu-studio-v0.1.0-corresponding-source.tar.gz"
tar -xf "$BBL_RELEASE/bambu-studio-v0.1.0-x86_64-unknown-linux-gnu.tar.gz" \
  -C "$BBL_INSTALLATION"
cargo +1.94.1 run --locked --offline --example protocol_validation_fixture -- \
  "$BBL_FIXTURES" "$BBL_INSTALLATION/slicer-project-generator-bambu-studio"
uv run --locked provenance/bambu-studio/handoffs/v0.1.0-v1/verify.py \
  --release-dir "$BBL_RELEASE" --installation "$BBL_INSTALLATION" \
  --fixtures "$BBL_FIXTURES" --work "$BBL_WORK" --report "$BBL_REPORT"
```

The fixture example builds only a local fixture recipe; it does not rebuild the
published generator. Its development descriptors are ignored. The verifier
invokes the downloaded installed binary with its actual adjacent descriptor.
It replaces every expected static binding from `immutableDeployment`, adds the
invocation's settings identity, and recomputes the neutral invocation identity.
Full expected/reported objects persist in [verification.json](verification.json).
Synthetic geometry inputs and candidates remain temporary generator-local files.

Five successful cases use the original source-authored fixtures and compare
candidate hashes with the existing approved release evidence. No target app is
rerun and no new target compatibility is claimed. Three ordinary failure cases
exercise malformed request, unmatched identity and declared output-limit handling;
they cannot establish successful binding acceptance. Parsed failures also match
the altered invocation identity. Every result is checked against the unchanged
neutral schema, measured limits, and candidate absence/presence rules.

The complete installed file set and digests match the approved release inventory;
the inventory itself is compared with the immutable release copy. Package/source,
binary and descriptor measurements are repeated after execution. Two fresh
invocation trees produce identical full reports in the recorded Python 3.12.13
environment. Candidate/result checks observe final files after exit; they do not
claim to observe rename timing. Candidate-before-result ordering and result-write
cleanup declarations inherit the exact reviewed runtime and its release tests.

Running the verifier without arguments performs the offline handoff/schema/hash
checks, strict JSON and nested-closure regression checks, settings-schema JCS
identity, the exact service-document construction, relocation invariance and
mutation checks for all 14 immutable fields. CI uses the locked UV tool and script
dependency graph for this check. The actual installed-binary experiment is
generator-local release evidence, not a service CI fixture.
