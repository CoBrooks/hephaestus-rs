pub mod engine;
pub mod world;
pub mod buffer_objects;
pub mod shaders;
pub mod material;
pub mod camera;
pub mod light;
pub mod renderer;
pub mod logger;
pub mod gui;
pub mod entity;
pub mod mesh_data;
pub mod input;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_relative_vectors() {
        use entity::Transform;
        use cgmath::{ Vector3, Rad };
        use std::f32::consts::FRAC_PI_2;

        let mut t = Transform::default();

        let x_axis = Vector3::unit_x();
        let y_axis = Vector3::unit_y();
        let z_axis = Vector3::unit_z();

        let r = t.right_vector();
        let u = t.up_vector();
        let f = t.forward_vector();

        assert_eq!(r, x_axis);
        assert_eq!(u, y_axis);
        assert_eq!(f, z_axis);

        assert_eq!(r.cross(u), f);
        assert_eq!(f.cross(r), u);
        assert_eq!(u.cross(f), r);

        t.rotate([Rad(0.0), Rad(FRAC_PI_2), Rad(0.0)]);
        
        let r = t.right_vector().map(|v| v.round());
        let u = t.up_vector().map(|v| v.round());
        let f = t.forward_vector().map(|v| v.round());

        assert_eq!(r, -z_axis);
        assert_eq!(u, y_axis);
        assert_eq!(f, x_axis);

        assert_eq!(r.cross(u), f);
        assert_eq!(f.cross(r), u);
        assert_eq!(u.cross(f), r);
    }
}

