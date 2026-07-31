use std::env;
use std::fs;

use slicer_project_generator_bambu_studio::{
    IndexedTriangleMesh, NamedObject, generate_named_objects,
};

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

fn main() {
    let output = env::args_os()
        .nth(1)
        .expect("usage: validation_fixture OUTPUT.3mf");
    let names = [
        "Bracket & <left> \"A\"",
        "支持部品 café",
        "Duplicate display name",
        "Duplicate display name",
    ];
    let objects = names
        .into_iter()
        .enumerate()
        .map(|(index, name)| {
            NamedObject::new(
                format!("validation-object-{index}"),
                name,
                tetrahedron(index as f64 * 20.0),
            )
        })
        .collect::<Vec<_>>();
    let project = generate_named_objects(&objects).expect("validation fixture must be valid");
    fs::write(output, project.bytes).expect("validation fixture must be written");
}
