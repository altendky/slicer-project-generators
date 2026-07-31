# Bambu Named Objects Rust Dependency Review v1

## Identity And Scope

- Record ID: `BBL-NAMED-OBJECTS-DEPENDENCIES-v1`
- Capability: `bambu-studio.named-objects` revision `1`
- Resolution: `Cargo.lock` at implementation commit
  `383e480673d73096942cd11d04ebe597f467a05b`
- Lockfile SHA-256:
  `34becedb9497d0cab6ff3e1acba547342e3e288c6d6dd43693828b27280ecf94`
- Review date: 2026-07-31

`zip` 7.2.0 is the runtime archive writer. Default features are disabled, so
the generator writes stored ZIP entries and resolves only `crc32fast` 1.5.0,
`cfg-if` 1.0.4, `indexmap` 2.14.0, `equivalent` 1.0.2, `hashbrown` 0.17.1,
`memchr` 2.8.3, and `typed-path` 0.12.3. `quick-xml` 0.41.0 is a development-only
test parser and also resolves `memchr` 2.8.3.

Cargo metadata declares `zip` and `quick-xml` as MIT. Every transitive package
offers MIT in its SPDX expression; this project selects MIT where an expression
offers a choice. No package uses a custom `license-file`, unknown registry, Git
source, wildcard requirement, or duplicate version. `deny.toml` permits MIT and
AGPL-3.0-only, and `cargo deny check` validates advisories, bans, licenses, and
sources. Applicable MIT copyright and permission notices must accompany any
future binary distribution; no generator release is authorized by this record.
