use crate::{HalfEdgeHandler, HalfEdgeID, HalfEdgeDS};
use crate::Plane;

pub fn slice(he_ds: &HalfEdgeDS, plan: &Plane){
    let mut intersection_he: Vec::<HalfEdgeID> = Vec::new();
    for (he_id, he) in &he_ds.half_edges {
        let he_nav = HalfEdgeHandler::new(&he_ds, he_id.clone());
        let end_vertex = he_nav.vertex();
        let start_vertex = he_nav.prev().vertex();
    }
}