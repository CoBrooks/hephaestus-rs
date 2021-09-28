use cgmath::Deg;

use hephaestus_lib::{
    engine::Engine,
    world::World,
    primitives::{ Plane, Cube },
    object::*,
    camera::Camera,
    light::DirectionalLight
};

fn main() {
    let mut world = World::new(Camera::default([2.0, 2.0, 2.0]));
    
    let monkey_1 = Object::new([-2.0, 0.0, 0.0], [0.2; 3], [0.4, 0.4, 0.4], "models/monke.obj".into());

    let monkey_2 = Object::new([-2.0, 0.0, 0.3], [0.1; 3], [1.0, 1.0, 1.0], "models/monke.obj".into());
    
    let r_light = DirectionalLight::new([-0.3, -0.3, 1.0, 1.0], [1.0, 0.0, 0.0]);
    let g_light = DirectionalLight::new([0.3, -0.3, 1.0, 1.0], [0.0, 1.0, 0.0]);
    let b_light = DirectionalLight::new([0.0, 1.0, 1.0, 1.0], [0.0, 0.0, 1.0]);
  
    world.add_object(Box::new(monkey_1));
    world.add_object(Box::new(monkey_2));
    
    world.add_light(r_light);
    world.add_light(g_light);
    world.add_light(b_light);

    let engine = Engine::initialize(world);
    engine.start();
}
