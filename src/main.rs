use cgmath::{ Euler, Deg };

use hephaestus_lib::{
    engine::Engine,
    world::World,
    primitives::{ Plane, Cube },
    object::*,
    camera::Camera,
    light::DirectionalLight
};

fn main() {
    let mut world = World::new(Camera::default([0.0, 4.0, 0.0]));
    
    let monkey_1 = Object::new([0.2, 0.0, 0.0], [0.2; 3], [1.0; 3], "models/monke.obj".into());
    // monkey_1.material.add_texture("models/monke_tex.png");

    let mut monkey_2 = Object::new([-0.2, 0.0, 0.0], [0.2; 3], [1.0, 0.0, 1.0], "models/monke.obj".into());
    monkey_2.material.add_texture("models/textures/obamium.png");

    let mut plane = Plane::new([0.0, -2.0, 0.0], [1.0; 3], [0.5, 1.0, 1.0]);
    plane.rotate(Euler { x: Deg(90.0), y: Deg(0.0), z: Deg(0.0) });
    plane.material.add_texture("models/textures/obamium.png");

    let mut cube = Cube::new([0.0, 0.0, 0.0], [0.2; 3], [1.0; 3]);
    cube.material.add_texture("models/textures/amogus.png");
    
    let light = DirectionalLight::new([-4.0, -4.0, 0.0, 1.0], [0.4, 0.4, 0.4]);
    
    //world.new_object(cube);
    //world.new_object(plane);
    world.add_object(Box::new(monkey_1));
    world.add_object(Box::new(monkey_2));
    
    world.add_light(light);

    let engine = Engine::initialize(world);
    engine.start();
}
