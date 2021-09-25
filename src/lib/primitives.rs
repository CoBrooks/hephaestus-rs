use cgmath::{ Deg, Euler, Quaternion, Vector3, Rotation };

use crate::{ 
    buffer_objects::Vertex,
    object::Viewable
};

pub trait Primitive { 
    fn get_faces(&self) -> Vec<Plane>;
}

//{{{ Plane
#[derive(Clone)]
pub struct Plane {
    pub origin: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub color: [f32; 3],
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
            color,
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
}

impl Primitive for Plane {
    fn get_faces(&self) -> Vec<Plane> {
        vec![
            self.to_owned()   
        ]
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
                color: self.color,
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0]
            },
            Vertex { // top right
                position: [
                    self.origin.x + self.scale.x / 2.0, 
                    self.origin.y - self.scale.y / 2.0, 
                    self.origin.z
                ],
                color: self.color,
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0]
            },
            Vertex { // bottom right
                position: [
                    self.origin.x + self.scale.x / 2.0, 
                    self.origin.y + self.scale.y / 2.0, 
                    self.origin.z
                ],
                color: self.color,
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0]
            },
            Vertex { // bottom left
                position: [
                    self.origin.x - self.scale.x / 2.0, 
                    self.origin.y + self.scale.y / 2.0, 
                    self.origin.z
                ],
                color: self.color,
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0]
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
}
//}}}

//{{{ Cube
pub struct Cube {
    pub origin: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub color: [f32; 3],
    faces: Vec<Plane>,
}

impl Cube {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3]) -> Self{
        let mut c = Cube {
            origin: origin.into(), 
            scale: scale.into(),
            faces: Vec::new(),
            color
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
            self.color
        );
        top.rotate(Euler { x: Deg(0.0), y: Deg(0.0), z: Deg(0.0) });

        let mut bottom = Plane::new(
            [self.origin.x, self.origin.y, self.origin.z - self.scale.z / 2.0], 
            [self.scale.x, self.scale.y, 1.0], 
            self.color
        );
        bottom.rotate(Euler { x: Deg(180.0), y: Deg(0.0), z: Deg(0.0) });
    
        let mut front = Plane::new(
            [self.origin.x - self.scale.x / 2.0, self.origin.y, self.origin.z], 
            [self.scale.x, self.scale.y, 1.0], 
            self.color
        );
        front.rotate(Euler { x: Deg(0.0), y: Deg(90.0), z: Deg(0.0) });

        let mut back = Plane::new(
            [self.origin.x + self.scale.x / 2.0, self.origin.y, self.origin.z], 
            [self.scale.x, self.scale.y, 1.0], 
            self.color
        );
        back.rotate(Euler { x: Deg(0.0), y: Deg(-90.0), z: Deg(0.0) });
        
        let mut left = Plane::new(
            [self.origin.x, self.origin.y - self.scale.y / 2.0, self.origin.z], 
            [self.scale.x, self.scale.y, 1.0], 
            self.color
        );
        left.rotate(Euler { x: Deg(90.0), y: Deg(0.0), z: Deg(0.0) });
        
        let mut right = Plane::new(
            [self.origin.x, self.origin.y + self.scale.y / 2.0, self.origin.z], 
            [self.scale.x, self.scale.y, 1.0], 
            self.color
        );
        right.rotate(Euler { x: Deg(-90.0), y: Deg(0.0), z: Deg(0.0) });
        
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
}
//}}}
