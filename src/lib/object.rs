use std::fs;
use std::fs::File;
use std::sync::Arc;
use std::io::{ BufReader, Cursor };
use obj::{ load_obj, Obj, TexturedVertex };
use cgmath::{ Euler, Deg, Quaternion, Vector3, Matrix4 };
use vulkano::image::{ ImmutableImage, ImageDimensions };
use vulkano::command_buffer::{ AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, DynamicState };
use vulkano::buffer::{ BufferUsage, CpuAccessibleBuffer };
use vulkano::format::Format;
use vulkano::device::{ Device, Queue };
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::descriptor_set::persistent::PersistentDescriptorSet;

use crate::{
    buffer_objects::Vertex,
    material::{ Diffuse, Material },
    shaders::{ VertShader, FragShader },
    camera::Camera
};

pub trait Viewable {
    fn get_indices(&self) -> Vec<u16>;
    fn get_vertices(&self) -> Vec<Vertex>;
    fn add_to_render_commands(&self, 
                              device: &Arc<Device>, 
                              queue: &Arc<Queue>,
                              dimensions: [u32; 2], 
                              color_format: Format, 
                              dynamic_state: &DynamicState, 
                              camera: &Camera,
                              commands: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>); 
}

pub struct Object {
    pub origin: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub model_path: String,
    pub material: Box<dyn Material>,
    object_data: Obj<TexturedVertex, u16>,
}

impl Object {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3], model_path: String) -> Self {
        let data = Object::get_object_data(&model_path);

        Object {
            origin: origin.into(),
            scale: scale.into(),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            model_path,
            material: Box::new(Diffuse { 
                texture_data: None,
                color
            }),
            object_data: data
        }
    }

    fn get_object_data(model_path: &str) -> Obj<TexturedVertex, u16> {
        let input = BufReader::new(File::open(&model_path).expect(&format!("Error loading model file: {}", model_path)));
        load_obj(input).expect(&format!("Error reading model data: {}", model_path))
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
                color: self.material.get_color(),
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

    fn add_to_render_commands(&self, 
                              device: &Arc<Device>, 
                              queue: &Arc<Queue>,
                              dimensions: [u32; 2], 
                              color_format: Format, 
                              dynamic_state: &DynamicState, 
                              camera: &Camera,
                              commands: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) 
    {
        let pipeline = self.material.get_pipeline(device, dimensions, color_format);

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            self.get_vertices().iter().cloned(),
        ).unwrap();
      
        let index_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            self.get_indices().iter().cloned()
        ).unwrap();

        let ubo = camera.get_ubo(Matrix4::from_translation(self.origin));
        let ubo_layout = pipeline.layout().descriptor_set_layouts()[0].clone();
        let ubo_buffer = CpuAccessibleBuffer::from_data(
            device.clone(),
            BufferUsage::uniform_buffer_transfer_destination(),
            false,
            ubo
        ).unwrap();
        let ubo_set = Arc::new(
            PersistentDescriptorSet::start(ubo_layout.clone())
                .add_buffer(ubo_buffer)
                .unwrap()
                .build()
                .unwrap()
        );

        let (texture, _) = self.material.get_texture_buffer(queue);
        let sampler = self.material.get_texture_sampler(device);
        let layout = pipeline.layout().descriptor_set_layouts()[1].clone();
        let tex_set = Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_sampled_image(texture.clone(), sampler)
                .unwrap()
                .build()
                .unwrap()
        );

        commands
            .draw_indexed(
                pipeline.clone(),
                &dynamic_state,
                vertex_buffer.clone(),
                index_buffer.clone(),
                (ubo_set.clone(), tex_set.clone()),
                (),
            ).unwrap();
    }
}
