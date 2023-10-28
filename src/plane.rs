pub struct Plane {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Plane {
    pub fn new(position: [f32; 3], normal: [f32; 3]) -> Plane {
        return Plane { 
            position, 
            normal
        };
    }

}