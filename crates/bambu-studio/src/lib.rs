// Copyright (C) 2026 Slicer Project Generators contributors
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 only.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{Cursor, Write as _};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

pub const CAPABILITY_ID: &str = "bambu-studio.named-objects";
pub const CAPABILITY_REVISION: u32 = 1;
pub const VALIDATED_TARGET_VERSION: &str = "2.7.1.62";

const CONTENT_TYPES_PATH: &str = "[Content_Types].xml";
const RELATIONSHIPS_PATH: &str = "_rels/.rels";
const MODEL_PATH: &str = "3D/3dmodel.model";

#[derive(Clone, Debug, PartialEq)]
pub struct IndexedTriangleMesh {
    pub vertices: Vec<[f64; 3]>,
    pub triangles: Vec<[usize; 3]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedObject {
    pub identity: Option<String>,
    pub name: Option<String>,
    pub mesh: Option<IndexedTriangleMesh>,
}

impl NamedObject {
    #[must_use]
    pub fn new(
        identity: impl Into<String>,
        name: impl Into<String>,
        mesh: IndexedTriangleMesh,
    ) -> Self {
        Self {
            identity: Some(identity.into()),
            name: Some(name.into()),
            mesh: Some(mesh),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectMapping {
    pub identity: String,
    pub object_id: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedProject {
    pub bytes: Vec<u8>,
    pub object_mappings: Vec<ObjectMapping>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    pub object_index: Option<usize>,
    pub identity: Option<String>,
    pub kind: ValidationErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationErrorKind {
    NoObjects,
    MissingIdentity,
    EmptyIdentity,
    MissingName,
    EmptyName,
    InvalidXmlCharacter {
        character: char,
        character_index: usize,
    },
    MissingGeometry,
    EmptyVertices,
    EmptyTriangles,
    NonFiniteCoordinate {
        vertex_index: usize,
        axis: usize,
    },
    VertexIndexOutOfBounds {
        triangle_index: usize,
        vertex_index: usize,
    },
    DegenerateTriangle {
        triangle_index: usize,
    },
    DuplicateIdentity {
        first_object_index: usize,
    },
    ConflictingNameMapping {
        first_object_index: usize,
    },
    AmbiguousIdentityMapping {
        first_object_index: usize,
    },
    ConflictingIdentityMapping {
        first_object_index: usize,
    },
}

#[derive(Debug)]
pub enum GenerationError {
    Validation(Vec<ValidationError>),
    Archive(zip::result::ZipError),
    Io(std::io::Error),
}

impl From<zip::result::ZipError> for GenerationError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Archive(error)
    }
}

impl From<std::io::Error> for GenerationError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// Generates a deterministic minimum 3MF after validating the complete input.
pub fn generate_named_objects(
    objects: &[NamedObject],
) -> Result<GeneratedProject, GenerationError> {
    let errors = validate(objects);
    if !errors.is_empty() {
        return Err(GenerationError::Validation(errors));
    }

    let mut model = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <model unit=\"millimeter\" xml:lang=\"en-US\" xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\">\n\
         <resources>\n",
    );
    let mut mappings = Vec::with_capacity(objects.len());

    for (index, object) in objects.iter().enumerate() {
        let object_id = u32::try_from(index + 1).expect("validated object count fits in u32");
        let identity = object.identity.as_deref().expect("validated identity");
        let name = object.name.as_deref().expect("validated name");
        let mesh = object.mesh.as_ref().expect("validated geometry");

        write!(
            model,
            "<object id=\"{object_id}\" type=\"model\" name=\"{}\"><mesh><vertices>",
            escape_xml(name)
        )
        .expect("writing to String cannot fail");
        for [x, y, z] in &mesh.vertices {
            write!(model, "<vertex x=\"{x}\" y=\"{y}\" z=\"{z}\"/>")
                .expect("writing to String cannot fail");
        }
        model.push_str("</vertices><triangles>");
        for [v1, v2, v3] in &mesh.triangles {
            write!(model, "<triangle v1=\"{v1}\" v2=\"{v2}\" v3=\"{v3}\"/>")
                .expect("writing to String cannot fail");
        }
        model.push_str("</triangles></mesh></object>\n");
        mappings.push(ObjectMapping {
            identity: identity.to_owned(),
            object_id,
        });
    }

    model.push_str("</resources><build>");
    for mapping in &mappings {
        write!(model, "<item objectid=\"{}\"/>", mapping.object_id)
            .expect("writing to String cannot fail");
    }
    model.push_str("</build></model>\n");

    let cursor = Cursor::new(Vec::new());
    let mut archive = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o644);

    for (path, contents) in [
        (
            CONTENT_TYPES_PATH,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\n\
             <Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\n\
             <Default Extension=\"model\" ContentType=\"application/vnd.ms-package.3dmanufacturing-3dmodel+xml\"/>\n\
             </Types>\n",
        ),
        (
            RELATIONSHIPS_PATH,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n\
             <Relationship Target=\"/3D/3dmodel.model\" Id=\"rel-1\" Type=\"http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel\"/>\n\
             </Relationships>\n",
        ),
        (MODEL_PATH, model.as_str()),
    ] {
        archive.start_file(path, options)?;
        archive.write_all(contents.as_bytes())?;
    }

    let bytes = archive.finish()?.into_inner();
    Ok(GeneratedProject {
        bytes,
        object_mappings: mappings,
    })
}

fn validate(objects: &[NamedObject]) -> Vec<ValidationError> {
    if objects.is_empty() {
        return vec![ValidationError {
            object_index: None,
            identity: None,
            kind: ValidationErrorKind::NoObjects,
        }];
    }

    let mut errors = Vec::new();
    let mut identities: HashMap<&str, usize> = HashMap::new();
    if objects.len() > u32::MAX as usize {
        unreachable!("slices cannot contain more than u32::MAX objects on supported targets");
    }

    for (object_index, object) in objects.iter().enumerate() {
        let identity = object.identity.as_deref();
        let error = |kind| ValidationError {
            object_index: Some(object_index),
            identity: identity.map(str::to_owned),
            kind,
        };

        match identity {
            None => errors.push(error(ValidationErrorKind::MissingIdentity)),
            Some("") => errors.push(error(ValidationErrorKind::EmptyIdentity)),
            Some(identity) => {
                if let Some(&first_object_index) = identities.get(identity) {
                    let first = &objects[first_object_index];
                    let kind = match (first.name == object.name, first.mesh == object.mesh) {
                        (true, true) => {
                            ValidationErrorKind::DuplicateIdentity { first_object_index }
                        }
                        (false, true) => {
                            ValidationErrorKind::ConflictingNameMapping { first_object_index }
                        }
                        (true, false) => {
                            ValidationErrorKind::AmbiguousIdentityMapping { first_object_index }
                        }
                        (false, false) => {
                            ValidationErrorKind::ConflictingIdentityMapping { first_object_index }
                        }
                    };
                    errors.push(error(kind));
                } else {
                    identities.insert(identity, object_index);
                }
            }
        }

        match object.name.as_deref() {
            None => errors.push(error(ValidationErrorKind::MissingName)),
            Some("") => errors.push(error(ValidationErrorKind::EmptyName)),
            Some(name) => {
                for (character_index, character) in name.chars().enumerate() {
                    if !is_xml_character(character) {
                        errors.push(error(ValidationErrorKind::InvalidXmlCharacter {
                            character,
                            character_index,
                        }));
                    }
                }
            }
        }

        let Some(mesh) = &object.mesh else {
            errors.push(error(ValidationErrorKind::MissingGeometry));
            continue;
        };
        if mesh.vertices.is_empty() {
            errors.push(error(ValidationErrorKind::EmptyVertices));
        }
        if mesh.triangles.is_empty() {
            errors.push(error(ValidationErrorKind::EmptyTriangles));
        }
        for (vertex_index, vertex) in mesh.vertices.iter().enumerate() {
            for (axis, coordinate) in vertex.iter().enumerate() {
                if !coordinate.is_finite() {
                    errors.push(error(ValidationErrorKind::NonFiniteCoordinate {
                        vertex_index,
                        axis,
                    }));
                }
            }
        }
        for (triangle_index, triangle) in mesh.triangles.iter().enumerate() {
            let mut in_bounds = true;
            for &vertex_index in triangle {
                if vertex_index >= mesh.vertices.len() {
                    in_bounds = false;
                    errors.push(error(ValidationErrorKind::VertexIndexOutOfBounds {
                        triangle_index,
                        vertex_index,
                    }));
                }
            }
            if in_bounds && is_degenerate(mesh, *triangle) {
                errors.push(error(ValidationErrorKind::DegenerateTriangle {
                    triangle_index,
                }));
            }
        }
    }
    errors
}

fn is_xml_character(character: char) -> bool {
    matches!(character, '\u{9}' | '\u{A}' | '\u{D}')
        || matches!(character as u32, 0x20..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x10FFFF)
}

fn is_degenerate(mesh: &IndexedTriangleMesh, [a, b, c]: [usize; 3]) -> bool {
    if a == b || a == c || b == c {
        return true;
    }
    let a = mesh.vertices[a];
    let b = mesh.vertices[b];
    let c = mesh.vertices[c];
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let cross = [
        ab[1] * ac[2] - ab[2] * ac[1],
        ab[2] * ac[0] - ab[0] * ac[2],
        ab[0] * ac[1] - ab[1] * ac[0],
    ];
    cross == [0.0, 0.0, 0.0]
}

fn escape_xml(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\t' => escaped.push_str("&#x9;"),
            '\n' => escaped.push_str("&#xA;"),
            '\r' => escaped.push_str("&#xD;"),
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(character),
        }
    }
    escaped
}
