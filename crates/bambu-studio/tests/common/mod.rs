use slicer_project_generator_bambu_studio::{
    cache_key,
    generator_protocol::*,
    geometry,
    runtime::{self, Package},
    settings::{Blocker, Placement, Settings},
};
use std::{
    fs,
    io::{Cursor, Write},
    path::{Path, PathBuf},
};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

pub struct Temp(pub PathBuf);
impl Temp {
    pub fn new() -> Self {
        let base = std::env::temp_dir().join("agents");
        fs::create_dir_all(&base).unwrap();
        let path = base.join(format!(
            "bambu-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

pub fn repack(files: impl IntoIterator<Item = (String, Vec<u8>)>, deflate: bool) -> Vec<u8> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, bytes) in files {
        zip.start_file(
            name,
            SimpleFileOptions::default().compression_method(if deflate {
                CompressionMethod::Deflated
            } else {
                CompressionMethod::Stored
            }),
        )
        .unwrap();
        zip.write_all(&bytes).unwrap();
    }
    zip.finish().unwrap().into_inner()
}

pub fn raw(model: &str, deflate: bool) -> Vec<u8> {
    repack([
        ("[Content_Types].xml".into(),br#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/></Types>"#.to_vec()),
        ("_rels/.rels".into(),br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Target="/3D/3dmodel.model" Id="rel-1" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/></Relationships>"#.to_vec()),
        ("3D/3dmodel.model".into(),model.as_bytes().to_vec()),
    ],deflate)
}

pub fn mesh_xml() -> &'static str {
    r#"<mesh><vertices><vertex x="0" y="0" z="0"/><vertex x="1" y="0" z="0"/><vertex x="0" y="1" z="0"/><vertex x="0" y="0" z="1"/></vertices><triangles><triangle v1="0" v2="2" v3="1"/><triangle v1="0" v2="1" v3="3"/><triangle v1="0" v2="3" v3="2"/><triangle v1="1" v2="2" v3="3"/></triangles></mesh>"#
}

pub fn model(unit: &str) -> String {
    format!(
        r#"<model unit="{unit}" xmlns="{}"><resources><object id="1">{}</object></resources><build><item objectid="1"/></build></model>"#,
        geometry::CORE,
        mesh_xml()
    )
}

pub fn package() -> Package {
    Package {
        build_identity: "test-build-v1".into(),
        binary_identity: "test-binary-v1".into(),
        provenance_set_identity: "test-provenance-v1".into(),
    }
}

pub fn fixture(
    root: &Path,
    package: &Package,
    blockers: bool,
    assembly: bool,
) -> (GeneratorRequest, InputManifest, Settings) {
    fs::create_dir_all(root.join("inputs")).unwrap();
    let bytes = raw(&model("meter"), true);
    let count = if blockers { 4 } else { 2 };
    let mut objects = Vec::new();
    let mut placements = Vec::new();
    for index in 0..count {
        let path = format!("inputs/geometry-{index}.3mf");
        fs::write(root.join(&path), &bytes).unwrap();
        let object: InputObject = serde_json::from_value(serde_json::json!({
            "objectIdentity":format!("object-{index}"),"role":if index < 2 {"rawGeometry"} else {"auxiliaryGeometry"},
            "retainedContent":{"contentIdentity":"equal-content","path":path,"sha256":cache_key::hex_sha256(&bytes),"byteLength":bytes.len(),"mediaType":"model/3mf","detectedKindIdentity":"geometry-3mf"},
            "mapping":{"status":"proven","evidence":{"classification":"immutable-leaf","evidenceIdentity":"synthetic-evidence"}},"displayName":"Duplicate & name"
        })).unwrap();
        objects.push(object);
        let mut matrix = geometry::IDENTITY;
        if assembly {
            matrix[3] = index as f64 * 2.;
            matrix[7] = index as f64 * 0.1;
        }
        placements.push(Placement {
            object_identity: format!("object-{index}"),
            matrix,
        });
    }
    let mut manifest: InputManifest = serde_json::from_value(serde_json::json!({
        "documentType":"inputManifest","protocolVersion":1,"manifestVersion":1,"manifestIdentity":"", "inputSetIdentity":"",
        "requirementsIdentity":"synthetic-requirements","sourceIdentity":"opaque-source","configurationIdentity":"opaque-configured",
        "export":{"kindIdentity":runtime::INPUT_KIND_IDENTITY,"schemaIdentity":runtime::INPUT_SCHEMA_IDENTITY,"groupingPolicy":"individual","observationStatus":"proven","observationEvidenceIdentity":"synthetic-observation"},
        "decision":{"status":"available"},"objects":objects
    })).unwrap();
    let settings = Settings {
        schema_version: 2,
        placements,
        blockers: if blockers {
            vec![
                Blocker {
                    object_identity: "object-2".into(),
                    targets: vec!["object-0".into(), "object-1".into()],
                },
                Blocker {
                    object_identity: "object-3".into(),
                    targets: vec!["object-0".into()],
                },
            ]
        } else {
            Vec::new()
        },
    };
    let settings_bytes = serde_json::to_vec(&settings).unwrap();
    let mut request: GeneratorRequest = serde_json::from_value(serde_json::json!({
        "documentType":"generatorRequest","protocolVersion":1,"invocationIdentity":"",
        "expectedIdentities":package.bindings(settings.identity().unwrap()).unwrap(),
        "inputManifest":{"path":"inputs/manifest.json","manifestIdentity":"","inputSetIdentity":""},
        "settings":{"settingsIdentity":settings.identity().unwrap(),"schemaIdentity":slicer_project_generator_bambu_studio::settings::schema_identity().unwrap(),
            "content":{"contentIdentity":"settings-content","path":"inputs/settings.json","sha256":cache_key::hex_sha256(&settings_bytes),"byteLength":settings_bytes.len(),"mediaType":"application/json","detectedKindIdentity":"settings-json"}},
        "output":{"outputIdentity":"output-project","role":"generatedProject","path":"outputs/project.3mf","mediaType":"model/3mf","maxByteLength":runtime::MAX_OUTPUT_BYTES}
    })).unwrap();
    bind(root, &mut request, &mut manifest, &settings);
    (request, manifest, settings)
}

pub fn bind(
    root: &Path,
    request: &mut GeneratorRequest,
    manifest: &mut InputManifest,
    settings: &Settings,
) {
    let settings_bytes = serde_json::to_vec(settings).unwrap();
    request.settings.content.sha256 = cache_key::hex_sha256(&settings_bytes);
    request.settings.content.byte_length = settings_bytes.len() as u64;
    request.settings.settings_identity = settings.identity().unwrap();
    request.expected_identities.settings_identity = request.settings.settings_identity.clone();
    fs::write(root.join("inputs/settings.json"), settings_bytes).unwrap();
    manifest.input_set_identity = manifest.computed_input_set_identity().unwrap();
    manifest.manifest_identity = manifest.computed_manifest_identity().unwrap();
    request.input_manifest.manifest_identity = manifest.manifest_identity.clone();
    request.input_manifest.input_set_identity = manifest.input_set_identity.clone().unwrap();
    request.invocation_identity = request.computed_invocation_identity().unwrap();
    fs::write(
        root.join("inputs/manifest.json"),
        serde_json::to_vec(manifest).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("request.json"),
        serde_json::to_vec(request).unwrap(),
    )
    .unwrap();
}
