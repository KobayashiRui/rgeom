use std::collections::HashMap;
pub struct HalfEdgeDS{
    pub half_edge: HashMap<[usize; 2], HalfEdge>,
    pub vertex: Vec<Vertex>, 
    pub face: Vec<Face>
}

impl HalfEdgeDS {

    pub fn new() -> HalfEdgeDS {
        HalfEdgeDS { 
            half_edge: HashMap::new(), 
            vertex: Vec::new(), 
            face: Vec::new(), 
        }
    }

    pub fn add_half_edge(&mut self, v1: usize, v2: usize, face: Option<usize>,twin: Option<[usize; 2]>, next: Option<[usize; 2]>, prev: Option<[usize; 2]>){
        if !self.half_edge.contains_key(&[v1,v2]) {
            self.half_edge.insert([v1,v2], HalfEdge::new(Some(v2), face, twin, next, prev));
        }
    }

    pub fn get_half_edge(&self, v1:usize, v2:usize) -> Option<&HalfEdge> {
        self.half_edge.get(&[v1, v2])
    }

    pub fn get(&self, key: &[usize; 2]) -> Option<&HalfEdge> {
        self.half_edge.get(key)
    }

    pub fn get_half_edge_twin(&self, he: &HalfEdge) -> Option<&HalfEdge> {
        self.half_edge.get(&he.twin.unwrap())
    }

    pub fn get_edge(&self, he: &HalfEdge) -> Option<[&Vertex; 2]>{
        let v2 = &self.vertex[he.opposite.unwrap()];
        let v1 = &self.vertex[self.get(&he.prev.unwrap()).unwrap().opposite.unwrap()];

        return Some([v1, v2])
    }

    pub fn get_edge_key(&self, key: &[usize; 2])-> [&Vertex; 2]{
        let he1 = self.get(key).unwrap();
        let he2 = self.get(&he1.prev.unwrap()).unwrap();

        return [&self.vertex[he2.opposite.unwrap()], &self.vertex[he1.opposite.unwrap()]]
    }

    pub fn add_vertex(&mut self, x: f32, y: f32, z:f32, half_edge: Option<[usize; 2]>)-> usize{
        self.vertex.push(
            Vertex{x,y,z,half_edge}
        );
        return self.vertex.len() - 1
    }

    pub fn add_face(&mut self, half_edge:Option<[usize; 2]>) -> usize{
        self.face.push(
            Face{half_edge}
        );
        return self.face.len() - 1
    }

    pub fn add_face_loop(&mut self, face_loop: [usize; 3]){
        let v1_i = face_loop[0]; 
        let v2_i = face_loop[1]; 
        let v3_i = face_loop[2]; 
        let ds_fi = self.add_face(Some([v1_i, v2_i]));

        //TODO: twinを追加する?
        self.add_half_edge(v1_i, v2_i, Some(ds_fi), Some([v2_i, v1_i]), Some([v2_i, v3_i]), Some([v3_i, v1_i]));
        if self.vertex[v2_i].half_edge.is_none() {
            self.vertex[v2_i].half_edge = Some([v1_i, v2_i]);
        }
        self.add_half_edge(v2_i, v3_i, Some(ds_fi), Some([v3_i, v2_i]), Some([v3_i, v1_i]), Some([v1_i, v2_i]));
        if self.vertex[v3_i].half_edge.is_none() {
            self.vertex[v3_i].half_edge = Some([v2_i, v3_i]);
        }
        self.add_half_edge(v3_i, v1_i, Some(ds_fi), Some([v1_i, v3_i]), Some([v1_i, v2_i]), Some([v2_i, v3_i]));
        if self.vertex[v1_i].half_edge.is_none() {
            self.vertex[v1_i].half_edge = Some([v3_i, v1_i]);
        }
    }

    pub fn get_face_loop(&self, face_index: usize) -> [&Vertex; 3]{
        let he_1 = self.get(&self.face[face_index].half_edge.unwrap()).unwrap();
        let he_2 = self.get(&he_1.next.unwrap()).unwrap();
        let he_3 = self.get(&he_2.next.unwrap()).unwrap();

        let v1 = &self.vertex[he_1.opposite.unwrap()];
        let v2 = &self.vertex[he_2.opposite.unwrap()];
        let v3 = &self.vertex[he_3.opposite.unwrap()];

        return [v1, v2, v3];
    }

    pub fn get_face_loop_index(&self, face_index: usize) -> [usize; 3]{
        let he_1 = self.get(&self.face[face_index].half_edge.unwrap()).unwrap();
        let he_2 = self.get(&he_1.next.unwrap()).unwrap();
        let he_3 = self.get(&he_2.next.unwrap()).unwrap();

        let v1_i = he_1.opposite.unwrap();
        let v2_i = he_2.opposite.unwrap();
        let v3_i = he_3.opposite.unwrap();

        return [v1_i, v2_i, v3_i];
    }

    pub fn get_face_loop_he(&self, face_index: usize) -> [[usize; 2]; 3]{
        let he1_key = self.face[face_index].half_edge.unwrap();
        let he1 = self.get(&he1_key).unwrap();

        let he2_key = he1.next.unwrap();
        let he2 = self.get(&he2_key).unwrap();

        let he3_key = he2.next.unwrap();
        //let he3 = self.get(&he3_key).unwrap();

        return [he1_key, he2_key, he3_key];
    }




}

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub half_edge: Option<[usize; 2]> 
}

#[derive(Debug)]
pub struct Face {
    half_edge: Option<[usize; 2]> 
}

#[derive(Debug)]
pub struct HalfEdge {
    pub opposite: Option<usize>,    //エッジ先端のvertexのindex
    pub face: Option<usize>,        //faceのindex
    pub twin: Option<[usize; 2]>,   //反対のhalfedgeのkey
    pub next: Option<[usize; 2]>,   //次のhalfedgeのkey
    pub prev: Option<[usize; 2]>,   //前のhalfedgeのkey
}

impl HalfEdge {
    pub fn new(opposite: Option<usize>, face: Option<usize>, twin: Option<[usize; 2]>, next: Option<[usize; 2]>, prev: Option<[usize; 2]>) -> HalfEdge {

        HalfEdge{
            opposite,
            face,
            twin,
            next,
            prev
        }

    }

}

