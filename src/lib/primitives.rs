use cgmath::{ Deg, Matrix4, Vector3 };
use crate::{ 
    buffer_objects::Vertex,
    object::{ Transform, Viewable },
    material::{ Diffuse, Material },
};

pub trait Primitive: Viewable { }

//{{{ Plane
pub struct Plane {
    pub transform: Transform,
    pub material: Box<dyn Material>,
    vertices: Vec<Vertex>,
}

#[allow(dead_code)]
impl Plane {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3]) -> Self {
        let mut p = Plane {
            transform: Transform::new(
                origin.into(),
                scale.into(),
                [Deg(0.0), Deg(0.0), Deg(0.0)]
            ), 
            vertices: Vec::new(),
            material: Box::new(Diffuse::new(color))
        };
        
        p.vertices = p.get_vertices();

        p
    }

    pub fn identity() -> Self {
        Self::new([0.0; 3], [1.0; 3], [1.0; 3])
    }
    
    pub fn add_texture(&mut self, tex_path: &str) {
        self.material.add_texture(tex_path);
    }
}

impl Primitive for Plane { }

impl Viewable for Plane {
    fn get_vertices(&self) -> Vec<Vertex> {
        vec![
            Vertex { // top left
                position: [-0.5, -0.5, 0.0],
                color: self.material.get_color(),
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 1.0]
            },
            Vertex { // top right
                position: [0.5, -0.5, 0.0],
                color: self.material.get_color(),
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 1.0]
            },
            Vertex { // bottom right
                position: [0.5, 0.5, 0.0],
                color: self.material.get_color(),
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0]
            },
            Vertex { // bottom left
                position: [-0.5, 0.5, 0.0],
                color: self.material.get_color(),
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 0.0]
            },
        ]
    }

    fn get_indices(&self) -> Vec<u16> {
        vec![
            0, 1, 2, 
            2, 3, 0
        ]
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
//}}}

//{{{ Cube
pub struct Cube {
    pub transform: Transform,
    pub material: Box<dyn Material>,
}

impl Cube {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3]) -> Self{
        Cube {
            transform: Transform::new(
                origin.into(),
                scale.into(),
                [Deg(0.0), Deg(0.0), Deg(0.0)]
            ), 
            material: Box::new(Diffuse::new(color))
        } 
    }

    pub fn identity() -> Self {
        Self::new([0.0; 3], [1.0; 3], [1.0; 3])   
    }
}

impl Viewable for Cube {
    fn get_vertices(&self) -> Vec<Vertex> {
        let color = self.material.get_color();

        vec![
            // Top
            Vertex { position: [-1.0, -1.0, 1.0],  color, normal: [0.0, 0.0, 1.0],  uv: [0.0, 0.0] }, // Back-Left
            Vertex { position: [1.0, -1.0, 1.0],   color, normal: [0.0, 0.0, 1.0],  uv: [1.0, 0.0] }, // Back-Right
            Vertex { position: [1.0, 1.0, 1.0],    color, normal: [0.0, 0.0, 1.0],  uv: [1.0, 1.0] }, // Front-Right
            Vertex { position: [-1.0, 1.0, 1.0],   color, normal: [0.0, 0.0, 1.0],  uv: [0.0, 1.0] }, // Front-Left
            // Bottom
            Vertex { position: [-1.0, -1.0, -1.0], color, normal: [0.0, 0.0, -1.0], uv: [1.0, 1.0] }, // Back-Left
            Vertex { position: [1.0, -1.0, -1.0],  color, normal: [0.0, 0.0, -1.0], uv: [0.0, 1.0] }, // Back-Right
            Vertex { position: [1.0, 1.0, -1.0],   color, normal: [0.0, 0.0, -1.0], uv: [0.0, 0.0] }, // Front-Right
            Vertex { position: [-1.0, 1.0, -1.0],  color, normal: [0.0, 0.0, -1.0], uv: [1.0, 0.0] }, // Front-Left
            // Front
            Vertex { position: [-1.0, 1.0, 1.0],   color, normal: [0.0, 1.0, 0.0], uv: [1.0, 1.0] }, // Top-Left
            Vertex { position: [1.0, 1.0, 1.0],    color, normal: [0.0, 1.0, 0.0], uv: [0.0, 1.0] }, // Top-Right
            Vertex { position: [1.0, 1.0, -1.0],   color, normal: [0.0, 1.0, 0.0], uv: [0.0, 0.0] }, // Bottom-Right
            Vertex { position: [-1.0, 1.0, -1.0],  color, normal: [0.0, 1.0, 0.0], uv: [1.0, 0.0] }, // Bottom-Left
            // Back
            Vertex { position: [-1.0, -1.0, 1.0],  color, normal: [0.0, -1.0, 0.0],  uv: [0.0, 1.0] }, // Top-Left     
            Vertex { position: [1.0, -1.0, 1.0],   color, normal: [0.0, -1.0, 0.0],  uv: [1.0, 1.0] }, // Top-Right    
            Vertex { position: [1.0, -1.0, -1.0],  color, normal: [0.0, -1.0, 0.0],  uv: [1.0, 0.0] }, // Bottom-Right 
            Vertex { position: [-1.0, -1.0, -1.0], color, normal: [0.0, -1.0, 0.0],  uv: [0.0, 0.0] }, // Bottom-Left  
            // Left
            Vertex { position: [-1.0, 1.0, 1.0],   color, normal: [-1.0, 0.0, 0.0], uv: [0.0, 1.0] }, // Top-Front     
            Vertex { position: [-1.0, -1.0, 1.0],  color, normal: [-1.0, 0.0, 0.0], uv: [1.0, 1.0] }, // Top-Back    
            Vertex { position: [-1.0, -1.0, -1.0], color, normal: [-1.0, 0.0, 0.0], uv: [1.0, 0.0] }, // Bottom-Back  
            Vertex { position: [-1.0, 1.0, -1.0],  color, normal: [-1.0, 0.0, 0.0], uv: [0.0, 0.0] }, // Bottom-Front 
            // Right
            Vertex { position: [1.0, 1.0, 1.0],    color, normal: [1.0, 0.0, 0.0], uv: [1.0, 1.0] }, // Top-Front     
            Vertex { position: [1.0, -1.0, 1.0],   color, normal: [1.0, 0.0, 0.0], uv: [0.0, 1.0] }, // Top-Back    
            Vertex { position: [1.0, -1.0, -1.0],  color, normal: [1.0, 0.0, 0.0], uv: [0.0, 0.0] }, // Bottom-Back  
            Vertex { position: [1.0, 1.0, -1.0],   color, normal: [1.0, 0.0, 0.0], uv: [1.0, 0.0] }, // Bottom-Front 
        ]
    }

    fn get_indices(&self) -> Vec<u16> {
        vec![
            0,  1,  2,  2,  3,  0,
            4,  5,  6,  6,  7,  4,
            8,  9,  10, 10, 11, 8,
            12, 15, 14, 14, 13, 12,
            16, 19, 18, 18, 17, 16,
            20, 21, 22, 22, 23, 20
        ]
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
//}}}
