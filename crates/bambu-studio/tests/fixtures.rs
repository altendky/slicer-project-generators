use slicer_project_generator_bambu_studio::{IndexedTriangleMesh, NamedObject};

pub fn tetrahedron(offset: f64) -> IndexedTriangleMesh {
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

pub fn object(identity: &str, name: &str, offset: f64) -> NamedObject {
    NamedObject::new(identity, name, tetrahedron(offset))
}
