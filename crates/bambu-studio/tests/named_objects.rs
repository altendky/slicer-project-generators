mod fixtures;

use std::io::{Cursor, Read};

use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use slicer_project_generator_bambu_studio::{
    GenerationError, NamedObject, ValidationErrorKind, generate_named_objects,
};
use zip::ZipArchive;

use fixtures::{object, tetrahedron};

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

fn model_objects(bytes: &[u8]) -> Vec<(u32, String)> {
    let model = archive_entry(bytes, "3D/3dmodel.model");
    let mut reader = Reader::from_str(&model);
    let mut objects = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(event) if event.local_name().as_ref() == b"object" => {
                let mut id = None;
                let mut name = None;
                for attribute in event.attributes() {
                    let attribute = attribute.unwrap();
                    match attribute.key.local_name().as_ref() {
                        b"id" => {
                            id = Some(
                                attribute
                                    .normalized_value(XmlVersion::Explicit1_0)
                                    .unwrap()
                                    .parse::<u32>()
                                    .unwrap(),
                            );
                        }
                        b"name" => {
                            name = Some(
                                attribute
                                    .normalized_value(XmlVersion::Explicit1_0)
                                    .unwrap()
                                    .into_owned(),
                            );
                        }
                        _ => {}
                    }
                }
                objects.push((id.unwrap(), name.unwrap()));
            }
            Event::Eof => break,
            _ => {}
        }
    }
    objects
}

fn validation_kinds(objects: &[NamedObject]) -> Vec<ValidationErrorKind> {
    match generate_named_objects(objects) {
        Err(GenerationError::Validation(errors)) => {
            errors.into_iter().map(|error| error.kind).collect()
        }
        result => panic!("expected validation failure, got {result:?}"),
    }
}

#[test]
fn generates_deterministic_named_objects_with_unicode_and_escaping() {
    let objects = vec![
        object("opaque-a", "Bracket & <left> \"A\"", 0.0),
        object("opaque-b", "支持部品 café\tline 2", 20.0),
    ];
    let first = generate_named_objects(&objects).unwrap();
    let second = generate_named_objects(&objects).unwrap();

    assert_eq!(first.bytes, second.bytes);
    assert_eq!(
        first
            .object_mappings
            .iter()
            .map(|mapping| (mapping.identity.as_str(), mapping.object_id))
            .collect::<Vec<_>>(),
        [("opaque-a", 1), ("opaque-b", 2)]
    );
    assert_eq!(
        model_objects(&first.bytes),
        [
            (1, "Bracket & <left> \"A\"".to_owned()),
            (2, "支持部品 café\tline 2".to_owned())
        ]
    );

    let mut archive = ZipArchive::new(Cursor::new(&first.bytes)).unwrap();
    assert_eq!(archive.len(), 3);
    assert!(archive.by_name("[Content_Types].xml").is_ok());
    assert!(archive.by_name("_rels/.rels").is_ok());
    assert!(archive.by_name("3D/3dmodel.model").is_ok());
}

#[test]
fn permits_duplicate_display_names_for_distinct_identities() {
    let project = generate_named_objects(&[
        object("opaque-a", "Same name", 0.0),
        object("opaque-b", "Same name", 20.0),
    ])
    .unwrap();

    assert_eq!(
        model_objects(&project.bytes),
        [(1, "Same name".to_owned()), (2, "Same name".to_owned())]
    );
}

#[test]
fn rejects_missing_and_empty_mappings_atomically() {
    let objects = [
        NamedObject {
            identity: None,
            name: None,
            mesh: None,
        },
        NamedObject {
            identity: Some(String::new()),
            name: Some(String::new()),
            mesh: Some(tetrahedron(0.0)),
        },
    ];

    let kinds = validation_kinds(&objects);
    assert!(kinds.contains(&ValidationErrorKind::MissingIdentity));
    assert!(kinds.contains(&ValidationErrorKind::MissingName));
    assert!(kinds.contains(&ValidationErrorKind::MissingGeometry));
    assert!(kinds.contains(&ValidationErrorKind::EmptyIdentity));
    assert!(kinds.contains(&ValidationErrorKind::EmptyName));
}

#[test]
fn distinguishes_duplicate_conflicting_and_ambiguous_identity_mappings() {
    let base = object("opaque", "name", 0.0);
    let objects = [
        base.clone(),
        base.clone(),
        object("opaque", "other", 0.0),
        object("opaque", "name", 20.0),
        object("opaque", "other", 20.0),
    ];
    let kinds = validation_kinds(&objects);

    assert!(matches!(
        kinds[0],
        ValidationErrorKind::DuplicateIdentity {
            first_object_index: 0
        }
    ));
    assert!(matches!(
        kinds[1],
        ValidationErrorKind::ConflictingNameMapping {
            first_object_index: 0
        }
    ));
    assert!(matches!(
        kinds[2],
        ValidationErrorKind::AmbiguousIdentityMapping {
            first_object_index: 0
        }
    ));
    assert!(matches!(
        kinds[3],
        ValidationErrorKind::ConflictingIdentityMapping {
            first_object_index: 0
        }
    ));
}

#[test]
fn rejects_invalid_xml_and_geometry() {
    let mut bad = object("opaque", "bad\u{1}name", 0.0);
    let mesh = bad.mesh.as_mut().unwrap();
    mesh.vertices[0][0] = f64::NAN;
    mesh.triangles.push([0, 0, 1]);
    mesh.triangles.push([0, 1, 99]);
    let kinds = validation_kinds(&[bad]);

    assert!(matches!(
        kinds[0],
        ValidationErrorKind::InvalidXmlCharacter {
            character: '\u{1}',
            ..
        }
    ));
    assert!(matches!(
        kinds[1],
        ValidationErrorKind::NonFiniteCoordinate {
            vertex_index: 0,
            axis: 0
        }
    ));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::DegenerateTriangle { triangle_index: 4 }
    )));
    assert!(kinds.iter().any(|kind| matches!(
        kind,
        ValidationErrorKind::VertexIndexOutOfBounds {
            triangle_index: 5,
            vertex_index: 99
        }
    )));
}

#[test]
fn rejects_empty_request_and_empty_mesh() {
    assert_eq!(validation_kinds(&[]), [ValidationErrorKind::NoObjects]);

    let kinds = validation_kinds(&[NamedObject::new(
        "opaque",
        "name",
        slicer_project_generator_bambu_studio::IndexedTriangleMesh {
            vertices: vec![],
            triangles: vec![],
        },
    )]);
    assert!(kinds.contains(&ValidationErrorKind::EmptyVertices));
    assert!(kinds.contains(&ValidationErrorKind::EmptyTriangles));
}
