use cgmath::{ Euler, Deg };

use hephaestus_lib::{
    engine::Engine,
    world::World,
    primitives::{ Plane, Cube },
    object::*,
    material::*,
    camera::Camera
};

fn main() {
    let mut world = World::new(Camera::default([-1.0, -1.0, 0.0], [1200, 1600]));
    
    let mut monkey_1 = Object::new([0.0, -1.0, 0.0], [0.5; 3], [1.0; 3], "models/monke.obj".into());
    monkey_1.material.add_texture("models/monke_tex.png");

    let mut monkey_2 = Object::new([0.0, 0.0, 0.0], [0.2; 3], [1.0; 3], "models/monke.obj".into());
    monkey_2.material.add_texture("models/textures/obamium.png");

    let mut plane = Plane::new([0.0, -2.0, 0.0], [1.0; 3], [0.5, 1.0, 1.0]);
    plane.rotate(Euler { x: Deg(90.0), y: Deg(0.0), z: Deg(0.0) });
    plane.material.add_texture("models/textures/obamium.png");

    let mut cube = Cube::new([0.0, 0.0, 0.0], [0.2; 3], [1.0; 3]);
    cube.material.add_texture("models/textures/amogus.png");
    
    world.new_object(cube);
    //world.new_object(plane);
    //world.new_object(monkey_1);
    //world.new_object(monkey_2);
    
    //let mut cube = Cube::identity();
    //cube.add_texture("models/textures/obamium.png");

    //world.new_object(cube);
    let engine = Engine::initialize(world);
    engine.main_loop();
}
