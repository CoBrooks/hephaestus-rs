use cgmath::Deg;
use crate::{ 
    buffer_objects::Vertex,
    object::{ Transform, Viewable },
    material::{ Diffuse, Material },
};

pub trait Primitive { 
    fn get_faces(&self) -> Vec<Plane>;
}

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

impl Primitive for Plane {
    fn get_faces(&self) -> Vec<Plane> {
        panic!("It's a plane...")
    }
}

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
    faces: Vec<Plane>,
}

impl Cube {
    pub fn new(origin: [f32; 3], scale: [f32; 3], color: [f32; 3]) -> Self{
        let mut c = Cube {
            transform: Transform::new(
                origin.into(),
                scale.into(),
                [Deg(0.0), Deg(0.0), Deg(0.0)]
            ), 
            faces: Vec::new(),
            material: Box::new(Diffuse::new(color))
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

        let mut top = Plane::new([0.0, 0.0, 0.0], [1.0; 3], [1.0, 0.0, 0.0]);
        top.transform.translate([0.0, 0.0, 0.5]);
        top.transform.rotate([Deg(0.0), Deg(0.0), Deg(0.0)]);

        let mut bottom = Plane::new([0.0, 0.0, 0.0], [1.0; 3], [0.0, 1.0, 0.0]);
        bottom.transform.translate([0.0, 0.0, 0.6]);
        bottom.transform.rotate([Deg(0.0), Deg(0.0), Deg(0.0)]);
    
        let mut front = Plane::new(
            [self.transform.translation.x - self.transform.scale.x / 2.0, self.transform.translation.y, self.transform.translation.z], 
            [self.transform.scale.x, self.transform.scale.y, 1.0], 
            self.material.get_color()
        );
        front.transform.rotate([Deg(0.0), Deg(90.0), Deg(0.0)]);

        let mut back = Plane::new(
            [self.transform.translation.x + self.transform.scale.x / 2.0, self.transform.translation.y, self.transform.translation.z], 
            [self.transform.scale.x, self.transform.scale.y, 1.0], 
            self.material.get_color()
        );
        back.transform.rotate([Deg(180.0), Deg(90.0), Deg(0.0)]);
        
        let mut left = Plane::new(
            [self.transform.translation.x, self.transform.translation.y - self.transform.scale.y / 2.0, self.transform.translation.z], 
            [self.transform.scale.x, self.transform.scale.y, 1.0], 
            self.material.get_color()
        );
        left.transform.rotate([Deg(90.0), Deg(0.0), Deg(90.0)]);
        
        let mut right = Plane::new(
            [self.transform.translation.x, self.transform.translation.y + self.transform.scale.y / 2.0, self.transform.translation.z], 
            [self.transform.scale.x, self.transform.scale.y, 1.0], 
            self.material.get_color()
        );
        right.transform.rotate([Deg(-90.0), Deg(0.0), Deg(-90.0)]);
        
        planes.push(top);
        planes.push(bottom);
        //planes.push(front);
        //planes.push(back);
        //planes.push(left);
        //planes.push(right);

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
