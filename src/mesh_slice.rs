use crate::{HalfEdgeDS, Vertex};
use crate::Plane;
//指定した平面に対して交点を求める
pub fn slice(he_ds: &HalfEdgeDS, plane: &Plane)-> Option<Vec<Vec<[f32;3]>>>{
    let mut slice_face: Vec<usize> = Vec::new(); //slice_face=今回の平面に該当するface
    //faceをループし、交差するfaceを探す
    for fi in 0..he_ds.face.len(){
        //let face_v_loop = he_ds.get_face_loop(fi);
        let fv = he_ds.get_face_loop(fi);
        let (cross_triangle , on_vertex) = cross_triangle_plane(fv, plane);
        if on_vertex {
            //println!("on_vertex: f:{:?}", fi);
        }
        if  cross_triangle{
            slice_face.push(fi);
        }
    }


    let mut slice_segment_list: Vec<Vec<[f32;3]>> = Vec::new();

    while slice_face.len() != 0 {

        let mut slice_segment: Vec<[f32;3]> = Vec::new();

        let mut lived_face: Vec<usize> = Vec::new(); //通過したface

        let mut search_faces: Vec<usize> = vec![slice_face[0]];

        let mut segment_end_flag = false;
        while !segment_end_flag {
            segment_end_flag = true;
            //println!("Search faces {:?}", search_faces);
            //println!("Search lived faces {:?}", lived_face);
            for sfi in 0..search_faces.len() {
                let mut end_flag = false;
                //faceの周りのfaceを取得する
                let he_key_list = he_ds.get_face_loop_he(search_faces[sfi]);


                for hek in he_key_list {
                    let he = he_ds.get(&hek).unwrap();
                    let he_twin_face = he_ds.get(&he.twin.unwrap()).unwrap().face.unwrap();

                    //println!("F: {:?}", he_twin_face);

                    //スライス対象のfaceでない場合はスキップ
                    if !slice_face.contains(&he_twin_face) {continue;}

                    //探索対象ならスキップ
                    if search_faces.contains(&he_twin_face) {continue;}

                    //すでに到達済みならスキップ
                    //if (lived_face.len() <= 2 && lived_face[0] != he_twin_face) && lived_face.contains(&he_twin_face) {continue;}
                    if lived_face.contains(&he_twin_face) {continue;}

                    //交点を求める
                    let ei = he_ds.get_edge_vertex_index(&hek);
                    let (cross_p, on_vertex) = get_edge_cross_point([&he_ds.vertex[ei[0]], &he_ds.vertex[ei[1]]], plane);

                    if cross_p.is_some() {
                        if on_vertex.is_some() { //頂点の場合
                            slice_segment.push(cross_p.unwrap());
                            lived_face.extend(search_faces.clone());
                            let on_vertex_index = if on_vertex.unwrap() == 0.0 { ei[0] } else { ei[1] };
                            let mut _search_faces = he_ds.get_vertex_face_loop(on_vertex_index).clone();
                            _search_faces.retain(|&f| slice_face.contains(&f));
                            _search_faces.retain(|&f| !lived_face.contains(&f));
                            search_faces = _search_faces;
                            end_flag = true;
                            break;
                        }else{ //エッジ
                            slice_segment.push(cross_p.unwrap());
                            lived_face.extend(search_faces.clone());
                            search_faces = vec![he_twin_face];
                            end_flag = true;
                            break;
                        }
                    }
                }
                if end_flag {
                    segment_end_flag = false;
                    break;
                }
            }

            if segment_end_flag {

                //println!("END Search faces {:?}", search_faces);
                //println!("END Search lived faces {:?}", lived_face);
                //println!("slice_face {:?}", slice_face);

                for sfi in 0..search_faces.len() {
                    let mut end_flag = false;
                    //faceの周りのfaceを取得する
                    let he_key_list = he_ds.get_face_loop_he(search_faces[sfi]);

                    for hek in he_key_list {
                        let he = he_ds.get(&hek).unwrap();
                        let he_twin_face = he_ds.get(&he.twin.unwrap()).unwrap().face.unwrap();
                        //println!("F: {:?}", he_twin_face);
                        //if lived_face[0] != he_twin_face {continue;}
                        if  !lived_face.contains(&he_twin_face) {continue;}


                        //交点を求める
                        let ei = he_ds.get_edge_vertex_index(&hek);
                        let (cross_p, on_vertex) = get_edge_cross_point([&he_ds.vertex[ei[0]], &he_ds.vertex[ei[1]]], plane);

                        if cross_p.is_some() {
                            if on_vertex.is_some() { //頂点の場合
                                slice_segment.push(cross_p.unwrap());
                                lived_face.extend(search_faces.clone());
                                //let on_vertex_index = if on_vertex.unwrap() == 0.0 { ei[0] } else { ei[1] };
                                //let mut _search_faces = he_ds.get_vertex_face_loop(on_vertex_index).clone();
                                //_search_faces.retain(|&f| !lived_face.contains(&f));
                                //search_faces = _search_faces;
                                end_flag = true;
                                break;
                            }else{ //エッジ
                                slice_segment.push(cross_p.unwrap());
                                lived_face.extend(search_faces.clone());
                                //search_faces = vec![he_twin_face];
                                end_flag = true;
                                break;
                            }
                        }
                    }
                    if end_flag {
                        break;
                    }
                }
                println!("Finish Search lived faces {:?}", lived_face);
            }

        }

        if lived_face.len() == 0 {
            break;
        }

        slice_segment.push(slice_segment[0]);
        slice_segment_list.push(slice_segment);

        slice_face.retain(|&f| !lived_face.contains(&f));

    }

    return Some(slice_segment_list);

    //交差するfaceの中でまだ線分を計算してないものがある場合
}

pub fn cross_triangle_plane(fv: [&Vertex; 3], plane: &Plane) -> (bool, bool){
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
        //println!("ONP");
        return (false,false);
    }else if *da > 0.0 && *db > 0.0 && *dc > 0.0 {
        return (false,false);
    }else if *da < 0.0 && *db < 0.0 && *dc < 0.0 {
        return (false,false);
    }else {
        if *da == 0.0 || *db == 0.0 || *dc == 0.0{
            //println!("ANY ON POINT");
            return (true, true);
        }
        //println!("da:{:?}, db:{:?}, dc: {:?}", da, db, dc);
        return (true, false);
    }
}

pub fn get_edge_cross_point(e: [&Vertex; 2], plane: &Plane)-> (Option<[f32; 3]>, Option<f32>){
    let pn = na::Vector3::new(plane.normal[0], plane.normal[1], plane.normal[2]);
    let b_a = na::Vector3::new(
        e[1].x - e[0].x, 
        e[1].y - e[0].y, 
        e[1].z - e[0].z);
    let bottom = pn.dot(&b_a);
    if bottom == 0.0 {
        return (None, None);
    }else {
        let p0_a = na::Vector3::new(
            plane.position[0] - e[0].x,
            plane.position[1] - e[0].y,
            plane.position[2] - e[0].z,
        );
        let top = pn.dot(&p0_a);
        let t = top / bottom;
        //println!("t:{:?}", t);
        if t < 0.0 || t > 1.0 {
            return (None, None);
        }

        let mut on_vertex = None;
        if t == 0.0 {
            on_vertex = Some(0.0);
        }else if t == 1.0{
            on_vertex = Some(1.0);
        }
        let a = na::Point3::new(e[0].x, e[0].y, e[0].z);
        let new_p = a + t * b_a;
        return (Some([new_p.x, new_p.y, new_p.z]), on_vertex);
    }
}
