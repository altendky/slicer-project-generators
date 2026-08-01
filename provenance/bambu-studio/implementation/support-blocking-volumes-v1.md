# Bambu Support-Blocking Volumes Implementation v1

## Identity

- Record ID: `BBL-SUPPORT-BLOCKING-VOLUMES-IMPLEMENTATION-v1`
- Capability ID and revision: `bambu-studio.support-blocking-volumes`, `1`
- Exact validated target: Bambu Studio `2.7.1.62`
- Implementation commits:
  `50825bfc5628cc588a4ae2803b8e9477d7548b77`,
  `de0952d7ffc0476185dc9c0cf9ef2e136591361e`, and test-completeness follow-up
  `2113110348d6f45f495b52481e8a5d0b9139dd7d`
- Author: Kyle Altendorf <sda@fstab.net>
- Authored: 2026-07-31
- License: `AGPL-3.0-only`

## Contract And Local Material

`crates/bambu-studio/src/lib.rs` adds a role-aware protocol-independent input,
the capability identity and revision, volume mappings, structured validation,
and `generate_support_blocking_volumes`. A request is validated completely
before ZIP creation. Every accepted blocker targets one unique printable-model
identity and is emitted as a `support_blocker` leaf volume under that model's
containing component object. Build items reference only containing models.

Revision 1 accepts a nonempty finite collection with globally unique nonempty
identities, exactly one supported role per entry, optional nonempty
XML-compatible names, and valid indexed triangle meshes. Blockers require one
nonempty target identity resolving uniquely to printable geometry. Target IDs,
indices, coordinates, and triangle geometry must remain representable by the
pinned target. Missing, unsupported, duplicate, conflicting, contradictory, or
ambiguous mappings and malformed geometry return structured errors and no
candidate bytes.

The deterministic stored ZIP contains exactly `[Content_Types].xml`,
`_rels/.rels`, `3D/3dmodel.model`, and
`Metadata/model_settings.config`. The existing named-objects function and its
three-member archive remain unchanged. The command binary remains the failing
protocol stub pending issue #8.

| Local material | Commit | SHA-256 |
| --- | --- | --- |
| `crates/bambu-studio/src/lib.rs` | `50825bfc5628cc588a4ae2803b8e9477d7548b77` | `60d3c6709cbf16c307599bd38a1601713db79aeab15ccc4f659fb13cc1c0187f` |
| `crates/bambu-studio/examples/support_blocking_validation_fixture.rs` | `50825bfc5628cc588a4ae2803b8e9477d7548b77` | `dacafb7609014d523a1b96fcf307a2bd6cef33d47cb46cca702c940608f41ab2` |
| `crates/bambu-studio/tests/support_blocking_volumes.rs` | `2113110348d6f45f495b52481e8a5d0b9139dd7d` | `f45b4283bf2c682419823338540df8c43b4d9cc2de491a42b41e752102defbb6` |

Dependencies and source influence resolve through
[`BBL-SUPPORT-BLOCKING-VOLUMES-DEPENDENCIES-v1`](../reviews/support-blocking-volumes-rust-dependencies-v1.md)
and
[`BBL-SUPPORT-BLOCKING-VOLUMES-SOURCE-INFLUENCE-v1`](../source-influence/support-blocking-volumes-v1.md).
