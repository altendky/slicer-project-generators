use std::env;
use std::fs;

use slicer_project_generator_bambu_studio::{
    IndexedTriangleMesh, ProjectGeometry, generate_support_blocking_volumes,
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
        .expect("usage: support_blocking_validation_fixture OUTPUT.3mf");
    let entries = [
        ProjectGeometry::printable("validation-model-a", tetrahedron(0.0))
            .with_name("Validation model A"),
        ProjectGeometry::support_blocker(
            "validation-blocker-a1",
            "validation-model-a",
            tetrahedron(2.0),
        )
        .with_name("Blocker A1"),
        ProjectGeometry::printable("validation-model-b", tetrahedron(30.0))
            .with_name("Validation model B"),
        ProjectGeometry::support_blocker(
            "validation-blocker-b",
            "validation-model-b",
            tetrahedron(32.0),
        )
        .with_name("Blocker B"),
        ProjectGeometry::support_blocker(
            "validation-blocker-a2",
            "validation-model-a",
            tetrahedron(4.0),
        )
        .with_name("Blocker A2"),
    ];
    let project = generate_support_blocking_volumes(&entries)
        .expect("support-blocking validation fixture must be valid");
    fs::write(output, project.bytes).expect("validation fixture must be written");
}
