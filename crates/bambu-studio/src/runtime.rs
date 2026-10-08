// Copyright (C) 2026 Slicer Project Generators contributors
// AGPL-3.0-only; authored 2026-10-08.
use crate::{
    GeometryRole, NamedObject, ProjectGeometry, cache_key, generator_protocol::*, geometry,
    settings::Settings,
};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub const DIALECT_IDENTITY: &str = "bambu-studio-2.7.1.62-3mf-v1";
pub const INPUT_KIND_IDENTITY: &str = "raw-geometry-3mf-v1";
pub const INPUT_SCHEMA_IDENTITY: &str = "raw-geometry-3mf-core-profile-v1";
pub const NORMALIZATION_IDENTITY: &str = "bambu-3mf-realize-meters-place-once-v1";
pub const VALIDATION_IDENTITY: &str = "bambu-3mf-semantic-readback-v1";
pub const OUTPUT_MEDIA_TYPE: &str = "model/3mf";
pub const MAX_TOTAL_RAW_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_OUTPUT_BYTES: usize = 32 * 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Package {
    pub build_identity: String,
    pub binary_identity: String,
    pub provenance_set_identity: String,
}

impl Package {
    pub fn identity(&self) -> Result<String> {
        cache_key::hash_json("bambu-generator-package-v1", self)
    }
    pub fn bindings(&self, settings_identity: String) -> Result<IdentityBindings> {
        let bindings = IdentityBindings {
            package_identity: self.identity()?,
            build_identity: self.build_identity.clone(),
            binary_identity: self.binary_identity.clone(),
            dialect_identity: DIALECT_IDENTITY.into(),
            capability_identities: vec![
                "bambu-studio.named-objects@1".into(),
                "bambu-studio.support-blocking-volumes@1".into(),
            ],
            input_kind_identity: INPUT_KIND_IDENTITY.into(),
            input_schema_identity: INPUT_SCHEMA_IDENTITY.into(),
            settings_identity,
            settings_schema_identity: crate::settings::schema_identity()?,
            provenance_set_identity: self.provenance_set_identity.clone(),
            normalization_identity: NORMALIZATION_IDENTITY.into(),
            validation_identity: VALIDATION_IDENTITY.into(),
        };
        bindings.validate("package")?;
        Ok(bindings)
    }
    pub fn load(executable: &Path) -> Result<Self> {
        let metadata = File::open(
            executable
                .parent()
                .context("executable parent")?
                .join("package.json"),
        )?;
        let mut bytes = Vec::new();
        metadata.take(4097).read_to_end(&mut bytes)?;
        ensure!(bytes.len() <= 4096, "package metadata byte limit");
        let package: Self = serde_json::from_slice(&bytes)?;
        package.bindings("per-invocation".into())?;
        let mut file = File::open(executable)?;
        use sha2::{Digest, Sha256};
        let mut hash = Sha256::new();
        let mut buffer = [0u8; 65536];
        loop {
            let length = file.read(&mut buffer)?;
            if length == 0 {
                break;
            }
            hash.update(&buffer[..length]);
        }
        let identity: String = hash.finalize().iter().map(|b| format!("{b:02x}")).collect();
        ensure!(
            identity == package.binary_identity,
            "package binary identity mismatch"
        );
        Ok(package)
    }
}

#[derive(Debug)]
struct Failure {
    category: ErrorCategory,
    code: &'static str,
}
fn fail(category: ErrorCategory, code: &'static str) -> Failure {
    Failure { category, code }
}

fn checked_path(root: &Path, relative: &str, existing: bool) -> Result<PathBuf> {
    crate::generator_protocol::validate_relative_path(relative, "path")?;
    let mut path = root.to_path_buf();
    let parts: Vec<_> = relative.split('/').collect();
    for (index, part) in parts.iter().enumerate() {
        path.push(part);
        let last = index + 1 == parts.len();
        match fs::symlink_metadata(&path) {
            Ok(meta) => {
                ensure!(!meta.file_type().is_symlink(), "symlink path");
                ensure!(
                    if last {
                        existing && meta.is_file()
                    } else {
                        meta.is_dir()
                    },
                    "unexpected file type or existing output"
                );
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound && !existing => {
                if !last {
                    fs::create_dir(&path)?;
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(path)
}

fn read_bounded(root: &Path, path: &str, limit: usize) -> Result<Vec<u8>> {
    let file = File::open(checked_path(root, path, true)?)?;
    ensure!(file.metadata()?.len() <= limit as u64, "input byte limit");
    let mut bytes = Vec::new();
    file.take((limit + 1) as u64).read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= limit, "input byte limit");
    Ok(bytes)
}

fn read_content(root: &Path, content: &FileContent, limit: usize) -> Result<Vec<u8>> {
    ensure!(content.byte_length <= limit as u64, "declared byte limit");
    let bytes = read_bounded(root, &content.path, limit)?;
    ensure!(
        bytes.len() as u64 == content.byte_length
            && cache_key::hex_sha256(&bytes) == content.sha256,
        "retained bytes mismatch"
    );
    Ok(bytes)
}

pub fn failure_result(
    code: &'static str,
    category: ErrorCategory,
    request: Option<&GeneratorRequest>,
) -> GeneratorResult {
    GeneratorResult {
        document_type: ResultDocumentType::GeneratorResult,
        protocol_version: 1,
        status: ResultStatus::Failure,
        invocation_identity: request.map(|r| r.invocation_identity.clone()),
        reported_identities: None,
        output: None,
        diagnostics: Vec::new(),
        errors: vec![StructuredError {
            category,
            code: code.into(),
            message: code.replace('_', " "),
            instance_path: None,
            context: Vec::new(),
        }],
    }
}

fn prepare(
    root: &Path,
    request: &GeneratorRequest,
    package: &Package,
) -> std::result::Result<Vec<u8>, Failure> {
    let unsupported = || {
        fail(
            ErrorCategory::UnsupportedRequest,
            "unsupported_generator_identity",
        )
    };
    let actual = package
        .bindings(request.settings.settings_identity.clone())
        .map_err(|_| unsupported())?;
    if actual != request.expected_identities || request.output.media_type != OUTPUT_MEDIA_TYPE {
        return Err(unsupported());
    }
    let manifest_bytes = read_bounded(root, &request.input_manifest.path, MAX_INPUT_MANIFEST_BYTES)
        .map_err(|_| fail(ErrorCategory::InvalidInput, "manifest_file_invalid"))?;
    let manifest = parse_input_manifest(&manifest_bytes)
        .map_err(|_| fail(ErrorCategory::InvalidInput, "manifest_invalid"))?;
    request
        .validate_with_manifest(&manifest)
        .map_err(|_| fail(ErrorCategory::InvalidInput, "manifest_binding_invalid"))?;
    let settings_bytes = read_content(
        root,
        &request.settings.content,
        crate::settings::MAX_SETTINGS_BYTES,
    )
    .map_err(|_| fail(ErrorCategory::InvalidInput, "settings_file_invalid"))?;
    if request.settings.content.media_type != "application/json" {
        return Err(fail(
            ErrorCategory::InvalidInput,
            "settings_media_type_invalid",
        ));
    }
    let settings = Settings::parse(&settings_bytes)
        .map_err(|_| fail(ErrorCategory::InvalidInput, "settings_invalid"))?;
    settings
        .validate_manifest(&manifest)
        .map_err(|_| fail(ErrorCategory::InvalidInput, "settings_context_invalid"))?;
    if settings
        .identity()
        .map_err(|_| fail(ErrorCategory::InvalidInput, "settings_identity_invalid"))?
        != request.settings.settings_identity
    {
        return Err(fail(
            ErrorCategory::InvalidInput,
            "settings_identity_invalid",
        ));
    }
    let mut entries = Vec::new();
    let mut total_raw = 0usize;
    let mut decoded_budget = MAX_TOTAL_RAW_BYTES;
    let mut total_vertices = 0usize;
    let mut total_triangles = 0usize;
    for (index, (object, placement)) in manifest
        .objects
        .iter()
        .zip(&settings.placements)
        .enumerate()
    {
        // Source metadata in protocol v1 is opaque and never used for realization.
        if object.retained_content.media_type != OUTPUT_MEDIA_TYPE {
            return Err(fail(
                ErrorCategory::UnsupportedRequest,
                "unsupported_input_media_type",
            ));
        }
        total_raw = total_raw
            .checked_add(
                usize::try_from(object.retained_content.byte_length)
                    .map_err(|_| fail(ErrorCategory::InvalidInput, "raw_input_limit"))?,
            )
            .ok_or_else(|| fail(ErrorCategory::InvalidInput, "raw_input_limit"))?;
        if total_raw > MAX_TOTAL_RAW_BYTES {
            return Err(fail(ErrorCategory::InvalidInput, "raw_input_limit"));
        }
        let bytes = read_content(root, &object.retained_content, geometry::MAX_RAW_BYTES)
            .map_err(|_| fail(ErrorCategory::InvalidInput, "raw_input_file_invalid"))?;
        let mesh = geometry::realize_with_budget(&bytes, &placement.matrix, &mut decoded_budget)
            .map_err(|error| {
                if error
                    .downcast_ref::<geometry::UnsupportedGeometry>()
                    .is_some()
                {
                    fail(
                        ErrorCategory::UnsupportedRequest,
                        "unsupported_geometry_profile",
                    )
                } else {
                    fail(ErrorCategory::InvalidInput, "geometry_realization_failed")
                }
            })?;
        let mut append = |entry: ProjectGeometry| -> std::result::Result<(), Failure> {
            total_vertices += entry.mesh.as_ref().unwrap().vertices.len();
            total_triangles += entry.mesh.as_ref().unwrap().triangles.len();
            if total_vertices > geometry::MAX_VERTICES || total_triangles > geometry::MAX_TRIANGLES
            {
                return Err(fail(ErrorCategory::InvalidInput, "project_geometry_limit"));
            }
            entries.push(entry);
            Ok(())
        };
        if object.role == InputRole::RawGeometry {
            append(ProjectGeometry {
                identity: Some(format!("manifest-{index}")),
                name: object.display_name.clone(),
                mesh: Some(mesh),
                role: Some(GeometryRole::PrintableModel),
            })?;
        } else {
            let blocker = settings
                .blockers
                .iter()
                .find(|b| b.object_identity == object.object_identity)
                .unwrap();
            for (target_index, target) in blocker.targets.iter().enumerate() {
                let target_position = manifest
                    .objects
                    .iter()
                    .position(|o| &o.object_identity == target)
                    .unwrap();
                append(ProjectGeometry {
                    identity: Some(format!("manifest-{index}-target-{target_index}")),
                    name: object.display_name.clone(),
                    mesh: Some(mesh.clone()),
                    role: Some(GeometryRole::SupportBlocker {
                        target_identity: Some(format!("manifest-{target_position}")),
                    }),
                })?;
            }
        }
    }
    // Apply the stricter target numeric profile to both dispatches.
    if !crate::validate_support_blocking_volumes(&entries).is_empty() {
        return Err(fail(
            ErrorCategory::GenerationFailed,
            "target_geometry_invalid",
        ));
    }
    let named = settings.blockers.is_empty() && entries.iter().all(|e| e.name.is_some());
    let bytes = if named {
        let objects: Vec<_> = entries
            .iter()
            .map(|e| NamedObject {
                identity: e.identity.clone(),
                name: e.name.clone(),
                mesh: e.mesh.clone(),
            })
            .collect();
        crate::generate_named_objects(&objects)
            .map_err(|_| fail(ErrorCategory::GenerationFailed, "named_generation_failed"))?
            .bytes
    } else {
        crate::generate_support_blocking_volumes(&entries)
            .map_err(|_| fail(ErrorCategory::GenerationFailed, "blocker_generation_failed"))?
            .bytes
    };
    if bytes.len() > MAX_OUTPUT_BYTES || bytes.len() as u64 > request.output.max_byte_length {
        return Err(fail(ErrorCategory::GenerationFailed, "output_byte_limit"));
    }
    self_validate(&bytes, &entries, named)
        .map_err(|_| fail(ErrorCategory::GenerationFailed, "self_validation_failed"))?;
    Ok(bytes)
}

/// Reopens completed bytes and checks exact target topology against the request.
pub fn self_validate(bytes: &[u8], entries: &[ProjectGeometry], named: bool) -> Result<()> {
    ensure!(
        crate::validate_support_blocking_volumes(entries).is_empty(),
        "invalid expected target geometry"
    );
    ensure!(
        !named
            || entries
                .iter()
                .all(|e| matches!(e.role, Some(GeometryRole::PrintableModel)) && e.name.is_some()),
        "invalid named dispatch"
    );
    let files = geometry::archive(bytes)?;
    ensure!(
        files.len() == if named { 3 } else { 4 }
            && files.contains_key("[Content_Types].xml")
            && files.contains_key("_rels/.rels"),
        "target archive inventory mismatch"
    );
    ensure!(
        geometry::primary_model(&files)? == "3D/3dmodel.model",
        "target primary relationship mismatch"
    );
    let doc = geometry::xml(
        files
            .get("3D/3dmodel.model")
            .context("missing target model")?,
    )?;
    let root = doc.root_element();
    ensure!(
        root.tag_name().name() == "model"
            && root.tag_name().namespace() == Some(geometry::CORE)
            && root.attribute("unit") == Some("millimeter"),
        "invalid target model"
    );
    geometry::attrs(root, &["unit"])?;
    let sections = geometry::children(root, &["resources", "build"], geometry::CORE)?;
    for section in &sections {
        geometry::attrs(*section, &[])?;
    }
    ensure!(
        sections.len() == 2
            && sections[0].tag_name().name() == "resources"
            && sections[1].tag_name().name() == "build",
        "target sections mismatch"
    );
    let objects = geometry::children(sections[0], &["object"], geometry::CORE)?;
    let printable: Vec<_> = entries
        .iter()
        .enumerate()
        .filter(|(_, e)| matches!(e.role, Some(GeometryRole::PrintableModel)))
        .collect();
    ensure!(
        objects.len() == entries.len() + if named { 0 } else { printable.len() },
        "target object cardinality mismatch"
    );
    let mut containing = HashMap::new();
    for (model_position, (index, entry)) in printable.iter().enumerate() {
        containing.insert(
            entry.identity.as_deref().unwrap(),
            if named {
                *index + 1
            } else {
                entries.len() + model_position + 1
            },
        );
    }
    for (index, entry) in entries.iter().enumerate() {
        let object = objects[index];
        geometry::attrs(object, &["id", "type", "name"])?;
        ensure!(
            geometry::required(object, "id")?.parse::<usize>()? == index + 1,
            "target ID/order mismatch"
        );
        let expected_type = if matches!(entry.role, Some(GeometryRole::PrintableModel)) {
            "model"
        } else {
            "other"
        };
        ensure!(
            object.attribute("type") == Some(expected_type),
            "target object role mismatch"
        );
        ensure!(
            object.attribute("name") == if named { entry.name.as_deref() } else { None },
            "target name mismatch"
        );
        let content = geometry::children(object, &["mesh"], geometry::CORE)?;
        ensure!(
            content.len() == 1
                && &geometry::parse_mesh(content[0])? == entry.mesh.as_ref().unwrap(),
            "target mesh mismatch"
        );
    }
    if !named {
        for (position, (_, entry)) in printable.iter().enumerate() {
            let object = objects[entries.len() + position];
            geometry::attrs(object, &["id", "type"])?;
            ensure!(
                geometry::required(object, "id")?.parse::<usize>()?
                    == containing[entry.identity.as_deref().unwrap()]
                    && object.attribute("type") == Some("model"),
                "containing object mismatch"
            );
            let child = geometry::children(object, &["components"], geometry::CORE)?;
            ensure!(child.len() == 1, "target component structure");
            geometry::attrs(child[0], &[])?;
            let components = geometry::children(child[0], &["component"], geometry::CORE)?;
            let expected = members(entries, entry.identity.as_deref().unwrap());
            ensure!(
                components.len() == expected.len(),
                "target component count mismatch"
            );
            for (component, index) in components.iter().zip(expected) {
                geometry::attrs(*component, &["objectid", "transform"])?;
                geometry::children(*component, &[], geometry::CORE)?;
                ensure!(
                    geometry::required(*component, "objectid")?.parse::<usize>()? == index + 1
                        && geometry::transform(component.attribute("transform"))?
                            == geometry::IDENTITY,
                    "target component association/placement mismatch"
                );
            }
        }
        let settings = geometry::xml(
            files
                .get("Metadata/model_settings.config")
                .context("missing target settings")?,
        )?;
        let config = settings.root_element();
        geometry::attrs(config, &[])?;
        ensure!(
            config.tag_name().name() == "config" && config.tag_name().namespace().is_none(),
            "invalid target settings"
        );
        let settings_objects = settings_children(config, &["object"])?;
        ensure!(
            settings_objects.len() == printable.len(),
            "target settings object count"
        );
        for (node, (_, entry)) in settings_objects.iter().zip(&printable) {
            geometry::attrs(*node, &["id"])?;
            ensure!(
                node.tag_name().name() == "object"
                    && geometry::required(*node, "id")?.parse::<usize>()?
                        == containing[entry.identity.as_deref().unwrap()],
                "target settings association"
            );
            validate_name(*node, entry.name.as_deref())?;
            let parts: Vec<_> = settings_children(*node, &["metadata", "part"])?
                .into_iter()
                .filter(|n| n.tag_name().name() == "part")
                .collect();
            let expected = members(entries, entry.identity.as_deref().unwrap());
            ensure!(parts.len() == expected.len(), "target part count mismatch");
            for (part, index) in parts.iter().zip(expected) {
                geometry::attrs(*part, &["id", "subtype"])?;
                let subtype = if matches!(entries[index].role, Some(GeometryRole::PrintableModel)) {
                    "normal_part"
                } else {
                    "support_blocker"
                };
                ensure!(
                    geometry::required(*part, "id")?.parse::<usize>()? == index + 1
                        && part.attribute("subtype") == Some(subtype),
                    "target part role/association mismatch"
                );
                validate_name(*part, entries[index].name.as_deref())?;
            }
        }
    }
    let items = geometry::children(sections[1], &["item"], geometry::CORE)?;
    ensure!(
        items.len() == printable.len(),
        "target build count mismatch"
    );
    for (item, (_, entry)) in items.iter().zip(&printable) {
        geometry::attrs(*item, &["objectid", "transform", "printable"])?;
        geometry::children(*item, &[], geometry::CORE)?;
        ensure!(
            geometry::required(*item, "objectid")?.parse::<usize>()?
                == containing[entry.identity.as_deref().unwrap()]
                && geometry::transform(item.attribute("transform"))? == geometry::IDENTITY,
            "target build membership/placement mismatch"
        );
        ensure!(
            item.attribute("printable").is_none_or(|p| p == "1"),
            "target printable mismatch"
        );
    }
    Ok(())
}

fn members(entries: &[ProjectGeometry], identity: &str) -> Vec<usize> {
    entries.iter().enumerate().filter(|(_,entry)| entry.identity.as_deref() == Some(identity) || matches!(&entry.role,Some(GeometryRole::SupportBlocker {target_identity:Some(target)}) if target == identity)).map(|(i,_)|i).collect()
}

fn validate_name(node: roxmltree::Node<'_, '_>, expected: Option<&str>) -> Result<()> {
    let mut names = Vec::new();
    let allowed: &[&str] = if node.tag_name().name() == "object" {
        &["metadata", "part"]
    } else {
        &["metadata"]
    };
    for child in settings_children(node, allowed)? {
        ensure!(
            child.tag_name().namespace().is_none(),
            "target settings namespace"
        );
        if child.tag_name().name() == "part" {
            continue;
        }
        geometry::attrs(child, &["key", "value"])?;
        settings_children(child, &[])?;
        ensure!(
            child.tag_name().name() == "metadata" && child.attribute("key") == Some("name"),
            "unexpected target metadata"
        );
        names.push(child.attribute("value").context("target name missing")?);
    }
    ensure!(
        names == expected.into_iter().collect::<Vec<_>>(),
        "target display name mismatch"
    );
    Ok(())
}

fn settings_children<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
    names: &[&str],
) -> Result<Vec<roxmltree::Node<'a, 'input>>> {
    let mut nodes = Vec::new();
    for child in node.children() {
        if child.is_element() {
            ensure!(
                child.tag_name().namespace().is_none() && names.contains(&child.tag_name().name()),
                "unexpected settings child"
            );
            nodes.push(child);
        } else if child.is_text() {
            ensure!(
                child.text().unwrap_or("").trim().is_empty(),
                "unexpected settings text"
            );
        }
    }
    Ok(nodes)
}

fn atomic_file(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_file_name(format!(
        "{}.generator-tmp",
        path.file_name()
            .context("output filename")?
            .to_str()
            .context("output filename encoding")?
    ));
    ensure!(
        fs::symlink_metadata(&temporary).is_err(),
        "temporary output already exists"
    );
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    let mut renamed = false;
    let outcome = (|| {
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        ensure!(fs::symlink_metadata(path).is_err(), "output already exists");
        fs::rename(&temporary, path)?;
        renamed = true;
        File::open(path.parent().context("output parent")?)?.sync_all()?;
        Ok(())
    })();
    if outcome.is_err() {
        let _ = fs::remove_file(&temporary);
        if renamed {
            let _ = fs::remove_file(path);
        }
    }
    outcome
}

/// Implements the trusted runner's candidate-then-result commit protocol.
pub fn invoke(
    root: &Path,
    request_path: &str,
    result_path: &str,
    package: &Package,
) -> Result<bool> {
    ensure!(
        request_path != result_path
            && !result_path.starts_with("inputs/")
            && !result_path.starts_with("outputs/"),
        "invalid control paths"
    );
    let result_destination = checked_path(root, result_path, false)?;
    let request = read_bounded(root, request_path, MAX_REQUEST_BYTES)
        .and_then(|bytes| Ok(parse_request(&bytes)?));
    let mut candidate_path = None;
    let result = match request {
        Err(_) => failure_result("malformed_request", ErrorCategory::MalformedRequest, None),
        Ok(ref request) => match checked_path(root, &request.output.path, false) {
            Err(_) => failure_result(
                "output_path_invalid",
                ErrorCategory::GenerationFailed,
                Some(request),
            ),
            Ok(path) => match prepare(root, request, package) {
                Err(failure) => failure_result(failure.code, failure.category, Some(request)),
                Ok(bytes) => {
                    let result = GeneratorResult {
                        document_type: ResultDocumentType::GeneratorResult,
                        protocol_version: 1,
                        status: ResultStatus::Success,
                        invocation_identity: Some(request.invocation_identity.clone()),
                        reported_identities: Some(
                            package.bindings(request.settings.settings_identity.clone())?,
                        ),
                        output: Some(GeneratedOutput {
                            output_identity: request.output.output_identity.clone(),
                            role: request.output.role,
                            path: request.output.path.clone(),
                            media_type: request.output.media_type.clone(),
                            byte_length: bytes.len() as u64,
                            sha256: cache_key::hex_sha256(&bytes),
                        }),
                        diagnostics: Vec::new(),
                        errors: Vec::new(),
                    };
                    result.validate_against_request(request)?;
                    if atomic_file(&path, &bytes).is_err() {
                        let _ = fs::remove_file(&path);
                        failure_result(
                            "candidate_write_failed",
                            ErrorCategory::GenerationFailed,
                            Some(request),
                        )
                    } else {
                        candidate_path = Some(path);
                        result
                    }
                }
            },
        },
    };
    result.validate()?;
    let bytes = serde_json::to_vec(&result)?;
    ensure!(bytes.len() <= MAX_RESULT_BYTES, "result byte limit");
    if let Err(error) = atomic_file(&result_destination, &bytes) {
        if let Some(path) = candidate_path {
            let _ = fs::remove_file(path);
        }
        return Err(error);
    }
    Ok(result.status == ResultStatus::Success)
}

pub fn write_installation_failure(
    root: &Path,
    result_path: &str,
    result: &GeneratorResult,
) -> Result<()> {
    ensure!(
        !result_path.starts_with("inputs/") && !result_path.starts_with("outputs/"),
        "invalid result path"
    );
    result.validate()?;
    atomic_file(
        &checked_path(root, result_path, false)?,
        &serde_json::to_vec(result)?,
    )
}
