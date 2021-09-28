use std::fs;
use std::sync::Arc;
use std::io::{ BufReader, Cursor };
use cgmath::{ Euler, Deg, Quaternion, Vector3, Matrix4, Rotation };
use vulkano::image::{ ImmutableImage, ImageDimensions };
use vulkano::command_buffer::{ AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, DynamicState };
use vulkano::buffer::{ BufferUsage, CpuAccessibleBuffer };
use vulkano::format::Format;
use vulkano::device::{ Device, Queue };
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::descriptor_set::persistent::PersistentDescriptorSet;

use crate::{ 
    buffer_objects::Vertex,
    object::Viewable,
    camera::Camera,
    material::{ Diffuse, Material },
};

pub trait Primitive { 
    fn get_faces(&self) -> Vec<Plane>;
}

//{{{ Plane
pub struct Plane {
    pub origin: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub material: Box<dyn Material>,
    vertices: Vec<Vertex>,
}

#[allow(dead_code)]
impl Plane {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3]) -> Self {
        let mut p = Plane {
            origin: origin.into(),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: scale.into(),
            vertices: Vec::new(),
            material: Box::new(Diffuse { color, texture_data: None })
        };
        
        p.vertices = p.get_vertices();

        p
    }

    pub fn identity() -> Self {
        Self::new([0.0; 3], [1.0; 3], [1.0; 3])
    }

    pub fn rotate(&mut self, r: Euler<Deg<f32>>) {
        self.rotation = r.into();
    }
    
    pub fn add_texture(&mut self, tex_path: &str) {
        self.material.add_texture(tex_path);
    }

    fn get_buffers(&self, device: &Arc<Device>) -> (Arc<CpuAccessibleBuffer<[Vertex]>>, Arc<CpuAccessibleBuffer<[u16]>>) {
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

        (vertex_buffer, index_buffer)
    }
}

impl Primitive for Plane {
    fn get_faces(&self) -> Vec<Plane> {
        panic!("It's a plane...")
    }
}

impl Viewable for Plane {
    fn get_vertices(&self) -> Vec<Vertex> {
        let mut v = vec![
            Vertex { // top left
                position: [
                    self.origin.x - self.scale.x / 2.0, 
                    self.origin.y - self.scale.y / 2.0, 
                    self.origin.z
                ],
                color: self.material.get_color(),
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0]
            },
            Vertex { // top right
                position: [
                    self.origin.x + self.scale.x / 2.0, 
                    self.origin.y - self.scale.y / 2.0, 
                    self.origin.z
                ],
                color: self.material.get_color(),
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 1.0]
            },
            Vertex { // bottom right
                position: [
                    self.origin.x + self.scale.x / 2.0, 
                    self.origin.y + self.scale.y / 2.0, 
                    self.origin.z
                ],
                color: self.material.get_color(),
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 1.0]
            },
            Vertex { // bottom left
                position: [
                    self.origin.x - self.scale.x / 2.0, 
                    self.origin.y + self.scale.y / 2.0, 
                    self.origin.z
                ],
                color: self.material.get_color(),
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 0.0]
            },
        ];

        v = v.iter()
            .map(|&vert| {
                Vertex {
                    position: (self.rotation.rotate_vector(Vector3::from(vert.position) - self.origin) + self.origin).into(),
                    normal: self.rotation.rotate_vector(Vector3::from(vert.normal)).into(),
                    ..vert
                }
            })
            .collect();

        v
    }

    fn get_indices(&self) -> Vec<u16> {
        vec![
            0, 1, 2, 
            2, 3, 0
        ]
    }

    fn get_model_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.origin)
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
//}}}

//{{{ Cube
pub struct Cube {
    pub origin: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub material: Box<dyn Material>,
    faces: Vec<Plane>,
}

impl Cube {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3]) -> Self{
        let mut c = Cube {
            origin: origin.into(), 
            scale: scale.into(),
            faces: Vec::new(),
            material: Box::new(Diffuse { color, texture_data: None })
        };

        c.faces = c.get_faces();

        c
    }

    pub fn identity() -> Self {
        Self::new([0.0; 3], [1.0; 3], [1.0; 3])   
    }
}

impl Primitive for Cube {
    fn get_faces(&self) -> Vec<Plane> {
        let mut planes: Vec<Plane> = Vec::new();

        let mut top = Plane::new(
            [self.origin.x, self.origin.y, self.origin.z + self.scale.z / 2.0], 
            [self.scale.x, self.scale.y, 1.0], 
            self.material.get_color()
        );
        top.rotate(Euler { x: Deg(0.0), y: Deg(0.0), z: Deg(0.0) });

        let mut bottom = Plane::new(
            [self.origin.x, self.origin.y, self.origin.z - self.scale.z / 2.0], 
            [self.scale.x, self.scale.y, 1.0], 
            self.material.get_color()
        );
        bottom.rotate(Euler { x: Deg(180.0), y: Deg(0.0), z: Deg(0.0) });
    
        let mut front = Plane::new(
            [self.origin.x - self.scale.x / 2.0, self.origin.y, self.origin.z], 
            [self.scale.x, self.scale.y, 1.0], 
            self.material.get_color()
        );
        front.rotate(Euler { x: Deg(0.0), y: Deg(-90.0), z: Deg(0.0) });

        let mut back = Plane::new(
            [self.origin.x + self.scale.x / 2.0, self.origin.y, self.origin.z], 
            [self.scale.x, self.scale.y, 1.0], 
            self.material.get_color()
        );
        back.rotate(Euler { x: Deg(180.0), y: Deg(90.0), z: Deg(0.0) });
        
        let mut left = Plane::new(
            [self.origin.x, self.origin.y - self.scale.y / 2.0, self.origin.z], 
            [self.scale.x, self.scale.y, 1.0], 
            self.material.get_color()
        );
        left.rotate(Euler { x: Deg(90.0), y: Deg(0.0), z: Deg(90.0) });
        
        let mut right = Plane::new(
            [self.origin.x, self.origin.y + self.scale.y / 2.0, self.origin.z], 
            [self.scale.x, self.scale.y, 1.0], 
            self.material.get_color()
        );
        right.rotate(Euler { x: Deg(-90.0), y: Deg(0.0), z: Deg(-90.0) });
        
        planes.push(top);
        planes.push(bottom);
        planes.push(front);
        planes.push(back);
        planes.push(left);
        planes.push(right);

        planes
    }
}

impl Viewable for Cube {
    fn get_vertices(&self) -> Vec<Vertex> {
        self.get_faces().iter()
            .flat_map(|p| p.get_vertices())
            .collect()
    }

    fn get_indices(&self) -> Vec<u16> {
        let i_per_face: u16 = 4;

        self.get_faces()
            .iter()
            .map(|f| f.get_indices())
            .enumerate()
            .flat_map(|(index, indices)|
                indices.iter()
                .map(|i| i + i_per_face * index as u16)
                .collect::<Vec<u16>>()
            ).collect()
    }

    fn get_model_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.origin)
    }

    /*fn get_texture_data(&self) -> (Vec<u8>, vulkano::image::ImageDimensions) {
        self.texture_data.clone().unwrap()
    }*/

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
//}}}
