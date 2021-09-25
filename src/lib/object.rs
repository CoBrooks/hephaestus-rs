use std::fs::File;
use std::io::BufReader;
use obj::{ load_obj, Obj };
use cgmath::{ Euler, Deg, Quaternion, Vector3 };

use crate::{
    buffer_objects::Vertex
};

pub trait Viewable {
    fn get_indices(&self) -> Vec<u16>;
    fn get_vertices(&self) -> Vec<Vertex>;
}

pub struct Object {
    pub origin: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub model_path: String,
    object_data: Obj,
}

impl Object {
    pub fn new(origin: [f32; 3], scale: [f32; 3], model_path: String) -> Self {
        let data = Object::get_object_data(&model_path);

        Object {
            origin: origin.into(),
            scale: scale.into(),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            model_path,
            object_data: data
        }
    }

    fn get_object_data(model_path: &str) -> Obj {
        let input = BufReader::new(File::open(&model_path).expect(&format!("Error loading model file: {}", model_path)));
        load_obj(input).expect(&format!("Error reading model data: {}", model_path))
    }

    fn rotate(&mut self, r: Euler<Deg<f32>>) {
        self.rotation = r.into();
    }
}

impl Viewable for Object {
    fn get_indices(&self) -> Vec<u16> {
        self.object_data.indices.clone()
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        self.object_data.vertices.iter()
            .map(|v| Vertex {
                position: v.position,
                normal: v.normal,
                color: [1.0; 3],
                ..Vertex::default()
            })
            .map(|v| Vertex {
                position: [
                    self.origin[0] + (v.position[0] - self.origin[0]) * self.scale[0],
                    self.origin[1] + (v.position[1] - self.origin[1]) * self.scale[1],
                    self.origin[2] + (v.position[2] - self.origin[2]) * self.scale[2],
                ],
                ..v
            })
            .collect()
    }
}
