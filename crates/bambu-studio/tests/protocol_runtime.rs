mod common;
use common::*;
use slicer_project_generator_bambu_studio::{
    ProjectGeometry, cache_key, generator_protocol::*, geometry, runtime, settings::Settings,
};
use std::{fs, process::Command};

fn result(root: &std::path::Path) -> GeneratorResult {
    parse_result(&fs::read(root.join("result.json")).unwrap()).unwrap()
}

#[test]
fn configured_and_rigid_assembly_inputs_keep_distinct_paths_order_and_placements() {
    for assembly in [false, true] {
        for blockers in [false, true] {
            let temp = Temp::new();
            let package = package();
            let (request, _, _) = fixture(&temp.0, &package, blockers, assembly);
            assert!(runtime::invoke(&temp.0, "request.json", "result.json", &package).unwrap());
            let result = result(&temp.0);
            result.validate_against_request(&request).unwrap();
            let bytes = fs::read(temp.0.join("outputs/project.3mf")).unwrap();
            assert_eq!(result.output.unwrap().sha256, cache_key::hex_sha256(&bytes));
            let files = geometry::archive(&bytes).unwrap();
            let doc = geometry::xml(&files["3D/3dmodel.model"]).unwrap();
            let vertices: Vec<_> = doc
                .descendants()
                .filter(|n| n.has_tag_name((geometry::CORE, "vertices")))
                .collect();
            assert_eq!(vertices.len(), if blockers { 5 } else { 2 });
            for (i, source_index) in if blockers {
                vec![0, 1, 2, 2, 3]
            } else {
                vec![0, 1]
            }
            .into_iter()
            .enumerate()
            {
                let x = vertices[i]
                    .children()
                    .find(|n| n.is_element())
                    .unwrap()
                    .attribute("x")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                assert_eq!(
                    x,
                    if assembly {
                        source_index as f64 * 2000.
                    } else {
                        0.
                    }
                );
            }
            if blockers {
                let config = geometry::xml(&files["Metadata/model_settings.config"]).unwrap();
                assert_eq!(
                    config
                        .descendants()
                        .filter(|n| n.attribute("subtype") == Some("support_blocker"))
                        .count(),
                    3
                );
            }
            assert!(!temp.0.join("result.json.generator-tmp").exists());
            assert!(!temp.0.join("outputs/project.3mf.generator-tmp").exists());
            fs::remove_file(temp.0.join("outputs/project.3mf")).unwrap();
            fs::remove_file(temp.0.join("result.json")).unwrap();
            assert!(runtime::invoke(&temp.0, "request.json", "result.json", &package).unwrap());
            assert_eq!(bytes, fs::read(temp.0.join("outputs/project.3mf")).unwrap());
        }
    }
}

#[test]
fn malformed_and_inconsistent_requests_fail_without_candidates() {
    for case in 0..16 {
        let temp = Temp::new();
        let package = package();
        let (mut request, mut manifest, mut settings) = fixture(&temp.0, &package, true, true);
        match case {
            0 => {
                manifest.objects[1].retained_content.path =
                    manifest.objects[0].retained_content.path.clone();
            }
            1 => {
                manifest.objects[1].object_identity = manifest.objects[0].object_identity.clone();
            }
            2 => {
                settings.placements.swap(0, 1);
            }
            3 => {
                settings.placements.pop();
            }
            4 => {
                settings.blockers[0].targets = vec!["object-3".into()];
            }
            5 => {
                manifest.objects[0].role = InputRole::AuxiliaryGeometry;
            }
            6 => {
                manifest.objects[0].retained_content.sha256 = "0".repeat(64);
            }
            7 => {
                manifest.objects[0].retained_content.byte_length += 1;
            }
            8 => {
                request.output.max_byte_length = 1;
            }
            9 => {
                request.expected_identities.validation_identity = "wrong-validator".into();
            }
            10 => {
                manifest.objects[1].retained_content.path = "inputs/missing.3mf".into();
            }
            11 => {
                manifest.objects[0].retained_content.path = "inputs/../escape.3mf".into();
            }
            12 => {
                settings.blockers[0].object_identity = "object-0".into();
            }
            13 => {
                settings.placements[0].matrix[0] = 0.;
            }
            14 => {
                settings.placements[0].matrix[3] = 1e30;
            }
            15 => {
                manifest.objects[0].display_name = Some("bad\u{1}".into());
            }
            _ => unreachable!(),
        }
        bind(&temp.0, &mut request, &mut manifest, &settings);
        assert!(
            !runtime::invoke(&temp.0, "request.json", "result.json", &package).unwrap(),
            "case {case}"
        );
        let result = result(&temp.0);
        assert_eq!(result.status, ResultStatus::Failure);
        assert!(result.output.is_none());
        assert!(!temp.0.join("outputs/project.3mf").exists());
        assert!(fs::read(temp.0.join("result.json")).unwrap().len() < MAX_RESULT_BYTES);
    }
}

#[test]
fn malformed_json_oversized_documents_and_symlinks_fail_closed() {
    for bytes in [b"{".to_vec(), vec![b' '; MAX_REQUEST_BYTES + 1]] {
        let temp = Temp::new();
        fs::write(temp.0.join("request.json"), bytes).unwrap();
        assert!(!runtime::invoke(&temp.0, "request.json", "result.json", &package()).unwrap());
        assert_eq!(
            result(&temp.0).errors[0].category,
            ErrorCategory::MalformedRequest
        );
    }
    for target in ["inputs/geometry-0.3mf", "inputs"] {
        let temp = Temp::new();
        let package = package();
        fixture(&temp.0, &package, false, false);
        let original = temp.0.join(target);
        let moved = temp.0.join("moved");
        fs::rename(&original, &moved).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&moved, &original).unwrap();
        assert!(!runtime::invoke(&temp.0, "request.json", "result.json", &package).unwrap());
        assert!(!temp.0.join("outputs/project.3mf").exists());
    }
}

#[test]
fn raw_profile_failures_are_structured_and_atomic() {
    let base = model("meter");
    for source in [
        base.replace("meter", "yard"),
        base.replace("objectid=\"1\"", "objectid=\"999\""),
        base.replace("v1=\"0\"", "v1=\"999\""),
        base.replace("x=\"1\"", "x=\"NaN\""),
        base.replace("<resources>", "<resources><basematerials id=\"3\"/>"),
        base.replace(
            "<build>",
            "<build><item objectid=\"1\" transform=\"1 2 3\"/>",
        ),
        base.replace("<model ", "<model requiredextensions=\"unknown\" "),
    ] {
        let temp = Temp::new();
        let package = package();
        let (mut request, mut manifest, settings) = fixture(&temp.0, &package, false, false);
        let bytes = raw(&source, true);
        fs::write(temp.0.join("inputs/geometry-0.3mf"), &bytes).unwrap();
        manifest.objects[0].retained_content.sha256 = cache_key::hex_sha256(&bytes);
        manifest.objects[0].retained_content.byte_length = bytes.len() as u64;
        bind(&temp.0, &mut request, &mut manifest, &settings);
        assert!(!runtime::invoke(&temp.0, "request.json", "result.json", &package).unwrap());
        assert!(!temp.0.join("outputs/project.3mf").exists());
    }
}

#[test]
fn geometry_units_nested_transforms_and_placement_are_realized_once() {
    for (unit, factor) in [
        ("micron", 0.001),
        ("millimeter", 1.),
        ("centimeter", 10.),
        ("inch", 25.4),
        ("foot", 304.8),
        ("meter", 1000.),
    ] {
        let source = format!(
            r#"<model xmlns="{}" unit="{unit}"><resources><object id="1">{}</object><object id="2"><components><component objectid="1" transform="1 0 0 0 1 0 0 0 1 2 0 0"/></components></object></resources><build><item objectid="2" transform="0 1 0 -1 0 0 0 0 1 0 3 0"/><item objectid="1"/></build></model>"#,
            geometry::CORE,
            mesh_xml()
        );
        let mut placement = geometry::IDENTITY;
        placement[3] = 0.01;
        let mesh = geometry::realize(&raw(&source, true), &placement).unwrap();
        assert_eq!(mesh.vertices.len(), 8);
        assert!((mesh.vertices[0][0] - 10.).abs() < 1e-10);
        assert!((mesh.vertices[0][1] - 5. * factor).abs() < 1e-10);
        assert!((mesh.vertices[4][0] - 10.).abs() < 1e-10);
    }
    let reflected = model("millimeter");
    let mut placement = geometry::IDENTITY;
    placement[0] = -1.;
    let mesh = geometry::realize(&raw(&reflected, false), &placement).unwrap();
    assert_eq!(mesh.triangles[0], [0, 1, 2]);
}

#[test]
fn settings_schema_signed_zero_and_closed_contract_match_service() {
    assert_eq!(
        slicer_project_generator_bambu_studio::settings::schema_identity().unwrap(),
        "adfbdd411a8562cd84918ca9facf8c91f9cfeebed1a5cc606f46b2289920e465"
    );
    let temp = Temp::new();
    let (_, _, mut settings) = fixture(&temp.0, &package(), false, false);
    let positive = settings.identity().unwrap();
    for value in &mut settings.placements[0].matrix {
        if *value == 0. {
            *value = -0.;
        }
    }
    assert_eq!(positive, settings.identity().unwrap());
    let parsed = Settings::parse(&serde_json::to_vec(&settings).unwrap()).unwrap();
    assert!(parsed.placements[0].matrix[3].is_sign_positive());
    for value in [
        serde_json::json!({"schemaVersion":2,"blockers":[],"placements":[],"extra":1}),
        serde_json::json!({"schemaVersion":2,"blockers":[],"placements":[{"objectIdentity":"one","matrix":[1,2]}]}),
    ] {
        assert!(Settings::parse(&serde_json::to_vec(&value).unwrap()).is_err());
    }
}

#[test]
fn final_validator_rejects_mesh_role_name_association_and_placement_corruption() {
    let temp = Temp::new();
    let package = package();
    fixture(&temp.0, &package, true, true);
    runtime::invoke(&temp.0, "request.json", "result.json", &package).unwrap();
    let bytes = fs::read(temp.0.join("outputs/project.3mf")).unwrap();
    let entries = vec![
        ProjectGeometry::printable(
            "a",
            geometry::realize(&raw(&model("meter"), false), &geometry::IDENTITY).unwrap(),
        )
        .with_name("Original"),
    ];
    let valid = slicer_project_generator_bambu_studio::generate_support_blocking_volumes(&entries)
        .unwrap()
        .bytes;
    runtime::self_validate(&valid, &entries, false).unwrap();
    for (path, from, to) in [
        ("3D/3dmodel.model", "x=\"1000\"", "x=\"999\""),
        ("3D/3dmodel.model", "objectid=\"2\"", "objectid=\"1\""),
        (
            "3D/3dmodel.model",
            "1 0 0 0 1 0 0 0 1 0 0 0",
            "1 0 0 0 1 0 0 0 1 1 0 0",
        ),
        (
            "Metadata/model_settings.config",
            "normal_part",
            "support_blocker",
        ),
        ("Metadata/model_settings.config", "Original", "Changed"),
    ] {
        let mut files = geometry::archive(&valid).unwrap();
        let changed = String::from_utf8(files[path].clone())
            .unwrap()
            .replace(from, to);
        files.insert(path.into(), changed.into_bytes());
        assert!(runtime::self_validate(&repack(files, false), &entries, false).is_err());
    }
    assert!(runtime::self_validate(&bytes, &entries, false).is_err());
}

#[test]
fn actual_binary_uses_runner_arguments_and_hash_bound_package_metadata() {
    let temp = Temp::new();
    let bin = temp.0.join("slicer-project-generator-bambu-studio");
    fs::copy(
        env!("CARGO_BIN_EXE_slicer-project-generator-bambu-studio"),
        &bin,
    )
    .unwrap();
    let mut package = package();
    package.binary_identity = cache_key::hex_sha256(&fs::read(&bin).unwrap());
    fs::write(
        temp.0.join("package.json"),
        serde_json::to_vec(&package).unwrap(),
    )
    .unwrap();
    let (request, _, _) = fixture(&temp.0, &package, true, true);
    let status = Command::new(&bin)
        .args(["--request", "request.json", "--result", "result.json"])
        .current_dir(&temp.0)
        .status()
        .unwrap();
    assert!(status.success());
    result(&temp.0).validate_against_request(&request).unwrap();
    fs::remove_file(temp.0.join("result.json")).unwrap();
    fs::remove_file(temp.0.join("outputs/project.3mf")).unwrap();
    package.binary_identity = "wrong-binary".into();
    fs::write(
        temp.0.join("package.json"),
        serde_json::to_vec(&package).unwrap(),
    )
    .unwrap();
    let status = Command::new(&bin)
        .args(["--request", "request.json", "--result", "result.json"])
        .current_dir(&temp.0)
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(1));
    assert_eq!(result(&temp.0).errors[0].code, "package_identity_invalid");
    assert!(!temp.0.join("outputs/project.3mf").exists());
}

#[test]
fn explicit_nulls_are_rejected_by_the_closed_protocol() {
    let temp = Temp::new();
    let (request, manifest, _) = fixture(&temp.0, &package(), false, false);
    let mut value = serde_json::to_value(manifest).unwrap();
    value["objects"][0]["displayName"] = serde_json::Value::Null;
    assert!(parse_input_manifest(&serde_json::to_vec(&value).unwrap()).is_err());
    let mut value = serde_json::to_value(runtime::failure_result(
        "test_failure",
        ErrorCategory::InvalidInput,
        Some(&request),
    ))
    .unwrap();
    value["output"] = serde_json::Value::Null;
    assert!(parse_result(&serde_json::to_vec(&value).unwrap()).is_err());
}

#[test]
fn archive_and_xml_bounds_and_component_cycles_are_rejected() {
    let bytes = raw(&model("millimeter"), false);
    let eocd = bytes.len() - 22;
    for mutated in {
        let mut count = bytes.clone();
        count[eocd + 8..eocd + 12].copy_from_slice(&[255, 255, 255, 255]);
        let mut offset = bytes.clone();
        offset[eocd + 16..eocd + 20].copy_from_slice(&u32::MAX.to_le_bytes());
        let mut fallback = bytes.clone();
        fallback.extend_from_slice(&bytes[eocd..]);
        let mut zip64 = bytes.clone();
        zip64.splice(eocd..eocd, b"PK\x06\x07".iter().copied().chain([0; 16]));
        vec![count, offset, fallback, zip64]
    } {
        assert!(geometry::archive(&mutated).is_err());
    }
    let mut names = geometry::archive(&bytes)
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    names.push(("../escape".into(), vec![1]));
    assert!(geometry::archive(&repack(names, false)).is_err());
    assert!(
        geometry::archive(&repack(
            [(
                String::from("bomb"),
                vec![0; geometry::MAX_DECODED_BYTES + 1]
            )],
            true
        ))
        .is_err()
    );
    let mut budget = 1;
    assert!(geometry::realize_with_budget(&bytes, &geometry::IDENTITY, &mut budget).is_err());
    let cycle = format!(
        r#"<model xmlns="{}"><resources><object id="1"><components><component objectid="1"/></components></object></resources><build><item objectid="1"/></build></model>"#,
        geometry::CORE
    );
    assert!(geometry::realize(&raw(&cycle, false), &geometry::IDENTITY).is_err());
    assert!(geometry::xml(b"<!DOCTYPE model [<!ENTITY x 'value'>]><model>&x;</model>").is_err());
    let deep = format!("{}{}", "<node>".repeat(65), "</node>".repeat(65));
    assert!(geometry::xml(deep.as_bytes()).is_err());
}

#[test]
fn validator_checks_package_metadata_and_exact_xml_structure() {
    let mesh = geometry::realize(&raw(&model("millimeter"), false), &geometry::IDENTITY).unwrap();
    let entries = vec![ProjectGeometry::printable("a", mesh).with_name("Original")];
    let valid = slicer_project_generator_bambu_studio::generate_support_blocking_volumes(&entries)
        .unwrap()
        .bytes;
    for (path, from, to) in [
        (
            "[Content_Types].xml",
            "application/vnd.ms-package.3dmanufacturing-3dmodel+xml",
            "wrong",
        ),
        ("3D/3dmodel.model", "<resources>", "<resources bad=\"1\">"),
        ("3D/3dmodel.model", "<components>", "<components bad=\"1\">"),
        ("3D/3dmodel.model", "<build>", "<build bad=\"1\">"),
        (
            "3D/3dmodel.model",
            "printable=\"1\"/>",
            "printable=\"1\"><unexpected/></item>",
        ),
        (
            "Metadata/model_settings.config",
            "<config>",
            "<config bad=\"1\">",
        ),
        (
            "Metadata/model_settings.config",
            "<object id=",
            "<object xmlns=\"foreign\" id=",
        ),
        (
            "Metadata/model_settings.config",
            "</part>",
            "<part id=\"99\"/></part>",
        ),
        (
            "Metadata/model_settings.config",
            "value=\"Original\"/>",
            "value=\"Original\"><unexpected/></metadata>",
        ),
        (
            "Metadata/model_settings.config",
            "</config>",
            "unexpected</config>",
        ),
    ] {
        let mut files = geometry::archive(&valid).unwrap();
        let text = String::from_utf8(files[path].clone()).unwrap();
        assert!(text.contains(from));
        files.insert(path.into(), text.replace(from, to).into_bytes());
        assert!(
            runtime::self_validate(&repack(files, false), &entries, false).is_err(),
            "mutation {from}"
        );
    }
}

#[test]
fn anonymous_names_and_temporary_output_conflicts_are_handled_atomically() {
    let temp = Temp::new();
    let package = package();
    let (mut request, mut manifest, settings) = fixture(&temp.0, &package, false, false);
    for object in &mut manifest.objects {
        object.display_name = None;
    }
    bind(&temp.0, &mut request, &mut manifest, &settings);
    assert!(runtime::invoke(&temp.0, "request.json", "result.json", &package).unwrap());
    fs::remove_file(temp.0.join("outputs/project.3mf")).unwrap();
    fs::remove_file(temp.0.join("result.json")).unwrap();
    fs::write(
        temp.0.join("outputs/project.3mf.generator-tmp"),
        b"preserve",
    )
    .unwrap();
    assert!(!runtime::invoke(&temp.0, "request.json", "result.json", &package).unwrap());
    assert!(!temp.0.join("outputs/project.3mf").exists());
    assert_eq!(
        fs::read(temp.0.join("outputs/project.3mf.generator-tmp")).unwrap(),
        b"preserve"
    );
    fs::remove_file(temp.0.join("outputs/project.3mf.generator-tmp")).unwrap();
    fs::remove_file(temp.0.join("result.json")).unwrap();
    fs::write(temp.0.join("result.json.generator-tmp"), b"preserve").unwrap();
    assert!(runtime::invoke(&temp.0, "request.json", "result.json", &package).is_err());
    assert!(!temp.0.join("outputs/project.3mf").exists());
    assert_eq!(
        fs::read(temp.0.join("result.json.generator-tmp")).unwrap(),
        b"preserve"
    );
}
