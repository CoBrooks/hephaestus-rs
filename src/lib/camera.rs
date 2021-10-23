use hephaestus_macros::*;

use cgmath::{ Point3, Euler, Matrix4, Vector3, Rad, Deg, SquareMatrix, InnerSpace, EuclideanSpace };
use crate::{
    buffer_objects::{ VPBufferObject, UniformBufferObject },
    entity::{ Component, Transform },
};

#[derive(Clone, Component)]
pub struct Camera {
    id: usize,
    pub view: Matrix4<f32>,
    pub proj: Matrix4<f32>
}

impl Camera {
    pub fn default() -> Self {
        Self {
            id: 0,
            view: Matrix4::from([[0.0; 4]; 4]),
            proj: Matrix4::from([[0.0; 4]; 4]),
        }
    }

    pub fn get_ubo(&self, model: Matrix4<f32>) -> UniformBufferObject {
        UniformBufferObject {
            model,
            view: self.view,
            proj: self.proj
        }
    }

    pub fn calculate_view(&mut self, position: &Transform) {
        let (x, y, z) = position.translation.into();

        self.view = Matrix4::look_at_rh(
            Point3::new(x, y, z), 
            Point3::new(x, y, z) + position.forward_vector(),
            position.up_vector()
        );
    }

    pub fn get_vp_buffer(&self, dimensions: [u32; 2]) -> VPBufferObject {
        let mut proj = cgmath::perspective(Rad::from(Deg(60.0)), dimensions[0] as f32 / dimensions[1] as f32, 0.1, 10.0);
        proj.y.y *= -1.0;

        VPBufferObject {
            view: self.view,
            proj
        }
    }
}

pub mod logic {
    use crate::entity::{ Transform, UpdateData };
    use crate::logger::{ self, MessageEmitter };
    use cgmath::{ Rad, Euler };
    use winit::event::VirtualKeyCode;

    pub fn first_person<const SENS: u8, const SPEED: u8>() -> Box<fn(usize, &mut UpdateData)> {
        Box::new(|id: usize, data: &mut UpdateData| {
            let transform = data.world.get_component_by_id_mut::<Transform>(id).unwrap();
            let (d_x, d_y) = data.input.mouse_delta();

            transform.rotate([Rad(0.0), -Rad(SENS as f32 * d_x / 500.0), Rad(0.0)]);
            transform.rotate_local([Rad(SENS as f32 * d_y / 500.0), Rad(0.0), Rad(0.0)]);

            if data.input.get_key_down(VirtualKeyCode::Space) {
                logger::log_debug(&format!("global: {:?}", Euler::from(transform.rotation)), MessageEmitter::Object("camera".into()));
                logger::log_debug(&format!("local: {:?}", Euler::from(transform.local_rotation)), MessageEmitter::Object("camera".into()));
            }
        })
    }
}

