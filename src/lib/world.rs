use crate::{
    buffer_objects::*,
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
            void_color: [0.01; 4]
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Viewable>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: DirectionalLight) {
        self.lights.push(light);
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.objects.iter()
            .flat_map(|o| o.get_vertices())
            .collect()
    }

    pub fn get_indices(&self) -> Vec<u16> {
        let num_vertices: Vec<u16> = self.objects.iter()
            .map(|o| o.get_vertices().len() as u16)
            .collect();
        
        let mut indices: Vec<u16> = Vec::new();

        //indices.append(&mut self.objects[0].get_indices()); ???

        for i in 1..self.objects.len() + 1 {
            let sum_up_to_i: u16 = num_vertices[..(i - 1)].iter().sum();

            let mut object_indices: Vec<u16> = self.objects[i - 1].get_indices()
                .iter()
                .map(|&v| if i > 0 { v + sum_up_to_i } else { v })
                .collect();

            indices.append(&mut object_indices);
        }

        indices
    }
}
