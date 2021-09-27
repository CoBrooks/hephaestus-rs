use cgmath::{ Point3, Vector3, Matrix4, Rad, Deg };
use crate::buffer_objects::UniformBufferObject;

pub struct Camera {
    position: Point3<f32>,
    view: Matrix4<f32>,
    proj: Matrix4<f32>
}

impl Camera {
    pub fn new(view: Matrix4<f32>, proj: Matrix4<f32>, position: Point3<f32>) -> Self {
        Self {
            position,
            view,
            proj
        }
    }

    pub fn default(position: [f32; 3], dimensions: [u32; 2]) -> Self {
        let mut proj = cgmath::perspective(Rad::from(Deg(45.0)), dimensions[0] as f32 / dimensions[1] as f32, 0.1, 10.0);
        proj.y.y *= -1.0;

        let view = Matrix4::look_at_rh(
            position.into(),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0) // "up vector"
        );

        Self {
            position: position.into(),
            view,
            proj
        }
    }

    pub fn get_ubo(&self, model: Matrix4<f32>) -> UniformBufferObject {
        UniformBufferObject {
            model,
            view: self.view,
            proj: self.proj
        }
    }
}
