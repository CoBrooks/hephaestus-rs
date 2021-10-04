use std::collections::HashMap;

use crate::{
    object::Viewable,
    camera::Camera,
    light::DirectionalLight,
    logger::{ self, MessageEmitter }
};

pub struct World {
    pub objects: Vec<Box<dyn Viewable>>,
    pub lights: Vec<DirectionalLight>,
    pub camera: Camera,
    pub void_color: [f32; 4],
    object_dict: HashMap<String, usize>,
}

impl Clone for World {
    fn clone(&self) -> Self {
        Self {
            objects: self.objects.iter()
                .map(|o| o.boxed_clone())
                .collect(),
            object_dict: self.object_dict.clone(),
            lights: self.lights.clone(),
            camera: self.camera.clone(),
            void_color: self.void_color.clone()
        }
    }
}

impl World {
    pub fn new(camera: Camera) -> Self {
        logger::log_debug("Instantiating world.", MessageEmitter::World);
        
        World {
            objects: Vec::new(),
            object_dict: HashMap::new(),
            lights: Vec::new(),
            camera,
            void_color: [0.01, 0.01, 0.01, 1.0]
        }
    }

    pub fn add_object(&mut self, name: &str, mut object: Box<dyn Viewable>) {
        object.set_name(name.into());

        let msg = format!("Adding \"{}\" (from '{}') to world.", name, object.get_model_path());
        logger::log_debug(&msg, MessageEmitter::World);
        
        self.objects.push(object);
        self.object_dict.insert(name.into(), self.objects.len());
    }

    pub fn get_object(&self, name: &str) -> Option<&Box<dyn Viewable>> {
        if let Some(&index) = self.object_dict.get(name) {
            Some(&self.objects.get(index).unwrap())
        } else {
            None
        }
    }

    pub fn add_light(&mut self, light: DirectionalLight) {
        logger::log_debug("Adding directional light to world.", MessageEmitter::World);
        
        self.lights.push(light);
    }
}
