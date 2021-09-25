use cgmath::{ Euler, Deg };

use hephaestus_lib::{
    engine::Engine,
    world::World,
    primitives::{ Plane, Cube },
    object::*
};

fn main() {
    let mut world = World::new();
    
    let cube_1 = Cube::new([0.0; 3], [1.0; 3], [0.5, 1.0, 0.25]);
    let mut plane_1 = Plane::new([0.0; 3], [0.5; 3], [1.0, 1.0, 0.5]);
    plane_1.rotate(Euler::new(Deg(90.0), Deg(-90.0), Deg(0.0)));

    let monkey_1 = Object::new([0.0; 3], [0.2; 3], "models/monke.obj".into());
    let monkey_2 = Object::new([0.0, 1.0, 0.0], [0.2; 3], "models/monke.obj".into());

    //world.new_object(cube_1);
    //world.new_object(plane_1);
    
    world.new_object(monkey_1);
    //world.new_object(monkey_2);
    
    let engine = Engine::initialize(world);
    engine.main_loop();
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/vert.glsl"
    }
}

 mod fs {
     vulkano_shaders::shader! {
         ty: "fragment",
         path: "src/shaders/frag.glsl"
     }
 }
