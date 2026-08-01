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
pub const SUPPORT_BLOCKING_VOLUMES_CAPABILITY_ID: &str = "bambu-studio.support-blocking-volumes";
pub const SUPPORT_BLOCKING_VOLUMES_CAPABILITY_REVISION: u32 = 1;
pub const VALIDATED_TARGET_VERSION: &str = "2.7.1.62";

const CONTENT_TYPES_PATH: &str = "[Content_Types].xml";
const RELATIONSHIPS_PATH: &str = "_rels/.rels";
const MODEL_PATH: &str = "3D/3dmodel.model";
const MODEL_SETTINGS_PATH: &str = "Metadata/model_settings.config";

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

#[derive(Clone, Debug, PartialEq)]
pub enum GeometryRole {
    PrintableModel,
    SupportBlocker { target_identity: Option<String> },
    Unsupported(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectGeometry {
    pub identity: Option<String>,
    pub name: Option<String>,
    pub mesh: Option<IndexedTriangleMesh>,
    pub role: Option<GeometryRole>,
}

impl ProjectGeometry {
    #[must_use]
    pub fn printable(identity: impl Into<String>, mesh: IndexedTriangleMesh) -> Self {
        Self {
            identity: Some(identity.into()),
            name: None,
            mesh: Some(mesh),
            role: Some(GeometryRole::PrintableModel),
        }
    }

    #[must_use]
    pub fn support_blocker(
        identity: impl Into<String>,
        target_identity: impl Into<String>,
        mesh: IndexedTriangleMesh,
    ) -> Self {
        Self {
            identity: Some(identity.into()),
            name: None,
            mesh: Some(mesh),
            role: Some(GeometryRole::SupportBlocker {
                target_identity: Some(target_identity.into()),
            }),
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
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
pub struct VolumeMapping {
    pub identity: String,
    pub containing_object_id: u32,
    pub volume_object_id: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedSupportBlockingProject {
    pub bytes: Vec<u8>,
    pub volume_mappings: Vec<VolumeMapping>,
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
    TargetIndexOutOfRange {
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
    MissingRole,
    UnsupportedRole {
        role: String,
    },
    ConflictingRoleMapping {
        first_object_index: usize,
    },
    NoPrintableModels,
    MissingBlockerTarget,
    EmptyBlockerTarget,
    UnknownBlockerTarget,
    AmbiguousBlockerTarget,
    BlockerTargetNotPrintable,
    CoordinateOutOfTargetRange {
        vertex_index: usize,
        axis: usize,
    },
    DegenerateTriangleInTarget {
        triangle_index: usize,
    },
    TooManyResourceObjects,
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

/// Generates a deterministic Bambu project with explicit support-blocker roles.
pub fn generate_support_blocking_volumes(
    entries: &[ProjectGeometry],
) -> Result<GeneratedSupportBlockingProject, GenerationError> {
    let errors = validate_support_blocking_volumes(entries);
    if !errors.is_empty() {
        return Err(GenerationError::Validation(errors));
    }

    let printable_count = entries
        .iter()
        .filter(|entry| matches!(entry.role, Some(GeometryRole::PrintableModel)))
        .count();
    let mut containing_ids = HashMap::with_capacity(printable_count);
    for (model_index, entry) in entries
        .iter()
        .filter(|entry| matches!(entry.role, Some(GeometryRole::PrintableModel)))
        .enumerate()
    {
        let identity = entry.identity.as_deref().expect("validated identity");
        let containing_object_id =
            u32::try_from(entries.len() + model_index + 1).expect("validated resource count");
        containing_ids.insert(identity, containing_object_id);
    }

    let mut model = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <model unit=\"millimeter\" xml:lang=\"en-US\" xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\">\n\
         <resources>\n",
    );
    let mut mappings = Vec::with_capacity(entries.len());

    for (index, entry) in entries.iter().enumerate() {
        let volume_object_id = u32::try_from(index + 1).expect("validated resource count");
        let identity = entry.identity.as_deref().expect("validated identity");
        let mesh = entry.mesh.as_ref().expect("validated geometry");
        let object_type = match entry.role.as_ref().expect("validated role") {
            GeometryRole::PrintableModel => "model",
            GeometryRole::SupportBlocker { .. } => "other",
            GeometryRole::Unsupported(_) => unreachable!("validated role"),
        };
        write_mesh_object(&mut model, volume_object_id, object_type, mesh);

        let containing_object_id = match entry.role.as_ref().expect("validated role") {
            GeometryRole::PrintableModel => containing_ids[identity],
            GeometryRole::SupportBlocker { target_identity } => {
                containing_ids[target_identity.as_deref().expect("validated target")]
            }
            GeometryRole::Unsupported(_) => unreachable!("validated role"),
        };
        mappings.push(VolumeMapping {
            identity: identity.to_owned(),
            containing_object_id,
            volume_object_id,
        });
    }

    for entry in entries
        .iter()
        .filter(|entry| matches!(entry.role, Some(GeometryRole::PrintableModel)))
    {
        let identity = entry.identity.as_deref().expect("validated identity");
        let containing_object_id = containing_ids[identity];
        write!(
            model,
            "<object id=\"{containing_object_id}\" type=\"model\"><components>"
        )
        .expect("writing to String cannot fail");
        for mapping in mappings
            .iter()
            .filter(|mapping| mapping.containing_object_id == containing_object_id)
        {
            write!(
                model,
                "<component objectid=\"{}\" transform=\"1 0 0 0 1 0 0 0 1 0 0 0\"/>",
                mapping.volume_object_id
            )
            .expect("writing to String cannot fail");
        }
        model.push_str("</components></object>\n");
    }

    model.push_str("</resources><build>");
    for entry in entries
        .iter()
        .filter(|entry| matches!(entry.role, Some(GeometryRole::PrintableModel)))
    {
        let identity = entry.identity.as_deref().expect("validated identity");
        write!(
            model,
            "<item objectid=\"{}\" transform=\"1 0 0 0 1 0 0 0 1 0 0 0\" printable=\"1\"/>",
            containing_ids[identity]
        )
        .expect("writing to String cannot fail");
    }
    model.push_str("</build></model>\n");

    let mut settings = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<config>\n");
    for entry in entries
        .iter()
        .filter(|entry| matches!(entry.role, Some(GeometryRole::PrintableModel)))
    {
        let identity = entry.identity.as_deref().expect("validated identity");
        let containing_object_id = containing_ids[identity];
        write!(settings, "<object id=\"{containing_object_id}\">")
            .expect("writing to String cannot fail");
        if let Some(name) = entry.name.as_deref() {
            write!(
                settings,
                "<metadata key=\"name\" value=\"{}\"/>",
                escape_xml(name)
            )
            .expect("writing to String cannot fail");
        }
        for mapping in mappings
            .iter()
            .filter(|mapping| mapping.containing_object_id == containing_object_id)
        {
            let volume = entries
                .iter()
                .find(|entry| entry.identity.as_deref() == Some(mapping.identity.as_str()))
                .expect("validated unique identity");
            let subtype = match volume.role.as_ref().expect("validated role") {
                GeometryRole::PrintableModel => "normal_part",
                GeometryRole::SupportBlocker { .. } => "support_blocker",
                GeometryRole::Unsupported(_) => unreachable!("validated role"),
            };
            write!(
                settings,
                "<part id=\"{}\" subtype=\"{subtype}\">",
                mapping.volume_object_id
            )
            .expect("writing to String cannot fail");
            if let Some(name) = volume.name.as_deref() {
                write!(
                    settings,
                    "<metadata key=\"name\" value=\"{}\"/>",
                    escape_xml(name)
                )
                .expect("writing to String cannot fail");
            }
            settings.push_str("</part>");
        }
        settings.push_str("</object>\n");
    }
    settings.push_str("</config>\n");

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
        (MODEL_SETTINGS_PATH, settings.as_str()),
    ] {
        archive.start_file(path, options)?;
        archive.write_all(contents.as_bytes())?;
    }

    Ok(GeneratedSupportBlockingProject {
        bytes: archive.finish()?.into_inner(),
        volume_mappings: mappings,
    })
}

fn write_mesh_object(
    model: &mut String,
    object_id: u32,
    object_type: &str,
    mesh: &IndexedTriangleMesh,
) {
    write!(
        model,
        "<object id=\"{object_id}\" type=\"{object_type}\"><mesh><vertices>"
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
}

fn validate_support_blocking_volumes(entries: &[ProjectGeometry]) -> Vec<ValidationError> {
    if entries.is_empty() {
        return vec![ValidationError {
            object_index: None,
            identity: None,
            kind: ValidationErrorKind::NoObjects,
        }];
    }

    let mut errors = Vec::new();
    let mut identities: HashMap<&str, Vec<usize>> = HashMap::new();
    let printable_count = entries
        .iter()
        .filter(|entry| matches!(entry.role, Some(GeometryRole::PrintableModel)))
        .count();
    if entries
        .len()
        .checked_add(printable_count)
        .is_none_or(|count| i32::try_from(count).is_err())
    {
        errors.push(ValidationError {
            object_index: None,
            identity: None,
            kind: ValidationErrorKind::TooManyResourceObjects,
        });
    }
    if printable_count == 0 {
        errors.push(ValidationError {
            object_index: None,
            identity: None,
            kind: ValidationErrorKind::NoPrintableModels,
        });
    }

    for (index, entry) in entries.iter().enumerate() {
        if let Some(identity) = entry
            .identity
            .as_deref()
            .filter(|identity| !identity.is_empty())
        {
            identities.entry(identity).or_default().push(index);
        }
    }

    for (object_index, entry) in entries.iter().enumerate() {
        let identity = entry.identity.as_deref();
        let error = |kind| ValidationError {
            object_index: Some(object_index),
            identity: identity.map(str::to_owned),
            kind,
        };

        match identity {
            None => errors.push(error(ValidationErrorKind::MissingIdentity)),
            Some("") => errors.push(error(ValidationErrorKind::EmptyIdentity)),
            Some(identity) => {
                let indexes = &identities[identity];
                if indexes[0] != object_index {
                    let first_object_index = indexes[0];
                    let first = &entries[first_object_index];
                    let kind = if first.role != entry.role {
                        ValidationErrorKind::ConflictingRoleMapping { first_object_index }
                    } else if first.name != entry.name || first.mesh != entry.mesh {
                        ValidationErrorKind::ConflictingIdentityMapping { first_object_index }
                    } else {
                        ValidationErrorKind::DuplicateIdentity { first_object_index }
                    };
                    errors.push(error(kind));
                }
            }
        }

        match entry.role.as_ref() {
            None => errors.push(error(ValidationErrorKind::MissingRole)),
            Some(GeometryRole::Unsupported(role)) => {
                errors.push(error(ValidationErrorKind::UnsupportedRole {
                    role: role.clone(),
                }));
            }
            Some(GeometryRole::SupportBlocker { target_identity }) => {
                match target_identity.as_deref() {
                    None => errors.push(error(ValidationErrorKind::MissingBlockerTarget)),
                    Some("") => errors.push(error(ValidationErrorKind::EmptyBlockerTarget)),
                    Some(target) => match identities.get(target) {
                        None => errors.push(error(ValidationErrorKind::UnknownBlockerTarget)),
                        Some(indexes) if indexes.len() > 1 => {
                            errors.push(error(ValidationErrorKind::AmbiguousBlockerTarget));
                        }
                        Some(indexes)
                            if !matches!(
                                entries[indexes[0]].role,
                                Some(GeometryRole::PrintableModel)
                            ) =>
                        {
                            errors.push(error(ValidationErrorKind::BlockerTargetNotPrintable));
                        }
                        Some(_) => {}
                    },
                }
            }
            Some(GeometryRole::PrintableModel) => {}
        }

        if let Some(name) = entry.name.as_deref() {
            if name.is_empty() {
                errors.push(error(ValidationErrorKind::EmptyName));
            } else {
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

        let Some(mesh) = &entry.mesh else {
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
                } else if !(*coordinate as f32).is_finite() {
                    errors.push(error(ValidationErrorKind::CoordinateOutOfTargetRange {
                        vertex_index,
                        axis,
                    }));
                }
            }
        }
        for (triangle_index, triangle) in mesh.triangles.iter().enumerate() {
            let mut in_bounds = true;
            for &vertex_index in triangle {
                if i32::try_from(vertex_index).is_err() {
                    errors.push(error(ValidationErrorKind::TargetIndexOutOfRange {
                        triangle_index,
                        vertex_index,
                    }));
                }
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
            } else if in_bounds && is_degenerate_in_target(mesh, *triangle) {
                errors.push(error(ValidationErrorKind::DegenerateTriangleInTarget {
                    triangle_index,
                }));
            }
        }
    }
    errors
}

fn is_degenerate_in_target(mesh: &IndexedTriangleMesh, [a, b, c]: [usize; 3]) -> bool {
    let a = mesh.vertices[a].map(|value| value as f32);
    let b = mesh.vertices[b].map(|value| value as f32);
    let c = mesh.vertices[c].map(|value| value as f32);
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let cross = [
        ab[1] * ac[2] - ab[2] * ac[1],
        ab[2] * ac[0] - ab[0] * ac[2],
        ab[0] * ac[1] - ab[1] * ac[0],
    ];
    cross == [0.0, 0.0, 0.0]
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
