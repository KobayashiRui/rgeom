use std::collections::HashMap;
use std::fmt;

type VertexID = usize;
type HalfEdgeID = usize;
type FaceID = usize;


// 頂点構造体
#[derive(Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub edge: Option<HalfEdgeID>
}

// ハーフエッジ構造体
#[derive(Debug)]
pub struct HalfEdge {
    vertex: VertexID,
    twin: Option<HalfEdgeID>,
    next: Option<HalfEdgeID>,
    prev: Option<HalfEdgeID>,
    face: Option<FaceID>,
}

// 面構造体
#[derive(Debug)]
pub struct Face {
    pub edge: Option<HalfEdgeID>,
}

pub struct HalfEdgeDS {
    pub vertices: HashMap<VertexID, Vertex>,
    pub half_edges: HashMap<HalfEdgeID, HalfEdge>,
    pub faces: HashMap<FaceID, Face>,
    next_vertex_id: VertexID,
    next_half_edge_id: HalfEdgeID,
    next_face_id: FaceID,
    edge_map: HashMap<(VertexID, VertexID), HalfEdgeID>,
}

#[derive(Debug)]
pub struct HalfEdgeError {
    pub message: String,
}

impl fmt::Display for HalfEdgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HalfEdgeError: {}", self.message)
    }
}

impl std::error::Error for HalfEdgeError {}


impl HalfEdgeDS {
    pub fn new() -> Self{
        HalfEdgeDS {
            vertices: HashMap::new(),
            half_edges: HashMap::new(),
            faces: HashMap::new(),
            next_vertex_id: 0,
            next_half_edge_id: 0,
            next_face_id: 0,
            edge_map: HashMap::new(),
        }
    }

    // 頂点を追加
    pub fn add_vertex(&mut self, position: [f32; 3]) -> VertexID {
        let id: usize = self.next_vertex_id;
        self.next_vertex_id += 1;
        self.vertices.insert(
            id,
            Vertex {
                position,
                edge: None,
            },
        );
        id
    }

    // 面を追加
    pub fn add_face(&mut self, vertex_ids: &[VertexID]) -> Result<FaceID, HalfEdgeError> {
        let face_id = self.next_face_id;
        self.next_face_id += 1;
        let mut face = Face { edge: None };
        let n = vertex_ids.len();
        let mut half_edge_ids = Vec::new();

        // ハーフエッジの作成と双対の設定
        for i in 0..n {
            let start_vertex = vertex_ids[i];
            let end_vertex = vertex_ids[(i + 1) % n];
            let he_id = self.next_half_edge_id;
            self.next_half_edge_id += 1;

            // ハーフエッジの作成
            self.half_edges.insert(
                he_id,
                HalfEdge {
                    vertex: end_vertex,
                    twin: None,
                    next: None,
                    prev: None,
                    face: Some(face_id),
                },
            );

            // エッジマップのチェックと双対の設定
            let edge_key = (start_vertex, end_vertex);
            let twin_key = (end_vertex, start_vertex);
            if let Some(&twin_id) = self.edge_map.get(&twin_key) {
                // 双対ハーフエッジを設定
                self.half_edges.get_mut(&he_id).unwrap().twin = Some(twin_id);
                self.half_edges.get_mut(&twin_id).unwrap().twin = Some(he_id);
            } else {
                // エッジマップに追加
                self.edge_map.insert(edge_key, he_id);
            }

            half_edge_ids.push(he_id);
        }

        // nextとprevの設定
        for i in 0..n {
            let he_id = half_edge_ids[i];
            let next_he_id = half_edge_ids[(i + 1) % n];
            let prev_he_id = half_edge_ids[(i + n - 1) % n];

            self.half_edges.get_mut(&he_id).unwrap().next = Some(next_he_id);
            self.half_edges.get_mut(&he_id).unwrap().prev = Some(prev_he_id);
        }

        // 面の任意のハーフエッジを設定
        face.edge = Some(half_edge_ids[0]);
        self.faces.insert(face_id, face);

        // 頂点にハーフエッジを設定
        for i in 0..n {
            let vertex_id = vertex_ids[i];
            let he_id = half_edge_ids[i];
            let vertex = self.vertices.get_mut(&vertex_id).unwrap();
            if vertex.edge.is_none() {
                vertex.edge = Some(he_id);
            }
        }

        Ok(face_id)
    }

    fn get_half_edge(&self, he_id: HalfEdgeID) -> Result<&HalfEdge, HalfEdgeError> {
        println!("HE{:?}", self.half_edges);
        self.half_edges.get(&he_id).ok_or_else(|| HalfEdgeError {
            message: format!("Half-edge with ID {} not found", he_id),
        })
    }

    fn prev_half_edge_id(&self, he_id: HalfEdgeID) -> Result<HalfEdgeID, HalfEdgeError> {
        self.get_half_edge(he_id)?.prev.ok_or_else(|| HalfEdgeError {
            message: format!("Previous half-edge of {} does not exist", he_id),
        })
    }

    fn twin_half_edge_id(&self, he_id: HalfEdgeID) -> Result<HalfEdgeID, HalfEdgeError> {
        self.get_half_edge(he_id)?.twin.ok_or_else(|| HalfEdgeError {
            message: format!("Twin half-edge of {} does not exist", he_id),
        })
    }

    fn next_half_edge_id(&self, he_id: HalfEdgeID) -> Result<HalfEdgeID, HalfEdgeError> {
        self.get_half_edge(he_id)?.next.ok_or_else(|| HalfEdgeError {
            message: format!("Next half-edge of {} does not exist", he_id),
        })
    }
}

pub struct HalfEdgeHandler<'a> {
    half_edge_ds: &'a HalfEdgeDS,
    he_id: HalfEdgeID,
}

impl<'a> HalfEdgeHandler<'a> {
    pub fn new(half_edge_ds: &'a HalfEdgeDS, he_id: HalfEdgeID) -> Self {
        HalfEdgeHandler { half_edge_ds, he_id }
    }

    //pub fn prev(&self) -> Result<Self, HalfEdgeError> {
    //    let prev_id = self.half_edge_ds.prev_half_edge_id(self.he_id)?;
    //    Ok(Self::new(self.half_edge_ds, prev_id))
    //}

    //pub fn twin(&self) -> Result<Self, HalfEdgeError> {
    //    let twin_id = self.half_edge_ds.twin_half_edge_id(self.he_id)?;
    //    Ok(Self::new(self.half_edge_ds, twin_id))
    //}

    //pub fn next(&self) -> Result<Self, HalfEdgeError> {
    //    let next_id = self.half_edge_ds.next_half_edge_id(self.he_id)?;
    //    Ok(Self::new(self.half_edge_ds, next_id))
    //}

    //pub fn vertex(&self) -> Result<&'a Vertex, HalfEdgeError> {
    //    println!("V:{:?}", self.he_id);
    //    let he = self.half_edge_ds.get_half_edge(self.he_id)?;
    //    self.half_edge_ds.vertices.get(&he.vertex).ok_or_else(|| HalfEdgeError {
    //        message: format!("Vertex with ID {} not found", he.vertex),
    //    })
    //}
    pub fn prev(self) -> HalfEdgeHandlerResult<'a, Self> {
        match self.half_edge_ds.prev_half_edge_id(self.he_id) {
            Ok(prev_id) => HalfEdgeHandlerResult::Ok(Self::new(self.half_edge_ds, prev_id)),
            Err(e) => HalfEdgeHandlerResult::Err(self, e),
        }
    }

    pub fn twin(self) -> HalfEdgeHandlerResult<'a, Self> {
        match self.half_edge_ds.twin_half_edge_id(self.he_id) {
            Ok(twin_id) => HalfEdgeHandlerResult::Ok(Self::new(self.half_edge_ds, twin_id)),
            Err(e) => HalfEdgeHandlerResult::Err(self, e),
        }
    }

    pub fn next(self) -> HalfEdgeHandlerResult<'a, Self> {
        match self.half_edge_ds.next_half_edge_id(self.he_id) {
            Ok(next_id) => HalfEdgeHandlerResult::Ok(Self::new(self.half_edge_ds, next_id)),
            Err(e) => HalfEdgeHandlerResult::Err(self, e),
        }
    }

    pub fn vertex(self) -> HalfEdgeHandlerResult<'a, &'a Vertex> {
        match self.half_edge_ds.get_half_edge(self.he_id) {
            Ok(he) => match self.half_edge_ds.vertices.get(&he.vertex) {
                Some(vertex) => HalfEdgeHandlerResult::Ok(vertex),
                None => HalfEdgeHandlerResult::Err(self, HalfEdgeError {
                    message: format!("Vertex with ID {} not found", he.vertex),
                }),
            },
            Err(e) => HalfEdgeHandlerResult::Err(self, e),
        }
    }
}


pub enum HalfEdgeHandlerResult<'a, T> {
    Ok(T),
    Err(HalfEdgeHandler<'a>, HalfEdgeError),
}

// HalfEdgeHandlerResult にメソッドを追加
impl<'a> HalfEdgeHandlerResult<'a, HalfEdgeHandler<'a>> {
    pub fn prev(self) -> HalfEdgeHandlerResult<'a, HalfEdgeHandler<'a>> {
        match self {
            HalfEdgeHandlerResult::Ok(nav) => nav.prev(),
            HalfEdgeHandlerResult::Err(nav, e) => HalfEdgeHandlerResult::Err(nav, e),
        }
    }

    pub fn twin(self) -> HalfEdgeHandlerResult<'a, HalfEdgeHandler<'a>> {
        match self {
            HalfEdgeHandlerResult::Ok(nav) => nav.twin(),
            HalfEdgeHandlerResult::Err(nav, e) => HalfEdgeHandlerResult::Err(nav, e),
        }
    }

    pub fn next(self) -> HalfEdgeHandlerResult<'a, HalfEdgeHandler<'a>> {
        match self {
            HalfEdgeHandlerResult::Ok(nav) => nav.next(),
            HalfEdgeHandlerResult::Err(nav, e) => HalfEdgeHandlerResult::Err(nav, e),
        }
    }

    pub fn vertex(self) -> HalfEdgeHandlerResult<'a, &'a Vertex> {
        match self {
            HalfEdgeHandlerResult::Ok(nav) => nav.vertex(),
            HalfEdgeHandlerResult::Err(nav, e) => HalfEdgeHandlerResult::Err(nav, e),
        }
    }
}

impl<'a, T> HalfEdgeHandlerResult<'a, T> {
    pub fn and_then<U, F>(self, f: F) -> HalfEdgeHandlerResult<'a, U>
    where
        F: FnOnce(T) -> HalfEdgeHandlerResult<'a, U>,
    {
        match self {
            HalfEdgeHandlerResult::Ok(value) => f(value),
            HalfEdgeHandlerResult::Err(nav, e) => HalfEdgeHandlerResult::Err(nav, e),
        }
    }
}
