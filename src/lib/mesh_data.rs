use std::fs::File;
use std::io::BufReader;
use cgmath::{ InnerSpace, Vector3 };
use obj::{ Obj, TexturedVertex, load_obj };

use crate::{ 
    buffer_objects::Vertex,
    logger::{ self, MessageEmitter }
};

#[derive(Clone)]
pub enum PrimitiveType {
    Plane,
    Cube,
    Sphere(u8)
}

#[derive(Clone)]
pub enum MeshType {
    Model(String),
    Primitive(PrimitiveType)
}

#[derive(Clone)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

impl MeshData {
    pub fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }

    pub fn load(path: &str) -> MeshData {
        if let Some(file) = File::open(path).ok() {
            let input = BufReader::new(file);

            if let Some(object) = load_obj(input).ok() as Option<Obj<TexturedVertex, u16>> {
                let mut data = MeshData::empty();
                data.indices = object.indices;
                
                data.vertices = object.vertices.iter()
                    .map(|v| Vertex {
                        position: v.position,
                        normal: v.normal,
                        color: [1.0; 3],
                        uv: [v.texture[0], v.texture[1]]
                    })
                    .collect();

                data
            } else {
                logger::log_error(&format!("Unable to load object data from '{}'", path), MessageEmitter::World);
                MeshData::empty()
            }
        } else {
            logger::log_error(&format!("Unable to read '{}'", path), MessageEmitter::World);
            MeshData::empty()
        }
    }
    
    pub fn generate(mesh_type: PrimitiveType) -> MeshData {
        match mesh_type {
            PrimitiveType::Plane => {
                Self::generate_plane()
            },
            PrimitiveType::Cube => {
                Self::generate_cube()
            },
            PrimitiveType::Sphere(resolution) => {
                Self::generate_sphere(resolution)
            }
        }
    }

    fn generate_plane() -> MeshData {
        let mut data = MeshData::empty();

        data.vertices = vec![
            Vertex { // top left
                position: [-0.5, 0.0, 0.5],
                color: [1.0; 3],
                normal: [0.0, 1.0, 0.0],
                uv: [0.0, 0.0]
            },
            Vertex { // top right
                position: [0.5, 0.0, 0.5],
                color: [1.0; 3],
                normal: [0.0, 1.0, 0.0],
                uv: [1.0, 0.0]
            },
            Vertex { // bottom right
                position: [0.5, 0.0, -0.5],
                color: [1.0; 3],
                normal: [0.0, 1.0, 0.0],
                uv: [0.0, 1.0]
            },
            Vertex { // bottom left
                position: [-0.5, 0.0, -0.5],
                color: [1.0; 3],
                normal: [0.0, 1.0, 0.0],
                uv: [1.0, 1.0]
            },
        ];

        data.indices = vec![
            0, 1, 2, 
            2, 3, 0
        ];

        data
    }

    fn generate_cube() -> MeshData { 
        let mut data = MeshData::empty();

        data.vertices = vec![
            // Top
            Vertex { position: [-1.0, -1.0, 1.0],  color: [1.0; 3], normal: [0.0, 0.0, 1.0],  uv: [0.0, 0.0] }, // Back-Left
            Vertex { position: [1.0, -1.0, 1.0],   color: [1.0; 3], normal: [0.0, 0.0, 1.0],  uv: [1.0, 0.0] }, // Back-Right
            Vertex { position: [1.0, 1.0, 1.0],    color: [1.0; 3], normal: [0.0, 0.0, 1.0],  uv: [1.0, 1.0] }, // Front-Right
            Vertex { position: [-1.0, 1.0, 1.0],   color: [1.0; 3], normal: [0.0, 0.0, 1.0],  uv: [0.0, 1.0] }, // Front-Left
            // Bottom
            Vertex { position: [-1.0, -1.0, -1.0], color: [1.0; 3], normal: [0.0, 0.0, -1.0], uv: [1.0, 1.0] }, // Back-Left
            Vertex { position: [1.0, -1.0, -1.0],  color: [1.0; 3], normal: [0.0, 0.0, -1.0], uv: [0.0, 1.0] }, // Back-Right
            Vertex { position: [1.0, 1.0, -1.0],   color: [1.0; 3], normal: [0.0, 0.0, -1.0], uv: [0.0, 0.0] }, // Front-Right
            Vertex { position: [-1.0, 1.0, -1.0],  color: [1.0; 3], normal: [0.0, 0.0, -1.0], uv: [1.0, 0.0] }, // Front-Left
            // Front
            Vertex { position: [-1.0, 1.0, 1.0],   color: [1.0; 3], normal: [0.0, 1.0, 0.0], uv: [1.0, 1.0] }, // Top-Left
            Vertex { position: [1.0, 1.0, 1.0],    color: [1.0; 3], normal: [0.0, 1.0, 0.0], uv: [0.0, 1.0] }, // Top-Right
            Vertex { position: [1.0, 1.0, -1.0],   color: [1.0; 3], normal: [0.0, 1.0, 0.0], uv: [0.0, 0.0] }, // Bottom-Right
            Vertex { position: [-1.0, 1.0, -1.0],  color: [1.0; 3], normal: [0.0, 1.0, 0.0], uv: [1.0, 0.0] }, // Bottom-Left
            // Back
            Vertex { position: [-1.0, -1.0, 1.0],  color: [1.0; 3], normal: [0.0, -1.0, 0.0],  uv: [0.0, 1.0] }, // Top-Left     
            Vertex { position: [1.0, -1.0, 1.0],   color: [1.0; 3], normal: [0.0, -1.0, 0.0],  uv: [1.0, 1.0] }, // Top-Right    
            Vertex { position: [1.0, -1.0, -1.0],  color: [1.0; 3], normal: [0.0, -1.0, 0.0],  uv: [1.0, 0.0] }, // Bottom-Right 
            Vertex { position: [-1.0, -1.0, -1.0], color: [1.0; 3], normal: [0.0, -1.0, 0.0],  uv: [0.0, 0.0] }, // Bottom-Left  
            // Left
            Vertex { position: [-1.0, 1.0, 1.0],   color: [1.0; 3], normal: [-1.0, 0.0, 0.0], uv: [0.0, 1.0] }, // Top-Front     
            Vertex { position: [-1.0, -1.0, 1.0],  color: [1.0; 3], normal: [-1.0, 0.0, 0.0], uv: [1.0, 1.0] }, // Top-Back    
            Vertex { position: [-1.0, -1.0, -1.0], color: [1.0; 3], normal: [-1.0, 0.0, 0.0], uv: [1.0, 0.0] }, // Bottom-Back  
            Vertex { position: [-1.0, 1.0, -1.0],  color: [1.0; 3], normal: [-1.0, 0.0, 0.0], uv: [0.0, 0.0] }, // Bottom-Front 
            // Right
            Vertex { position: [1.0, 1.0, 1.0],    color: [1.0; 3], normal: [1.0, 0.0, 0.0], uv: [1.0, 1.0] }, // Top-Front     
            Vertex { position: [1.0, -1.0, 1.0],   color: [1.0; 3], normal: [1.0, 0.0, 0.0], uv: [0.0, 1.0] }, // Top-Back    
            Vertex { position: [1.0, -1.0, -1.0],  color: [1.0; 3], normal: [1.0, 0.0, 0.0], uv: [0.0, 0.0] }, // Bottom-Back  
            Vertex { position: [1.0, 1.0, -1.0],   color: [1.0; 3], normal: [1.0, 0.0, 0.0], uv: [1.0, 0.0] }, // Bottom-Front 
        ];

        data.indices = vec![
            0,  1,  2,  2,  3,  0,
            4,  5,  6,  6,  7,  4,
            8,  9,  10, 10, 11, 8,
            12, 15, 14, 14, 13, 12,
            16, 19, 18, 18, 17, 16,
            20, 21, 22, 22, 23, 20
        ];

        data
    }
    
    fn generate_sphere(resolution: u8) -> MeshData { 
        let mut data = MeshData::empty();

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
        for _ in 0..resolution {
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

        data.vertices = v.into_iter().map(|v| {
            Vertex {
                position: v.into(),
                color: [1.0; 3],
                uv: [
                    v.z.atan2(v.x) / std::f32::consts::TAU,
                    (v.y.asin() / std::f32::consts::PI) + 0.5,
                ], // https://www.alexisgiard.com/icosahedron-sphere/
                normal: v.normalize().into() // smooth shading
            }
        }).collect();
        data.indices = i;
        
        data
    }
}

