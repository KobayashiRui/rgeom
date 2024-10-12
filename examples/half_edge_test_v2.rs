use rgeom::{self, HalfEdgeError, HalfEdgeHandlerResult};


fn main(){
    let mut mesh =  rgeom::HalfEdgeDS::new();

    // 頂点の追加
    let v0 = mesh.add_vertex([0.0, 0.0, 0.0]);
    println!("add {}", v0);
    let v1 = mesh.add_vertex([1.0, 0.0, 0.0]);
    let v2 = mesh.add_vertex([1.0, 1.0, 0.0]);
    let v3 = mesh.add_vertex([0.0, 1.0, 0.0]);

    // 面の追加
    mesh.add_face(&[v0, v1, v2]).unwrap_or_else(|e| panic!("{}", e));
    mesh.add_face(&[v0, v2, v3]).unwrap_or_else(|e| panic!("{}", e));

    // ハーフエッジIDの取得
    let he_id = mesh
        .faces
        .get(&0)
        .and_then(|face| face.edge)
        .unwrap_or_else(|| panic!("Face 0 does not have an edge"));

    // エッジナビゲータを使用したチェーンアクセス
    let navigator = rgeom::HalfEdgeHandler::new(&mesh, he_id);

    let vertex = navigator.prev().twin().vertex();

    match vertex {
        HalfEdgeHandlerResult::Ok(vertex) => {
            println!("Vertex Position: {:?}", vertex.position);
        }
        HalfEdgeHandlerResult::Err(partial_nav, e) => {
            println!("An error occurred: {}", e);
            // 部分的な結果として partial_nav を使用できます
            // 例: partial_nav.he_id
        }
    }


}