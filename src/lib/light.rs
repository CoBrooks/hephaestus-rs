pub struct DirectionalLight {
    pub position: [f32; 4],
    pub color: [f32; 3]
}

impl DirectionalLight {
    pub fn new(position: [f32; 4], color: [f32; 3]) -> Self {
        Self {
            position,
            color
        }
    }
}
