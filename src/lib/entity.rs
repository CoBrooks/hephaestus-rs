use hephaestus_macros::*;
use std::sync::Arc;
use std::io::Cursor;
use std::fs;
use downcast_rs::{ Downcast, impl_downcast };
use cgmath::{ Vector3, Matrix4, Quaternion, Euler, Deg };
use vulkano::image::{ ImageDimensions, ImmutableImage, view::ImageView };
use vulkano::sampler::Sampler;
use vulkano::sync::NowFuture;
use vulkano::command_buffer::{ CommandBufferExecFuture, PrimaryAutoCommandBuffer };
use vulkano::device::{ Device, Queue };
use vulkano::format::Format;

use crate::{
    mesh_data::{ MeshData, MeshType },
    world::World
};

pub trait Component: Downcast + ComponentClone { 
    fn get_id(&self) -> &usize;
}
impl_downcast!(Component);

pub trait ComponentClone {
    fn boxed_clone(&self) -> Box<dyn Component>;
}

impl<C: 'static> ComponentClone for C where C: Component + Clone {
    fn boxed_clone(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}

pub trait ComponentCollection {
    fn has_transform(&self) -> bool;
    fn has_mesh(&self) -> bool;
    fn has_material(&self) -> bool;
    fn has_texture(&self) -> bool;
}

impl ComponentCollection for Vec<&Box<dyn Component>> {
    fn has_transform(&self) -> bool {
        !self.iter()
            .filter_map(|c| c.downcast_ref::<Transform>())
            .collect::<Vec<&Transform>>()
            .is_empty()
    }
    fn has_mesh(&self) -> bool {
        !self.iter()
            .filter_map(|c| c.downcast_ref::<Mesh>())
            .collect::<Vec<&Mesh>>()
            .is_empty()
    }
    fn has_material(&self) -> bool {
        !self.iter()
            .filter_map(|c| c.downcast_ref::<Material>())
            .collect::<Vec<&Material>>()
            .is_empty()
    }
    fn has_texture(&self) -> bool {
        !self.iter()
            .filter_map(|c| c.downcast_ref::<Texture>())
            .collect::<Vec<&Texture>>()
            .is_empty()
    }
}

#[derive(Clone, Component)]
pub struct Entity {
    id: usize,
}

#[derive(Clone, Component)]
pub struct Transform {
    id: usize,
    pub translation: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>
}

impl Transform {
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

#[derive(Clone, Component)]
pub struct Logic {
    id: usize,
    pub init: Box<fn(usize, &mut World)>,
    pub update: Box<fn(usize, &mut World)>
}

#[derive(Clone, Component)]
pub struct Mesh {
    id: usize,
    pub data: MeshData,
    pub mesh_type: MeshType
}

impl Mesh {
    pub fn init(&mut self) {
        match self.mesh_type.clone() {
            MeshType::Model(path) => {
                self.data = MeshData::load(&path);
            },
            MeshType::Primitive(primitive_type) => {
                self.data = MeshData::generate(primitive_type);
            }
        }
    }
}

#[derive(Clone, Component)]
pub struct Material {
    id: usize,
    pub color: [f32; 3]
}

#[derive(Clone, Component)]
pub struct Texture {
    id: usize,
    pub path: String,
    data: Option<(Vec<u8>, ImageDimensions)>, 
}

impl Texture {
    fn init(&mut self) {
        let png_bytes = fs::read(&self.path).unwrap(); 
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

        self.data = Some((image_data, dimensions));
    }

    pub fn get_sampler(device: &Arc<Device>) -> Arc<Sampler> {
        Sampler::new(
            device.clone(),
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::MipmapMode::Nearest,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            0.0, 1.0, 0.0, 0.0
        ).unwrap()
    }

    pub fn get_buffer(&self, queue: &Arc<Queue>) -> (Arc<ImageView<Arc<ImmutableImage>>>, CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>) {
        let (tex_bytes, dimensions) = self.data.as_ref().unwrap();
        
        let (image, future) = ImmutableImage::from_iter(
            tex_bytes.iter().cloned(),
            *dimensions,
            vulkano::image::MipmapsCount::One,
            Format::R8G8B8A8Srgb,
            queue.clone()
        ).unwrap();
        
        (ImageView::new(image).unwrap(), future)
    }

    pub fn get_null_buffer(queue: &Arc<Queue>) -> (Arc<ImageView<Arc<ImmutableImage>>>, CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>) {
        let png_bytes = fs::read("models/textures/null_texture.png").unwrap(); 
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

        let (image, future) = ImmutableImage::from_iter(
            image_data.iter().cloned(),
            dimensions,
            vulkano::image::MipmapsCount::One,
            Format::R8G8B8A8Srgb,
            queue.clone()
        ).unwrap();
        
        (ImageView::new(image).unwrap(), future)
    }
}

#[derive(Clone)]
pub struct EntityBuilder {
    id: usize,
    pub components: Vec<Box<dyn Component>>
}

impl EntityBuilder {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            components: vec![Box::new(Entity { id })]
        }
    }

    pub fn transform(mut self, translation: [f32; 3], scale: [f32; 3], rotation: [Deg<f32>; 3]) -> Self {
        let t = Transform {
            id: self.id,
            translation: translation.into(),
            scale: scale.into(),
            rotation: Quaternion::from(Euler::new(rotation[0], rotation[1], rotation[2]))
        };
        self.components.push(Box::new(t));

        self
    }

    pub fn logic(mut self, init: Box<fn(usize, &mut World)>, update: Box<fn(usize, &mut World)>) -> Self {
        let l = Logic {
            id: self.id,
            init,
            update
        };
        self.components.push(Box::new(l));

        self
    }

    pub fn mesh(mut self, mesh: MeshType) -> Self {
        let mut m = Mesh {
            id: self.id,
            data: MeshData::empty(),
            mesh_type: mesh
        };
        m.init();

        self.components.push(Box::new(m));

        self
    }

    pub fn texture(mut self, path: &str) -> Self {
        let mut t = Texture {
            id: self.id,
            path: path.into(),
            data: None
        };
        t.init();

        self.components.push(Box::new(t));

        self
    }
}

