// Repository-authored neutral fixtures for offline exact-target validation.
#[allow(dead_code)]
#[path = "../tests/common/mod.rs"]
mod common;
use slicer_project_generator_bambu_studio::{cache_key, geometry, runtime};
use std::{fs, path::PathBuf};

fn main() {
    let mut args = std::env::args_os().skip(1);
    let destination = PathBuf::from(args.next().expect("fixture directory"));
    let binary = PathBuf::from(args.next().expect("generator binary"));
    assert!(args.next().is_none());
    fs::create_dir_all(&destination).unwrap();
    let package = runtime::Package {
        build_identity: "validation-build-v1".into(),
        binary_identity: cache_key::hex_sha256(&fs::read(&binary).unwrap()),
        provenance_set_identity: "BBL-PROTOCOL-PROVENANCE-SET-v1".into(),
    };
    for (name, blockers, assembly) in [
        ("configured", false, false),
        ("assembly", false, true),
        ("configured-blockers", true, false),
        ("assembly-blockers", true, true),
        ("assembly-internal-blockers", true, true),
    ] {
        let dir = destination.join(name);
        fs::create_dir(&dir).unwrap();
        let (mut request, mut manifest, mut settings) =
            common::fixture(&dir, &package, blockers, assembly);
        let model = if name == "assembly-internal-blockers" {
            format!(
                r#"<model xmlns="{}" unit="centimeter"><resources><object id="1">{}</object><object id="2"><components><component objectid="1" transform="1 0 0 0 1 0 0 0 1 2 0 0"/></components></object></resources><build><item objectid="2" transform="0 1 0 -1 0 0 0 0 1 0 3 0"/><item objectid="1"/></build></model>"#,
                geometry::CORE,
                common::mesh_xml()
            )
        } else {
            common::model("centimeter")
        };
        let bytes = common::raw(&model, true);
        for object in &mut manifest.objects {
            fs::write(dir.join(&object.retained_content.path), &bytes).unwrap();
            object.retained_content.sha256 = cache_key::hex_sha256(&bytes);
            object.retained_content.byte_length = bytes.len() as u64;
        }
        if assembly {
            for (index, placement) in settings.placements.iter_mut().enumerate() {
                placement.matrix[3] = index as f64 * 0.025;
                placement.matrix[7] = index as f64 * 0.005;
            }
        }
        // The leaf resource is 10 mm; placements are neutral meters.
        if name != "assembly-internal-blockers" {
            assert_eq!(
                geometry::realize(&bytes, &geometry::IDENTITY)
                    .unwrap()
                    .vertices[1],
                [10., 0., 0.]
            );
        }
        common::bind(&dir, &mut request, &mut manifest, &settings);
        fs::copy(&binary, dir.join("slicer-project-generator-bambu-studio")).unwrap();
        fs::write(
            dir.join("package.json"),
            serde_json::to_vec(&package).unwrap(),
        )
        .unwrap();
    }
}
