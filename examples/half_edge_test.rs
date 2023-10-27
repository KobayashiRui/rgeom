use rgeom;

fn main() {
    println!("Hello, world!");
    let test_data_vertex = [
                    [0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]
                ];
    let test_face = [0,1,2,0,3,1];

    let mut he_ds = rgeom::HalfEdgeDS::new();
    for v in test_data_vertex{
        println!("add vertex: {}", he_ds.add_vertex(v[0], v[1],v[2], None));
    }

    for fi in 0..test_face.len()/3 {
        println!("f:{}", fi);
        he_ds.add_face_loop([test_face[fi*3], test_face[fi*3+1], test_face[fi*3+2]])
        //let v1_i = test_face[fi*3]; 
        //let v2_i = test_face[fi*3+1]; 
        //let v3_i = test_face[fi*3+2]; 
        //println!("v1:{}, v2:{}, v3:{}", v1_i, v2_i, v3_i);
        //let ds_fi = he_ds.add_face(Some([v1_i, v2_i]));

        //he_ds.add_half_edge(v1_i, v2_i, Some(ds_fi), Some([v2_i, v1_i]), None, None);
        ////he_ds.add_half_edge(v2_i, v1_i, Some(ds_fi), Some([v1_i, v2_i]), None, None);

        //he_ds.add_half_edge(v2_i, v3_i, Some(ds_fi), Some([v3_i, v2_i]), None, None);
        ////he_ds.add_half_edge(v3_i, v2_i, Some(ds_fi), Some([v2_i, v3_i]), None, None);

        //he_ds.add_half_edge(v3_i, v1_i, Some(ds_fi), Some([v1_i, v3_i]), None, None);
        ////he_ds.add_half_edge(v1_i, v3_i, Some(ds_fi), Some([v3_i, v1_i]), None, None);
    }

    println!("{:?}",he_ds.half_edge.len());
    println!("{:?}",he_ds.get_half_edge(0, 1).unwrap());
    //twinを取得
    println!("{:?}",he_ds.get_half_edge_twin(he_ds.get_half_edge(0, 1).unwrap()).unwrap());

    //faceを取得
    println!("face0: {:?}", he_ds.face[0]);

    //vertexを取得
    println!("v0: {:?}", he_ds.vertex[0]);

    //edgeを取得
    println!("edge: {:?}", he_ds.get_edge(&he_ds.get_half_edge(0, 1).unwrap()));


}
