use crate::{HalfEdgeHandler, HalfEdgeID, HalfEdgeDS, Vertex, HalfEdgeError};
use crate::Plane;

fn find_intersectiong_edges(he_ds: &HalfEdgeDS, plane:&Plane){
    let mut intersection_he: Vec::<HalfEdgeID> = Vec::new();
    for (he_id, he) in &he_ds.half_edges {
        let he_nav = HalfEdgeHandler::new(&he_ds, he_id.clone());
        let edge = (|| -> Result<(&Vertex, &Vertex), HalfEdgeError> {
            let end_vertex = he_nav.vertex()?;
            let start_vertex = he_nav.prev()?.vertex()?;
            Ok((start_vertex, end_vertex))
        })();

        //エラー処理
        if let Err(_) = edge {
            continue;
        }
    }
}


pub fn slice(he_ds: &HalfEdgeDS, plane: &Plane){
    find_intersectiong_edges(he_ds, plane);
}