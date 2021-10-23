use hephaestus_macros::*;
use std::sync::Arc;
use std::io::Cursor;
use std::fs;
use downcast_rs::{ Downcast, impl_downcast };
use cgmath::{ Vector3, Point3, Matrix4, Quaternion, Euler, Deg, Rad, Rotation3, Rotation, SquareMatrix, InnerSpace };
use vulkano::image::{ ImageDimensions, ImmutableImage, view::ImageView };
use vulkano::sampler::Sampler;
use vulkano::sync::GpuFuture;
use vulkano::device::{ Device, Queue };
use vulkano::format::Format;
use vulkano::buffer::{ BufferUsage, CpuAccessibleBuffer };

use crate::{
    mesh_data::{ MeshData, MeshType },
    world::World,
    engine::EngineTime,
    input::Input,
    camera::Camera,
    // logger::{ self, MessageEmitter }
};

pub trait Component: Downcast + ComponentClone { 
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
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

#[derive(Clone, Component)]
pub struct Entity {
    id: usize,
}

#[derive(Clone, Component)]
pub struct Transform {
    id: usize,
    pub translation: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub local_rotation: Quaternion<f32>,
}

impl Transform {
    pub fn default() -> Self {
        Self {
            id: 0,
            translation: [0.0; 3].into(),
            scale: [1.0; 3].into(),
            rotation: Quaternion::from(Euler::new(Deg(0.0), Deg(0.0), Deg(0.0))),
            local_rotation: Quaternion::from(Euler::new(Deg(0.0), Deg(0.0), Deg(0.0))),
        }
    }

    pub fn translate(&mut self, translation: [f32; 3]) {
        self.translation += Vector3::from(translation);
    }

    pub fn translate_local(&mut self, translation: [f32; 3]) {
        let (x, y, z) = (translation[0], translation[1], translation[2]);

        self.translation += 
            self.right_vector() * x +
            self.up_vector() * y +
            self.forward_vector() * z;
    }

    pub fn rotate(&mut self, rotation: [Rad<f32>; 3]) {
        let (x, y, z) = (rotation[0], rotation[1], rotation[2]);
        
        self.rotation = Quaternion::from(Euler::new(x, y, z)) * self.rotation;
    }

    pub fn rotate_local(&mut self, rotation: [Rad<f32>; 3]) {
        let (x, y, z) = (rotation[0], rotation[1], rotation[2]);
        
        self.local_rotation = Quaternion::from(Euler::new(x, y, z)) * self.local_rotation;
    }

    pub fn scale(&mut self, scale: [f32; 3]) {
        self.scale = Vector3::new(self.scale.x * scale[0], self.scale.y * scale[1], self.scale.z * scale[2]);
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        // https://solarianprogrammer.com/2013/05/22/opengl-101-matrices-projection-view-model/
        let (x, y, z) = self.translation.into();
        let t = Matrix4::from_cols(
            [1.0, 0.0, 0.0, 0.0].into(),
            [0.0, 1.0, 0.0, 0.0].into(),
            [0.0, 0.0, 1.0, 0.0].into(),
            [x, y, z, 1.0].into(),
        );

        let (sx, sy, sz) = self.scale.into();
        let s = Matrix4::from_cols(
            [sx, 0.0, 0.0, 0.0].into(),
            [0.0, sy, 0.0, 0.0].into(),
            [0.0, 0.0, sz, 0.0].into(),
            [0.0, 0.0, 0.0, 1.0].into(),
        );

        let global_r = Matrix4::from(self.rotation);
        let local_r = Matrix4::from(self.local_rotation);

        t * global_r * local_r * s
    }

    pub fn forward_vector(&self) -> Vector3<f32> {
        (Matrix4::from(self.rotation) * Matrix4::from(self.local_rotation) * Vector3::unit_z().extend(1.0)).truncate()
    }

    pub fn right_vector(&self) -> Vector3<f32> {
        (Matrix4::from(self.rotation) * Matrix4::from(self.local_rotation) * Vector3::unit_x().extend(1.0)).truncate()
    }
    
    pub fn up_vector(&self) -> Vector3<f32> {
        (Matrix4::from(self.rotation) * Matrix4::from(self.local_rotation) * Vector3::unit_y().extend(1.0)).truncate()
    }
}

#[derive(Clone, Component)]
pub struct Logic {
    id: usize,
    pub init: Box<fn(usize, &mut World)>,
    pub update: Box<fn(usize, &mut UpdateData)>
}

pub struct UpdateData<'a> {
    pub world: &'a mut World,
    pub time: &'a EngineTime,
    pub input: &'a Input
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
    bytes: Vec<u8>,
    dimensions: ImageDimensions,
    buffer: Option<Arc<CpuAccessibleBuffer<[u8]>>>
}

impl Texture {
    fn init(&mut self) {
        let png_bytes = fs::read(&self.path).unwrap(); 
        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();

        self.dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1
        };

        self.bytes.resize((info.width * info.height * 4) as usize, 0);
        reader.next_frame(&mut self.bytes).unwrap();
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

    pub unsafe fn get_buffer(&mut self, queue: &Arc<Queue>) -> (Arc<ImageView<Arc<ImmutableImage>>>, Box<dyn GpuFuture>) {
        let buffer = if let Some(b) = &self.buffer {
            b.clone()
        } else {
            let buffer: Arc<CpuAccessibleBuffer<[u8]>> = CpuAccessibleBuffer::uninitialized_array(
                queue.device().clone(),
                (self.dimensions.width() * self.dimensions.height() * 4) as u64,
                BufferUsage::transfer_source(),
                true
            ).unwrap();

            { // New scope to "fool" the borrow-checker (can't borrow `buffer` as mutable and immutable in the same scope)
                let mut mapping = buffer.write().unwrap();
                mapping.copy_from_slice(self.bytes.as_slice());
            }

            self.buffer = Some(buffer.clone());

            buffer
        };

        let (image, future) = ImmutableImage::from_buffer(
            buffer,
            self.dimensions,
            vulkano::image::MipmapsCount::One,
            Format::R8G8B8A8Srgb,
            queue.clone()
        ).unwrap();

        (ImageView::new(image).unwrap(), future.boxed())
    }

    pub fn get_null_buffer(queue: &Arc<Queue>) -> (Arc<ImageView<Arc<ImmutableImage>>>, Box<dyn GpuFuture>) {
        let (image, future) = ImmutableImage::from_iter(
            [255u8, 255, 255, 0].iter().cloned(),
            ImageDimensions::Dim2d { width: 1, height: 1, array_layers: 1},
            vulkano::image::MipmapsCount::One,
            Format::R8G8B8A8Srgb,
            queue.clone()
        ).unwrap();
        
        (ImageView::new(image).unwrap(), future.boxed())
    }
}

#[derive(Clone)]
pub struct EntityBuilder {
    pub components: Vec<Box<dyn Component>>
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            components: vec![Box::new(Entity { id: 0 })]
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.components.iter_mut()
            .for_each(|c| c.set_id(id));
    }

    pub fn transform(mut self, translation: [f32; 3], scale: [f32; 3], rotation: [Deg<f32>; 3]) -> Self {
        let rotation = Quaternion::from(Euler::new(rotation[0], rotation[1], rotation[2]));

        let t = Transform {
            id: 0,
            translation: translation.into(),
            scale: scale.into(),
            rotation,
            local_rotation: rotation,
        };
        self.components.push(Box::new(t));

        self
    }

    pub fn logic(mut self, init: Box<fn(usize, &mut World)>, update: Box<fn(usize, &mut UpdateData)>) -> Self {
        let l = Logic {
            id: 0,
            init,
            update
        };
        self.components.push(Box::new(l));

        self
    }

    pub fn mesh(mut self, mesh: MeshType) -> Self {
        let mut m = Mesh {
            id: 0,
            data: MeshData::empty(),
            mesh_type: mesh
        };
        m.init();

        self.components.push(Box::new(m));

        self
    }

    pub fn material(mut self, color: [f32; 3]) -> Self {
        let m = Material {
            id: 0,
            color
        };

        self.components.push(Box::new(m));

        self
    }

    pub fn texture(mut self, path: &str) -> Self {
        let mut t = Texture {
            id: 0,
            path: path.into(),
            bytes: Vec::new(),
            dimensions: ImageDimensions::Dim2d { width: 0, height: 0, array_layers: 0 },
            buffer: None
        };
        t.init();

        self.components.push(Box::new(t));

        self
    }

    pub fn camera(mut self) -> Self {
        let c = Camera::default();

        self.components.push(Box::new(c));

        self
    }
}

