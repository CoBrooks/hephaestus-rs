use std::fs::File;
use std::io::BufReader;
use obj::{ load_obj, Obj, TexturedVertex };
use cgmath::{ Euler, Deg, Quaternion, Vector3, Matrix4 };

use crate::{
    buffer_objects::Vertex,
    material::{ Diffuse, Material }
};

pub trait Viewable {
    fn get_indices(&self) -> &Vec<u16>;
    fn get_vertices(&self) -> &Vec<Vertex>;
    fn transform_mut(&mut self) -> &mut Transform;
    fn transform(&self) -> &Transform;
    fn get_material(&self) -> &Box<dyn Material>; 
}

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>
}

impl Transform {
    pub fn new(translation: [f32; 3], scale: [f32; 3], rotation: [Deg<f32>; 3]) -> Self {
        Self {
            translation: translation.into(),
            scale: scale.into(),
            rotation: Quaternion::from(Euler::new(rotation[0], rotation[1], rotation[2]))
        }
    }

    pub fn translate(&mut self, translation: [f32; 3]) {
        self.translation += Vector3::from(translation);
    }

    pub fn rotate(&mut self, rotation: [Deg<f32>; 3]) {
        let q = Quaternion::from(Euler::new(rotation[0], rotation[1], rotation[2]));
        self.rotation = self.rotation * q; // To "add" two rotations, you multiply the Quaternions
    }

    pub fn scale(&mut self, scale: [f32; 3]) {
        self.scale = Vector3::new(self.scale.x * scale[0], self.scale.y * scale[1], self.scale.z * scale[2]);
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        // Not SRT order? - https://docs.microsoft.com/en-us/dotnet/desktop/winforms/advanced/why-transformation-order-is-significant
        // This does get the desired result, though...
        Matrix4::from_translation(self.translation) 
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
            * Matrix4::from(self.rotation) 
    }
}

pub struct Object {
    pub transform: Transform,
    pub model_path: String,
    pub material: Box<dyn Material>,
    vertices: Vec<Vertex>,
    indices: Vec<u16>
}

impl Object {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3], model_path: String) -> Self {
        let data = Object::get_object_data(&model_path);

        let indices = data.indices.clone();
        let vertices: Vec<Vertex> = data.vertices.iter()
            .map(|v| Vertex {
                position: v.position,
                normal: v.normal,
                color,
                uv: [v.texture[0], v.texture[1]]
            })
            .collect();


        Object {
            transform: Transform::new(
                origin.into(),
                scale.into(),
                [Deg(0.0), Deg(0.0), Deg(0.0)]
            ),
            model_path,
            material: Box::new(Diffuse::new(color)),
            vertices,
            indices
        } 
    }

    fn get_object_data(model_path: &str) -> Obj<TexturedVertex, u16> {
        let input = BufReader::new(File::open(&model_path).expect(&format!("Error loading model file: {}", model_path)));
        load_obj(input).expect(&format!("Error reading model data: {}", model_path))
    }
}

impl Viewable for Object {
    fn get_indices(&self) -> &Vec<u16> {
        &self.indices
    }

    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_material(&self) -> &Box<dyn Material> {
        &self.material
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
    
    fn transform(&self) -> &Transform {
        &self.transform
    }
}
