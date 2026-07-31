# Bambu Named Objects Notices v1

- Record ID: `BBL-NAMED-OBJECTS-NOTICES-v1`
- Capability: `bambu-studio.named-objects` revision `1`
- Date: 2026-07-31

The implementation is derivative of Bambu Studio source at commit
`42d319c6692fa8e64790fddf0cdaafd2a4254bcc`, licensed under GNU Affero General
Public License version 3. Bambu Studio is based on PrusaSlicer by Prusa Research;
PrusaSlicer is based on Slic3r, created by Alessandro Ranellucci with help from
contributors. Consulted line history also identifies Bambu Studio contributors
lane.wei, chunmao.guo, zhimin.zeng, and zhou.xu. The destination remains
`AGPL-3.0-only`, modification is identified by the 2026 implementation commit,
and the root `LICENSE` supplies the complete license text.

The runtime `zip` dependency, test-only `quick-xml`, and their locked transitive
dependencies are used under MIT license choices as recorded by
[`BBL-NAMED-OBJECTS-DEPENDENCIES-v1`](../reviews/rust-dependencies-v1.md). Their
applicable copyright and permission notices must accompany any future binary
distribution. No binary distribution or generator release is authorized here.

The official AppImage and its embedded CKEditor, Swiper, Dom7, and SSR Window
notices were inspected only as compatibility-test inputs. No AppImage component
is incorporated or redistributed by this repository.
