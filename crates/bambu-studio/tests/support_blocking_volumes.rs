use std::collections::BTreeMap;
use std::io::{Cursor, Read};

use quick_xml::Reader;
use quick_xml::XmlVersion;
use quick_xml::events::Event;
use quick_xml::name::QName;
use slicer_project_generator_bambu_studio::{
    GenerationError, GeometryRole, IndexedTriangleMesh, ProjectGeometry, ValidationErrorKind,
    generate_support_blocking_volumes,
};
use zip::ZipArchive;

fn tetrahedron(offset: f64) -> IndexedTriangleMesh {
    IndexedTriangleMesh {
        vertices: vec![
            [offset, 0.0, 0.0],
            [offset + 10.0, 0.0, 0.0],
            [offset, 10.0, 0.0],
            [offset, 0.0, 10.0],
        ],
        triangles: vec![[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]],
    }
}

fn printable(identity: &str, name: &str, offset: f64) -> ProjectGeometry {
    ProjectGeometry::printable(identity, tetrahedron(offset)).with_name(name)
}

fn blocker(identity: &str, target_identity: &str, name: &str, offset: f64) -> ProjectGeometry {
    ProjectGeometry::support_blocker(identity, target_identity, tetrahedron(offset)).with_name(name)
}

fn archive_entry(bytes: &[u8], path: &str) -> String {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut value = String::new();
    archive
        .by_name(path)
        .unwrap()
        .read_to_string(&mut value)
        .unwrap();
    value
}

fn assert_well_formed(xml: &str) {
    let mut reader = Reader::from_str(xml);
    loop {
        if reader.read_event().unwrap() == Event::Eof {
            break;
        }
    }
}

fn attribute_u32(event: &quick_xml::events::BytesStart<'_>, name: &[u8]) -> u32 {
    event
        .try_get_attribute(QName(name))
        .unwrap()
        .unwrap()
        .normalized_value(XmlVersion::Explicit1_0)
        .unwrap()
        .parse()
        .unwrap()
}

fn component_associations(model: &str) -> BTreeMap<u32, Vec<u32>> {
    let mut reader = Reader::from_str(model);
    let mut associations: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    let mut current_object = None;
    let mut in_components = false;
    loop {
        match reader.read_event().unwrap() {
            Event::Start(event) if event.local_name().as_ref() == b"object" => {
                current_object = Some(attribute_u32(&event, b"id"));
            }
            Event::Start(event) if event.local_name().as_ref() == b"components" => {
                in_components = true;
                associations.entry(current_object.unwrap()).or_default();
            }
            Event::Empty(event) if event.local_name().as_ref() == b"component" => {
                assert!(in_components);
                associations
                    .get_mut(&current_object.unwrap())
                    .unwrap()
                    .push(attribute_u32(&event, b"objectid"));
            }
            Event::End(event) if event.local_name().as_ref() == b"components" => {
                in_components = false;
            }
            Event::End(event) if event.local_name().as_ref() == b"object" => {
                current_object = None;
            }
            Event::Eof => break,
            _ => {}
        }
    }
    associations
}

fn part_associations(settings: &str) -> BTreeMap<u32, Vec<(u32, String)>> {
    let mut reader = Reader::from_str(settings);
    let mut associations: BTreeMap<u32, Vec<(u32, String)>> = BTreeMap::new();
    let mut current_object = None;
    loop {
        match reader.read_event().unwrap() {
            Event::Start(event) if event.local_name().as_ref() == b"object" => {
                let object_id = attribute_u32(&event, b"id");
                associations.insert(object_id, Vec::new());
                current_object = Some(object_id);
            }
            Event::Start(event) if event.local_name().as_ref() == b"part" => {
                let subtype = event
                    .try_get_attribute("subtype")
                    .unwrap()
                    .unwrap()
                    .normalized_value(XmlVersion::Explicit1_0)
                    .unwrap()
                    .into_owned();
                associations
                    .get_mut(&current_object.unwrap())
                    .unwrap()
                    .push((attribute_u32(&event, b"id"), subtype));
            }
            Event::End(event) if event.local_name().as_ref() == b"object" => {
                current_object = None;
            }
            Event::Eof => break,
            _ => {}
        }
    }
    associations
}

fn validation_kinds(entries: &[ProjectGeometry]) -> Vec<ValidationErrorKind> {
    match generate_support_blocking_volumes(entries) {
        Err(GenerationError::Validation(errors)) => {
            errors.into_iter().map(|error| error.kind).collect()
        }
        result => panic!("expected validation failure, got {result:?}"),
    }
}

#[test]
fn generates_deterministic_models_with_associated_support_blockers() {
    let entries = [
        printable("model-a", "Bracket & <A>", 0.0),
        blocker("blocker-a1", "model-a", "A blocker", 2.0),
        printable("model-b", "Second model", 30.0),
        blocker("blocker-b", "model-b", "B blocker", 32.0),
        blocker("blocker-a2", "model-a", "A blocker 2", 4.0),
    ];
    let first = generate_support_blocking_volumes(&entries).unwrap();
    let second = generate_support_blocking_volumes(&entries).unwrap();

    assert_eq!(first.bytes, second.bytes);
    assert_eq!(
        first
            .volume_mappings
            .iter()
            .map(|mapping| {
                (
                    mapping.identity.as_str(),
                    mapping.containing_object_id,
                    mapping.volume_object_id,
                )
            })
            .collect::<Vec<_>>(),
        [
            ("model-a", 6, 1),
            ("blocker-a1", 6, 2),
            ("model-b", 7, 3),
            ("blocker-b", 7, 4),
            ("blocker-a2", 6, 5),
        ]
    );

    let mut archive = ZipArchive::new(Cursor::new(&first.bytes)).unwrap();
    assert_eq!(archive.len(), 4);
    assert!(archive.by_name("[Content_Types].xml").is_ok());
    assert!(archive.by_name("_rels/.rels").is_ok());
    assert!(archive.by_name("3D/3dmodel.model").is_ok());
    assert!(archive.by_name("Metadata/model_settings.config").is_ok());
    drop(archive);

    let model = archive_entry(&first.bytes, "3D/3dmodel.model");
    assert_well_formed(&model);
    assert!(model.contains("<object id=\"2\" type=\"other\"><mesh>"));
    assert!(model.contains("<object id=\"4\" type=\"other\"><mesh>"));
    assert_eq!(
        component_associations(&model),
        BTreeMap::from([(6, vec![1, 2, 5]), (7, vec![3, 4])])
    );
    assert!(model.contains("<item objectid=\"6\""));
    assert!(model.contains("<item objectid=\"7\""));
    assert!(!model.contains("<item objectid=\"2\""));
    assert!(!model.contains("<item objectid=\"4\""));

    let settings = archive_entry(&first.bytes, "Metadata/model_settings.config");
    assert_well_formed(&settings);
    assert!(
        settings.contains(
            "<object id=\"6\"><metadata key=\"name\" value=\"Bracket &amp; &lt;A&gt;\"/>"
        )
    );
    assert!(settings.contains("<part id=\"1\" subtype=\"normal_part\">"));
    assert!(settings.contains("<part id=\"2\" subtype=\"support_blocker\">"));
    assert!(settings.contains("<part id=\"5\" subtype=\"support_blocker\">"));
    assert!(settings.contains("<object id=\"7\">"));
    assert!(settings.contains("<part id=\"3\" subtype=\"normal_part\">"));
    assert!(settings.contains("<part id=\"4\" subtype=\"support_blocker\">"));
    assert_eq!(
        part_associations(&settings),
        BTreeMap::from([
            (
                6,
                vec![
                    (1, "normal_part".to_owned()),
                    (2, "support_blocker".to_owned()),
                    (5, "support_blocker".to_owned()),
                ],
            ),
            (
                7,
                vec![
                    (3, "normal_part".to_owned()),
                    (4, "support_blocker".to_owned()),
                ],
            ),
        ])
    );
}

#[test]
fn permits_models_without_blockers_and_optional_names() {
    let entries = [
        ProjectGeometry::printable("model-a", tetrahedron(0.0)),
        ProjectGeometry::printable("model-b", tetrahedron(20.0)),
        ProjectGeometry::support_blocker("blocker", "model-b", tetrahedron(22.0)),
    ];
    let project = generate_support_blocking_volumes(&entries).unwrap();
    let settings = archive_entry(&project.bytes, "Metadata/model_settings.config");

    assert!(
        settings
            .contains("<object id=\"4\"><part id=\"1\" subtype=\"normal_part\"></part></object>")
    );
    assert!(settings.contains("<object id=\"5\">"));
    assert!(settings.contains("<part id=\"3\" subtype=\"support_blocker\"></part>"));
}

#[test]
fn rejects_missing_unsupported_and_conflicting_roles_atomically() {
    let base = printable("duplicate", "same", 0.0);
    let duplicate_blocker = blocker("duplicate-blocker", "duplicate", "same blocker", 5.0);
    let entries = [
        base.clone(),
        base.clone(),
        duplicate_blocker.clone(),
        duplicate_blocker,
        ProjectGeometry {
            identity: Some("missing-role".to_owned()),
            name: None,
            mesh: Some(tetrahedron(10.0)),
            role: None,
        },
        ProjectGeometry {
            identity: Some("unsupported".to_owned()),
            name: None,
            mesh: Some(tetrahedron(20.0)),
            role: Some(GeometryRole::Unsupported("support_enforcer".to_owned())),
        },
        ProjectGeometry {
            role: Some(GeometryRole::SupportBlocker {
                target_identity: Some("duplicate".to_owned()),
            }),
            ..base
        },
    ];
    let kinds = validation_kinds(&entries);

    assert!(kinds.contains(&ValidationErrorKind::MissingRole));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::DuplicateIdentity {
            first_object_index: 0
        }
    )));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::UnsupportedRole { role } if role == "support_enforcer"
    )));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::ConflictingRoleMapping {
            first_object_index: 0
        }
    )));
    assert!(kinds.contains(&ValidationErrorKind::AmbiguousBlockerTarget));
}

#[test]
fn rejects_incomplete_and_invalid_blocker_targets() {
    let entries = [
        printable("model", "model", 0.0),
        ProjectGeometry {
            identity: Some("missing".to_owned()),
            name: None,
            mesh: Some(tetrahedron(10.0)),
            role: Some(GeometryRole::SupportBlocker {
                target_identity: None,
            }),
        },
        ProjectGeometry::support_blocker("empty", "", tetrahedron(20.0)),
        ProjectGeometry::support_blocker("unknown", "not-present", tetrahedron(30.0)),
        ProjectGeometry::support_blocker("target", "model", tetrahedron(40.0)),
        ProjectGeometry::support_blocker("wrong-kind", "target", tetrahedron(50.0)),
    ];
    let kinds = validation_kinds(&entries);

    assert!(kinds.contains(&ValidationErrorKind::MissingBlockerTarget));
    assert!(kinds.contains(&ValidationErrorKind::EmptyBlockerTarget));
    assert!(kinds.contains(&ValidationErrorKind::UnknownBlockerTarget));
    assert!(kinds.contains(&ValidationErrorKind::BlockerTargetNotPrintable));
}

#[test]
fn rejects_invalid_geometry_and_names_for_every_role() {
    let mut model = ProjectGeometry::printable("model", tetrahedron(0.0)).with_name("bad\u{1}");
    model.mesh.as_mut().unwrap().vertices[0][0] = f64::MAX;
    let mut blocker = ProjectGeometry::support_blocker(
        "blocker",
        "model",
        IndexedTriangleMesh {
            vertices: vec![
                [10_000_000_000.0, 0.0, 0.0],
                [10_000_000_001.0, 0.0, 0.0],
                [10_000_000_000.0, 1.0, 0.0],
            ],
            triangles: vec![[0, 1, 2]],
        },
    );
    blocker.name = Some(String::new());
    let kinds = validation_kinds(&[model, blocker]);

    assert!(
        kinds
            .iter()
            .any(|kind| matches!(kind, ValidationErrorKind::InvalidXmlCharacter { .. }))
    );
    assert!(
        kinds
            .iter()
            .any(|kind| matches!(kind, ValidationErrorKind::CoordinateOutOfTargetRange { .. }))
    );
    assert!(kinds.contains(&ValidationErrorKind::EmptyName));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::DegenerateTriangleInTarget { triangle_index: 0 }
    )));

    let mut out_of_range = ProjectGeometry::printable("model", tetrahedron(0.0));
    out_of_range.mesh.as_mut().unwrap().triangles[0][0] = usize::MAX;
    let kinds = validation_kinds(&[out_of_range]);
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::TargetIndexOutOfRange {
            triangle_index: 0,
            vertex_index: usize::MAX
        }
    )));
}

#[test]
fn rejects_empty_requests_and_incomplete_or_malformed_geometry() {
    assert_eq!(validation_kinds(&[]), vec![ValidationErrorKind::NoObjects]);

    let entries = [
        ProjectGeometry {
            identity: None,
            name: None,
            mesh: None,
            role: Some(GeometryRole::PrintableModel),
        },
        ProjectGeometry {
            identity: Some(String::new()),
            name: None,
            mesh: Some(IndexedTriangleMesh {
                vertices: Vec::new(),
                triangles: Vec::new(),
            }),
            role: Some(GeometryRole::PrintableModel),
        },
        ProjectGeometry::printable(
            "non-finite",
            IndexedTriangleMesh {
                vertices: vec![[f64::NAN, 0.0, 0.0]],
                triangles: vec![[0, 1, 2]],
            },
        ),
        ProjectGeometry::printable(
            "degenerate",
            IndexedTriangleMesh {
                vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
                triangles: vec![[0, 1, 2]],
            },
        ),
    ];
    let kinds = validation_kinds(&entries);

    assert!(kinds.contains(&ValidationErrorKind::MissingIdentity));
    assert!(kinds.contains(&ValidationErrorKind::MissingGeometry));
    assert!(kinds.contains(&ValidationErrorKind::EmptyIdentity));
    assert!(kinds.contains(&ValidationErrorKind::EmptyVertices));
    assert!(kinds.contains(&ValidationErrorKind::EmptyTriangles));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::NonFiniteCoordinate {
            vertex_index: 0,
            axis: 0
        }
    )));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::VertexIndexOutOfBounds {
            triangle_index: 0,
            vertex_index: 1 | 2
        }
    )));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::DegenerateTriangle { triangle_index: 0 }
    )));
}

#[test]
fn rejects_requests_without_printable_models() {
    let entries = [ProjectGeometry::support_blocker(
        "blocker",
        "missing-model",
        tetrahedron(0.0),
    )];
    let kinds = validation_kinds(&entries);

    assert!(kinds.contains(&ValidationErrorKind::NoPrintableModels));
    assert!(kinds.contains(&ValidationErrorKind::UnknownBlockerTarget));
}
