# Bambu Protocol v1 Runtime

The Bambu command implements neutral protocol v1 and generator-settings v2.
Invoke a reviewed installation from the private invocation directory:

```console
/path/to/slicer-project-generator-bambu-studio \
  --request request.json --result result.json
```

Exit 0 commits a success result after the candidate. Exit 1 reports structured
failure when the result destination is writable. The caller must begin without
candidate/result files and accept only a matching success result and exit 0.
The invocation directory and staged inputs must be private and immutable while
the process runs. Existing outputs, symlinks, nonregular files, and unsafe paths
are rejected; existing files are preserved. Temporary sibling files are closed,
synced, and renamed; a result-write failure removes the newly created candidate.
No geometry is published on a parsing, generation, or self-validation failure.

## Package Binding

The executable requires a closed `package.json` beside it, containing
`buildIdentity`, `binaryIdentity`, and `provenanceSetIdentity`. It recomputes the
SHA-256 of its own executable and requires exact agreement with `binaryIdentity`.
`packageIdentity` is SHA-256 of the JCS object with domain
`bambu-generator-package-v1` and that descriptor as `payload`. It identifies the
descriptor; the release record separately identifies distributable archive bytes.
Build and provenance identities must name reviewed immutable release inputs.
Development tests use explicitly synthetic descriptors and do not authorize a
release. The trusted caller separately pins and approves the installed package.

The fixed request bindings are:

| Field | Identity |
| --- | --- |
| `dialectIdentity` | `bambu-studio-2.7.1.62-3mf-v1` |
| `capabilityIdentities` | `bambu-studio.named-objects@1`, `bambu-studio.support-blocking-volumes@1` in that order |
| `inputKindIdentity` | `raw-geometry-3mf-v1` |
| `inputSchemaIdentity` | `raw-geometry-3mf-core-profile-v1` |
| `normalizationIdentity` | `bambu-3mf-realize-meters-place-once-v1` |
| `validationIdentity` | `bambu-3mf-semantic-readback-v1` |
| `settingsSchemaIdentity` | `adfbdd411a8562cd84918ca9facf8c91f9cfeebed1a5cc606f46b2289920e465` |

Settings identity is computed using the unchanged service-owned settings-v2 JCS
domain and normalized signed zeros. Input and output media types are `model/3mf`;
settings use `application/json`. No protocol schema is extended. Source identities,
occurrence paths, producer references, filenames, and parent references permitted
by protocol v1 are opaque: they never control generated names, roles, geometry,
association, or placement. The explicit `displayName` is forwarded unchanged.
Source selection and transform derivation remain service responsibilities.

## Raw Geometry Profile

Each manifest identity has one distinct retained path. Bytes, declared length,
SHA-256, manifest/input-set/invocation identities, settings identities, and full
placement order are independently checked. Equal bytes at distinct paths are
parsed separately. Display names are optional and never act as identity.

Supported inputs are single-primary-model core 3MF packages with exactly the
content-types file, root relationships file, and primary model file. Stored and
Deflate members are supported. Optional directory entries are allowed. Multiple
mesh resources, nested same-file components, and multiple build items are
flattened into the one logical manifest object in their explicit traversal order.
Every resource must be reachable. Model units default to millimeters; micron,
millimeter, centimeter, inch, foot, and meter are supported. All internal component
and build transforms are realized in the declared local unit, followed by unit
conversion to meters, one neutral absolute placement, and conversion to target
millimeters. Reflections reverse triangle winding; singular, overflowing, or
nonfinite transformations fail. Target float conversion must retain finite,
nondegenerate triangles.

Required extensions, materials, textures, external components/relationships,
nonprintable build items, additional members, ZIP64, multipart/embedded ZIP
directories, and unknown geometry elements/attributes are unsupported. Unsupported
geometry/extension features produce structured `unsupportedRequest`; rejected
archive encodings, malformed data, and resource limits produce `invalidInput`.
No fields or transforms are silently
ignored. Optional XML language attributes and comments have no geometry semantics.

| Resource | Bound |
| --- | --- |
| Request, manifest, settings | 1 MiB each |
| Result | 256 KiB; emitted failures contain one fixed bounded error |
| Manifest objects | 256 |
| Raw archive | 16 MiB each; 64 MiB total |
| Decoded archive members | 32 MiB each archive; 64 MiB total invocation |
| ZIP directory entries | 32 |
| XML nodes/depth | 1,000,000 / 64 |
| Model resources/build items/components per resource | 256 each |
| Component depth/expanded resource visits | 32 / 4096 per logical input |
| Stored or realized vertices/triangles per input | 100,000 / 200,000 |
| Final project vertices/triangles including blocker copies | 100,000 / 200,000 |
| Candidate | Lesser of 32 MiB and the request's declared limit |

Every auxiliary input must correspond to one settings blocker, and each target
must be a printable manifest identity. A shared blocker produces one physical
volume per declared target, in manifest order followed by target order. Its input
is parsed and placed once before volume replication. Internal mapping identities
use manifest positions, never names or content hashes. Printable model build
order and each model's volume order follow the manifest.

Named-only requests with display names dispatch the named-object generator. The
support-blocking generator handles blockers and optional absent names. Both use
the same stricter target numeric checks. Final self-validation reopens the exact
completed archive and compares its package relationship/content types, geometry,
resource order, names, containing models, build membership, identity transforms,
and every blocker association against the realized request. Byte hashing and
atomic candidate/result success occur only afterward.

See the [protocol provenance set](../../../provenance/bambu-studio/provenance-sets/protocol-v1.md).
