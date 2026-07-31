# Bambu Studio 2.7.1.62 Terms And Compatibility Review

## Record Identity

- Record ID: `BBL-UPSTREAM-TERMS-v1`
- Reviewed upstream: `bambulab/BambuStudio`
- Canonical URL: <https://github.com/bambulab/BambuStudio>
- Release: `v02.07.01.62` (`2.7.1.62 Public Release`)
- Pinned revision: `42d319c6692fa8e64790fddf0cdaafd2a4254bcc`
- Destination crate: `crates/bambu-studio`
- Destination license: `AGPL-3.0-only`
- Applicable capabilities: `bambu-studio.named-objects` revision `1` and
  `bambu-studio.support-blocking-volumes` revision `1`
- Review date: 2026-07-30

This is a repository compliance record under the canonical provenance policy,
not legal advice or a release approval.

## Material And Planned Use

The reviewed source material is the Bambu project 3MF implementation and model
definitions identified by the two capability baselines. The planned use is to
learn target schema facts and, where implementation work actually establishes
it, adapt constants or logic into an AGPL-3.0-only Rust generator. No upstream
source file, binary, fixture, generated project, networking component, or
dependency is incorporated by this review itself.

## License, Copyright, And Attribution Facts

The pinned upstream [`README.md` license section](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/README.md#L42-L52)
states that Bambu Studio is licensed under GNU Affero General Public License
version 3. The pinned root [`LICENSE`](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/LICENSE)
contains the GNU Affero General Public License version 3 text. No grant of an
"or later" option for the Bambu Studio program is asserted by this review.

The pinned [`README.md` lineage statement](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/README.md#L8-L8)
and [license section](https://github.com/bambulab/BambuStudio/blob/42d319c6692fa8e64790fddf0cdaafd2a4254bcc/README.md#L42-L49)
identify these attribution and copyright facts:

- Bambu Studio is based on PrusaSlicer by Prusa Research.
- PrusaSlicer is owned by Prusa Research and is based on Slic3r.
- Slic3r was created by Alessandro Ranellucci with help from contributors.

The consulted implementation files do not contain a complete file-level
copyright inventory. Their Git histories include work from multiple upstream
projects and contributors. Each later source-influence record must identify the
exact lines used and preserve applicable Bambu Studio, PrusaSlicer, Slic3r,
author, and contributor notices established for those lines. Unknown or
disputed ownership blocks affected public incorporation; it must not be filled
with a guessed copyright statement.

## Exceptions And Excluded Components

No file-specific license exception or additional term was identified in the
consulted `bbs_3mf.cpp`, `Model.hpp`, or `Model.cpp` files. That finding applies
only to those files at the pinned revision and does not waive notices discovered
through their histories or through material consulted later.

The upstream README states that the optional Bambu networking plugin is based
on non-free libraries. The plugin, its libraries, protocol, binaries, and
networking functionality are outside both capability baselines and must not be
used, incorporated, tested, packaged, or distributed under this review. Any
future use requires a separate exact-material terms and compatibility review.

## Compatibility And Obligations

The upstream's stated GNU AGPL version 3 license and the destination crate's
`AGPL-3.0-only` declaration are compatible for the planned derivative source
incorporation described here, subject to compliance with the license and all
applicable notices. This conclusion is limited to the identified source and
planned use; it is not a conclusion for every file, dependency, binary,
fixture, service, or generated artifact in the upstream repository.

Applicable obligations for affected derivative material include:

- Keep applicable copyright, license, attribution, and warranty notices.
- Mark modified source prominently with the fact and relevant date of
  modification.
- License the covered work as a whole under GNU AGPL version 3 when conveyed.
- Provide complete corresponding source in the manner required for conveyed
  object code.
- If a modified covered program supports remote network interaction, offer its
  corresponding source to remote users as required by AGPL section 13.
- Do not impose further restrictions inconsistent with the GNU AGPL.

The final notice record must state the Bambu Studio, PrusaSlicer, and Slic3r
lineage and retain any file- or contribution-specific notices found during
implementation provenance review. Package and service distribution plans must
be reviewed separately against the actual candidate.

## Access Terms And Non-Source Inputs

The source and release metadata were accessed from the public canonical GitHub
repository through ordinary GitHub web and API access on 2026-07-30. The
applicable GitHub Terms of Service have an effective date of 2026-04-27 and are
pinned here at GitHub's canonical site-policy commit
[`166eb97e485b7de3cd5dbaf8c8d16e4310fda46c`](https://github.com/github/site-policy/blob/166eb97e485b7de3cd5dbaf8c8d16e4310fda46c/Policies/github-terms/github-terms-of-service.md#L41-L45),
Git blob SHA-1 `1312224e6c0338e627f37644b697ce415d747886`.

The terms state that they do not restrict lawful access to public repository
contents in
[section D.8](https://github.com/github/site-policy/blob/166eb97e485b7de3cd5dbaf8c8d16e4310fda46c/Policies/github-terms/github-terms-of-service.md#L155-L159).
Their [API terms](https://github.com/github/site-policy/blob/166eb97e485b7de3cd5dbaf8c8d16e4310fda46c/Policies/github-terms/github-terms-of-service.md#L217-L229)
prohibit abusive or excessively frequent requests, sharing tokens to exceed
rate limits, and downloads for spam. This research used ordinary low-volume
repository and API access, did not share a token or evade rate limits, and did
not collect data for spam. No private or confidential upstream material was
accessed.

GitHub is an access host, not the source-license grantor. This review retains
immutable repository links and factual metadata; it does not incorporate GitHub
website presentation or claim rights beyond the upstream license and the
reviewed service access.

No Bambu Studio binary, EULA-governed installer, project fixture, sample model,
captured output, generated output, third-party specification, target service,
or proprietary documentation was used or retained for this baseline. There is
therefore no review result for any such material. Before later work uses any of
those categories, its exact version or immutable artifact, terms, planned use,
retention, modification, testing, publication, and distribution requirements
must be added to a named review. Unreviewed use blocks affected public
incorporation or distribution.

## Review Outcome And Limits

The selected source baseline may support the two planned AGPL-3.0-only
capabilities under the conditions above. Public implementation work remains
blocked until relationship-level source influence, exact consulted lines,
content hashes, local paths, authorship, notices, and any newly used input
reviews are complete. Generator release remains blocked until immutable
implementation, fixtures, tests, target-version results, provenance set,
package identity, hashes, distribution review, and named release approval all
exist. No implementation, test, fixture, artifact, qualified legal opinion, or
release approval is asserted here.
