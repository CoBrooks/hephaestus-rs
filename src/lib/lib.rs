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
    fn transform_rotation() {
        use entity::Transform;
        use cgmath::{ Quaternion, Vector3, Rad };
        use std::f32::consts::{ FRAC_PI_2, FRAC_PI_4 };

        let mut t = Transform::default();
        t.translation = [1.0; 3].into();

        // Is the initial rotation 0?
        let zero_q = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(t.rotation, zero_q);

        // Rotate 90deg (pi/2 rad) on x-axis
        let expected_vec = Quaternion::new(FRAC_PI_4.cos(), FRAC_PI_4.sin(), 0.0, 0.0) * Vector3::new(1.0, 1.0, 1.0);
        t.rotate([Rad(FRAC_PI_2), Rad(0.0), Rad(0.0)]);
        assert_eq!(t.rotation * t.translation, expected_vec);
    }

    #[test]
    fn transform_relative_vectors() {
        use entity::Transform;
        use cgmath::{ Quaternion, Vector3, Rad };
        use std::f32::consts::{ FRAC_PI_2, PI };

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

