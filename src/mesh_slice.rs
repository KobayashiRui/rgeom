use crate::{HalfEdgeDS, Vertex};
use crate::Plane;
//指定した平面に対して交点を求める
pub fn slice(he_ds: &HalfEdgeDS, plane: &Plane)-> Vec<[f32;3]>{
    let mut slice_face: Vec<usize> = Vec::new();
    //faceをループし、交差するfaceを探す
    for fi in 0..he_ds.face.len(){
        //let face_v_loop = he_ds.get_face_loop(fi);
        let fv = he_ds.get_face_loop(fi);
        if cross_triangle_plane(fv, plane) {
            slice_face.push(fi);
        }
    }

    println!("cross faces: {:?}", slice_face);
    let mut slice_segment: Vec<[f32;3]> = Vec::new();

    if slice_face.len() == 0 {
        return slice_segment;
    }
    //最初のクロスするエッジを探す
    let mut lived_face: Vec<usize> = Vec::new();

    let mut now_he: Option<[usize;2]> = None;

    let mut get_init_edge = || {
        for i in 0..slice_face.len(){
            println!("face: {:?}", slice_face[i]);
            let he_key_list = he_ds.get_face_loop_he(slice_face[i]);
            for hek in he_key_list {
                //twinのエッジのfaceが対象に入っているか確認
                let he = he_ds.get(&hek).unwrap();
                let he_twin_face = he_ds.get(&he.twin.unwrap()).unwrap().face.unwrap();
                if !slice_face.contains(&he_twin_face) {continue;}

                let e = he_ds.get_edge_key(&hek);
                let cross_p = get_edge_cross_point(e, plane);
                println!("cross edge: {:?}", cross_p);
                if cross_p.is_some() {
                    lived_face.push(slice_face[i]);
                    slice_segment.push(cross_p.unwrap());
                    now_he = Some(hek);
                    return;
                }
            }
        }
    };

    get_init_edge();

    //faceがなくなるまでループする;
    //TODO: 一周するまでループする;
    //while slice_face.len() >= slice_segment.len() {
    loop {
        println!("sf: {:?}, lf: {:?}", slice_face.len(), lived_face.len());
        let old_he = he_ds.get(&now_he.unwrap()).unwrap();
        let he_twin_key = old_he.twin.unwrap();
        let he_twin = he_ds.get(&he_twin_key).unwrap();
        
        let fi = he_twin.face.unwrap();
        println!("face: {:?}", fi);

        lived_face.push(fi);
        
        let he_key_list = he_ds.get_face_loop_he(fi);
        let mut count = 0;
        println!("len: {:?}", he_key_list.len());
        for hek in he_key_list {
            println!("count: {:?}", count);
            count += 1;
            if hek == he_twin_key {continue} ;
            let he = he_ds.get(&hek).unwrap();
            let he_twin_face = he_ds.get(&he.twin.unwrap()).unwrap().face.unwrap();
            println!("twinface: {:?}", he_twin_face);
            if !slice_face.contains(&he_twin_face) {continue;}
            if lived_face.contains(&he_twin_face) {continue;}

            let e = he_ds.get_edge_key(&hek);
            let cross_p = get_edge_cross_point(e, plane);
            println!("cross edge: {:?}", cross_p);
            if cross_p.is_some() {
                slice_segment.push(cross_p.unwrap());
                now_he = Some(hek);
                count = -1;
                break;
            }
        }
        if count >= 3 {
            break;
        }
    }

    slice_segment.push( slice_segment[0]);
    println!("slice_face: {:?}\nlived_face: {:?}", slice_face, lived_face);
    println!("slice_segment: {:?}", slice_segment);

    return slice_segment;

    //交差するfaceの中でまだ線分を計算してないものがある場合
}

pub fn cross_triangle_plane(fv: [&Vertex; 3], plane: &Plane) -> bool {
    //face
    let pn = na::Vector3::new(plane.normal[0], plane.normal[1], plane.normal[2]);
    let pa = na::Vector3::new(
        plane.position[0] - fv[0].x, 
        plane.position[1] - fv[0].y, 
        plane.position[2] - fv[0].z);
    let pb = na::Vector3::new(
        plane.position[0] - fv[1].x, 
        plane.position[1] - fv[1].y, 
        plane.position[2] - fv[1].z);
    let pc = na::Vector3::new(
        plane.position[0] - fv[2].x, 
        plane.position[1] - fv[2].y, 
        plane.position[2] - fv[2].z);

    let da = &pn.dot(&pa);
    let db = &pn.dot(&pb);
    let dc = &pn.dot(&pc);

    if *da == 0.0 && *db == 0.0 && *dc == 0.0 {
        return false;
    }else if *da > 0.0 && *db > 0.0 && *dc > 0.0 {
        return false;
    }else if *da < 0.0 && *db < 0.0 && *dc < 0.0 {
        return false;
    }else {
        println!("da:{:?}, db:{:?}, dc: {:?}", da, db, dc);
        return true;
    }
}

pub fn get_edge_cross_point(e: [&Vertex; 2], plane: &Plane)-> Option<[f32; 3]>{
    let pn = na::Vector3::new(plane.normal[0], plane.normal[1], plane.normal[2]);
    let b_a = na::Vector3::new(
        e[1].x - e[0].x, 
        e[1].y - e[0].y, 
        e[1].z - e[0].z);
    let bottom = pn.dot(&b_a);
    if bottom == 0.0 {
        return None;
    }else {
        let p0_a = na::Vector3::new(
            plane.position[0] - e[0].x,
            plane.position[1] - e[0].y,
            plane.position[2] - e[0].z,
        );
        let top = pn.dot(&p0_a);
        let t = top / bottom;
        let a = na::Point3::new(e[0].x, e[0].y, e[0].z);
        let new_p = a + t * b_a;
        return Some([new_p.x, new_p.y, new_p.z]);
    }
}

//線分と平面の交点を求める
//pub fn cross_segment_plane() -> bool {
//}