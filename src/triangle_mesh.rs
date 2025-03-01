struct TriangleMesh {
    vertices: Vec<[f32; 3]>,
    triangles: Vec<[usize; 3]>,
}

impl TriangleMesh {
    pub fn new() -> Self {
        TriangleMesh {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }
}