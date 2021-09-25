use std::fs;
use std::fs::File;
use std::io::{ BufReader, Cursor };
use obj::{ load_obj, Obj, TexturedVertex };
use cgmath::{ Euler, Deg, Quaternion, Vector3 };
use vulkano::image::{ ImmutableImage, ImageDimensions };

use crate::{
    buffer_objects::Vertex
};

pub trait Viewable {
    fn get_indices(&self) -> Vec<u16>;
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_texture_data(&self) -> (Vec<u8>, ImageDimensions);
}

pub struct Object {
    pub origin: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub model_path: String,
    pub texture_data: Option<(Vec<u8>, ImageDimensions)>,
    object_data: Obj<TexturedVertex, u16>,
}

impl Object {
    pub fn new(origin: [f32; 3], scale: [f32; 3], model_path: String) -> Self {
        let data = Object::get_object_data(&model_path);

        Object {
            origin: origin.into(),
            scale: scale.into(),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            model_path,
            texture_data: None,
            object_data: data
        }
    }

    fn get_object_data(model_path: &str) -> Obj<TexturedVertex, u16> {
        let input = BufReader::new(File::open(&model_path).expect(&format!("Error loading model file: {}", model_path)));
        load_obj(input).expect(&format!("Error reading model data: {}", model_path))
    }

    pub fn add_texture(&mut self, tex_path: &str) {
        let png_bytes = fs::read(&tex_path).unwrap(); 
        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1
        };
        let mut image_data = Vec::new();
        image_data.resize((info.width * info.height * 4) as usize, 0);
        reader.next_frame(&mut image_data).unwrap();

        self.texture_data = Some((image_data, dimensions));
    }

    pub fn rotate(&mut self, r: Euler<Deg<f32>>) {
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
                uv: [v.texture[0], v.texture[1]]
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

    fn get_texture_data(&self) -> (Vec<u8>, ImageDimensions) {
        self.texture_data.clone().unwrap()
    }
}
