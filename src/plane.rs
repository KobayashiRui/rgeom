pub struct Plane {
    //pub position: [f32; 3], //平面の位置
    //pub normal: [f32; 3], //平面の法線方向単位ベクトル
    pub position: na::Vector3<f32>,
    pub normal: na::Vector3<f32>,
}

impl Plane {
    pub fn new(position: [f32; 3], normal: [f32; 3]) -> Plane {
        let position_vec = na::Vector3::new(position[0], position[1], position[2]);
        let normal_vec = na::Vector3::new(normal[0], normal[1], normal[2]);
        return Plane { 
            position:position_vec, 
            normal: normal_vec,
        };
    }

}