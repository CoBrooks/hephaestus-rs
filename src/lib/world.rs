use crate::{
    object::Viewable,
    camera::Camera,
    light::DirectionalLight
};

pub struct World {
    pub objects: Vec<Box<dyn Viewable>>,
    pub lights: Vec<DirectionalLight>,
    pub camera: Camera,
    pub void_color: [f32; 4]
}

impl World {
    pub fn new(camera: Camera) -> Self {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
            camera,
            void_color: [0.01, 0.01, 0.01, 1.0]
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Viewable>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: DirectionalLight) {
        self.lights.push(light);
    }
}
