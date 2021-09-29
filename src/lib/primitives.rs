use cgmath::{ Deg, Vector3, InnerSpace };
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
    indices: Vec<u16>
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
            indices: Vec::new(),
            material: Box::new(Diffuse::new(color))
        };
        p.load_data();

        p
    }

    pub fn identity() -> Self {
        Self::new([0.0; 3], [1.0; 3], [1.0; 3])
    }

    pub fn load_data(&mut self) {
        self.vertices = vec![
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
        ];

        self.indices = vec![
            0, 1, 2, 
            2, 3, 0
        ];

        println!("Created plane.")
    }
    
    pub fn add_texture(&mut self, tex_path: &str) {
        self.material.add_texture(tex_path);
    }
}

impl Primitive for Plane { }
impl Viewable for Plane {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<u16> {
        &self.indices
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
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Cube {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3]) -> Self{
        let mut c = Cube {
            transform: Transform::new(
                origin.into(),
                scale.into(),
                [Deg(0.0), Deg(0.0), Deg(0.0)]
            ), 
            material: Box::new(Diffuse::new(color)),
            vertices: Vec::new(),
            indices: Vec::new()
        };
        c.load_data();

        c
    }

    pub fn identity() -> Self {
        Self::new([0.0; 3], [1.0; 3], [1.0; 3])   
    }

    fn load_data(&mut self) {
        let color = self.material.get_color();

        self.vertices = vec![
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
        ];

        self.indices = vec![
            0,  1,  2,  2,  3,  0,
            4,  5,  6,  6,  7,  4,
            8,  9,  10, 10, 11, 8,
            12, 15, 14, 14, 13, 12,
            16, 19, 18, 18, 17, 16,
            20, 21, 22, 22, 23, 20
        ];

        println!("Created cube");
    }
}

impl Viewable for Cube {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<u16> {
        &self.indices
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

//{{{ Sphere
pub struct Sphere {
    pub transform: Transform,
    pub material: Box<dyn Material>,
    resolution: u8,
    vertices: Vec<Vertex>,
    indices: Vec<u16>
}

impl Sphere {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3], resolution: u8) -> Self{
        let mut s = Sphere {
            transform: Transform::new(
                origin.into(),
                scale.into(),
                [Deg(0.0), Deg(0.0), Deg(0.0)]
            ), 
            material: Box::new(Diffuse::new(color)),
            resolution,
            vertices: Vec::new(),
            indices: Vec::new()
        };
        s.load_data();

        s
    }

    pub fn identity() -> Self {
        Self::new([0.0; 3], [1.0; 3], [1.0; 3], 3)   
    }

    fn load_data(&mut self) { 
        let t = (1.0 + (5.0f32).sqrt()) / 2.0;
        let mut v: Vec<Vector3<f32>> = Vec::new();
        let mut i: Vec<u16> = Vec::new();

        // Initial vertices
        v.append(&mut vec![
            [-1.0, t, 0.0].into(),
            [1.0, t, 0.0].into(),
            [-1.0, -t, 0.0].into(),
            [1.0, -t, 0.0].into(),
            [0.0, -1.0, t].into(),
            [0.0, 1.0, t].into(),
            [0.0, -1.0, -t].into(),
            [0.0, 1.0, -t].into(),
            [t, 0.0, -1.0].into(),
            [t, 0.0, 1.0].into(),
            [-t, 0.0, -1.0].into(),
            [-t, 0.0, 1.0].into(),
        ]);
        // Put all vertices on unit sphere
        v = v.iter().map(|vertex| vertex.normalize()).collect();

        // Initial faces
        i.append(&mut vec![
            0, 11, 5,
            0, 5, 1,
            0, 1, 7,
            0, 7, 10,
            0, 10, 11,
            1, 5, 9,
            5, 11, 4,
            11, 10, 2, 
            10, 7, 6,
            7, 1, 8,
            3, 9, 4, 
            3, 4, 2, 
            3, 2, 6, 
            3, 6, 8,
            3, 8, 9,
            4, 9, 5,
            2, 4, 11, 
            6, 2, 10, 
            8, 6, 7,
            9, 8, 1
        ]);

        let mut last_index = 11;
        for _ in 0..self.resolution {
        let mut new_indices: Vec<u16> = Vec::new();
            for face in i.clone().chunks(3) {
                let mut new_points: Vec<Vector3<f32>> = vec![
                    (v[face[0] as usize] + v[face[1] as usize]) / 2.0,
                    (v[face[1] as usize] + v[face[2] as usize]) / 2.0,
                    (v[face[2] as usize] + v[face[0] as usize]) / 2.0 
                ];
                new_points = new_points.iter().map(|vertex| vertex.normalize()).collect();

                v.append(&mut new_points);
                new_indices.append(&mut vec![
                    face[0], last_index + 1, last_index + 3,
                    last_index + 1, face[1], last_index + 2,
                    last_index + 1, last_index + 2, last_index + 3,
                    last_index + 3, last_index + 2, face[2]
                ]);
                last_index += 3;
            }
            i = new_indices;
        }

        let color = self.material.get_color();
        self.vertices = v.into_iter().map(|v| {
            Vertex {
                position: v.into(),
                color,
                uv: [
                    v.z.atan2(v.x) / std::f32::consts::TAU,
                    (v.y.asin() / std::f32::consts::PI) + 0.5,
                ], // https://www.alexisgiard.com/icosahedron-sphere/
                normal: v.normalize().into() // smooth shading
            }
        }).collect();
        self.indices = i;

        println!("Created sphere (resolution = {:?}, vertices = {:?})", self.resolution, self.vertices.len());
    }
}

impl Viewable for Sphere {
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<u16> {
        &self.indices
    }

    fn get_material(&self) -> &Box<dyn Material> {
        &self.material
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
