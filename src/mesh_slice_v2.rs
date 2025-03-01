use std::collections::btree_set::Intersection;

use crate::{HalfEdgeHandler, HalfEdgeID, HalfEdgeDS, Edge, Vertex, HalfEdgeError};
use crate::Plane;


struct IntersectionInfo {
    he_id : HalfEdgeID,
    point: na::Vector3<f32>
}

fn calc_intersectiong_edge(edge: &Edge, plane: &Plane) -> Option<na::Vector3<f32>> {
    // エッジ両端と平面の位置ベクトルの差を計算
    let d1 = (edge.start_vertex - plane.position).dot(&plane.normal);
    let d2 = (edge.end_vertex - plane.position).dot(&plane.normal);

    // 交差なし
    if d1 * d2 > 0.0 {
        return None;
    }

    // エッジが平面上にある
    // TODO: 浮動小数点誤差の対策
    if d1 == 0.0 && d2 == 0.0 {
        return  None;
    }

    let t = d1 / (d1 - d2);

    let intersection = edge.start_vertex + t * (edge.end_vertex - edge.start_vertex);

    return Some(intersection);
}

/// 平面とエッジとの交点を求める
/// 交差するエッジのハーフエッジIDと交点の情報を返す
fn find_intersectiong_edges(he_ds: &HalfEdgeDS, plane:&Plane) -> Vec<IntersectionInfo>{
    let mut intersections = Vec::new();
    for (he_id, he) in &he_ds.half_edges {
        let he_nav = HalfEdgeHandler::new(&he_ds, he_id.clone());
        let edge = he_nav.edge().unwrap();
        // check edge intersection plane
        let point = calc_intersectiong_edge(&edge, plane);
        if let Some(p) = point {
            intersections.push(IntersectionInfo{
                he_id: he_id.clone(),
                point: p,
            })
        }
    }
    return intersections;
}

/// 輪郭線をDFSで作成する
fn extract_contours(he_ds: &HalfEdgeDS, intersections: Vec<IntersectionInfo>){

}


pub fn slice(he_ds: &HalfEdgeDS, plane: &Plane){
    //交点リストを取得
    let intersections = find_intersectiong_edges(he_ds, plane);

    //輪郭抽出
    extract_contours(he_ds, intersections);
}