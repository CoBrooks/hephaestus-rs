use std::fs::File;
use std::sync::Arc;
use std::io::BufReader;
use obj::{ load_obj, Obj, TexturedVertex };
use cgmath::{ Euler, Deg, Quaternion, Vector3, Matrix4 };

use crate::{
    buffer_objects::Vertex,
    material::{ Diffuse, Material },
    world::World,
    engine::EngineTime,
    logger::{ APP_LOGGER, Logger, MessageEmitter }
};

pub trait Viewable: ViewableClone {
    fn get_indices(&self) -> &Vec<u16>;
    fn get_vertices(&self) -> &Vec<Vertex>;
    fn transform_mut(&mut self) -> &mut Transform;
    fn transform(&self) -> &Transform;
    fn get_material(&self) -> &Box<dyn Material>; 
    fn update(&mut self, world: &World, time: &EngineTime);
    fn set_name(&mut self, name: String);
    fn get_model_path(&self) -> String;
}

pub trait ViewableClone {
    fn boxed_clone(&self) -> Box<dyn Viewable>;
}

impl<V: 'static> ViewableClone for V where V: Viewable + Clone {
    fn boxed_clone(&self) -> Box<dyn Viewable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Viewable> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
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

#[derive(Clone)]
pub struct Object {
    pub transform: Transform,
    pub model_path: String,
    pub material: Box<dyn Material>,
    pub name: String,
    update_function: Arc<dyn Fn(&Self, &World, &EngineTime) -> Self>,
    vertices: Vec<Vertex>,
    indices: Vec<u16>
}

impl Object {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3], model_path: String) -> Self {
        let (indices, vertices) = if let Some(data) = Object::get_object_data(&model_path) {
            let indices = data.indices.clone();
            let vertices: Vec<Vertex> = data.vertices.iter()
                .map(|v| Vertex {
                    position: v.position,
                    normal: v.normal,
                    color,
                    uv: [v.texture[0], v.texture[1]]
                })
                .collect();

            (indices, vertices)
        } else {
            (Vec::new(), Vec::new())
        };

        Object {
            transform: Transform::new(
                origin.into(),
                scale.into(),
                [Deg(0.0), Deg(0.0), Deg(0.0)]
            ),
            model_path,
            material: Box::new(Diffuse::new(color)),
            name: String::new(),
            update_function: Arc::new(|o: &Object, _, _|{ o.clone() }),
            vertices,
            indices
        } 
    }

    fn get_object_data(model_path: &str) -> Option<Obj<TexturedVertex, u16>> {
        if let Some(file) = File::open(&model_path).ok() {
            let input = BufReader::new(file);
            
            if let Some(obj) = load_obj(input).ok() {
                APP_LOGGER.log_debug(&format!("Loaded OBJ data from '{}'", model_path), MessageEmitter::Object("Object Initializer".into()));
                Some(obj)
            } else {
                APP_LOGGER.log_error(&format!("Failed to load OBJ data from '{}'", model_path), MessageEmitter::Object("Object Initializer".into()));
                None
            }
        } else {
            APP_LOGGER.log_error(&format!("Cannot open file '{}'", model_path), MessageEmitter::Object("Object Initializer".into()));
            None
        }
    }

    pub fn add_update(&mut self, update: Box<dyn Fn(&mut Self, &World, &EngineTime)>) {
        // Allows the `add_update` method signature to be nicer to the end user
        let f = move |object: &Object, world: &World, time: &EngineTime| { 
            let mut o = object.clone(); // `object` is essentially `&self` when called later by `update`
            update(&mut o, world, time);// update self.clone() with the user-defined function
            o                           // return the updated value, which will then be assigned to `self` later
        };

        self.update_function = Arc::new(f); // Arc instead of Box so that Object: Clone
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

    fn update(&mut self, world: &World, time: &EngineTime) {
        // "This object (self) now equals the returned value of the update function when called with itself"
        *self = (self.update_function)(&self, world, time); 
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_model_path(&self) -> String {
        self.model_path.clone()
    }
}
