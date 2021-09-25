use cgmath::{ Euler, Deg };

use hephaestus_lib::{
    engine::Engine,
    world::World,
    primitives::{ Plane, Cube },
    object::*
};

fn main() {
    let mut world = World::new();
    
    let mut monkey_1 = Object::new([0.0; 3], [0.2; 3], "models/monke.obj".into());
    monkey_1.add_texture("models/monke_tex.png");
    
    world.new_object(monkey_1);
    
    //let mut cube = Cube::identity();
    //cube.add_texture("models/textures/obamium.png");

    //world.new_object(cube);
    
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
