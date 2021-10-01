use std::sync::Arc;
use cgmath::Deg;

use hephaestus_lib::{
    engine::{ Engine, EngineTime },
    world::World,
    primitives::{ Plane, Cube, Sphere },
    object::*,
    camera::Camera,
    light::DirectionalLight,
};

fn main() {
    let mut world = World::new(Camera::default([2.0, 2.0, 2.0]));
    
    let mut monkey_1 = Object::new([0.0, 0.0, 0.0], [0.5; 3], [1.0; 3], "models/suzanne.obj".into());
    monkey_1.material.add_texture("models/textures/monkey_texture.png");
    monkey_1.add_update(Box::new(|m: &mut Object, _: &World, time: &EngineTime| {
        m.transform.rotate([Deg(0.0), Deg(0.0), Deg(1.0)]);

        let r = time.total_time.sin().abs();
        let g = (time.total_time + 2.09).sin().abs();
        let b = (time.total_time + 4.18).sin().abs();
        let new_color = [r, g, b];
        
        m.material.set_color(new_color);
        m.transform.scale = [r; 3].into();
    }));
    
    let mut cube_1 = Cube::identity();
    cube_1.add_update(Box::new(extern_update));

    let white_light = DirectionalLight::new([1.0, 2.0, 1.0, 1.0], [0.5, 0.5, 0.5]);
  
    world.add_object(Box::new(cube_1));
    
    world.add_light(white_light);

    let engine = Engine::initialize(world);
    engine.start();
}

fn extern_update(object: &mut Cube, _: &World, time: &EngineTime) {
    object.transform.rotate([Deg(0.0), Deg(0.0), Deg(1.0)]);

    let r = time.total_time.sin().abs();
    let g = (time.total_time + 2.09).sin().abs();
    let b = (time.total_time + 4.18).sin().abs();
    let new_color = [r, g, b];
    object.material.set_color(new_color);
    object.transform.scale = [r; 3].into();
}
