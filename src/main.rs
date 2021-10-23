use winit::event_loop::EventLoop;
use cgmath::Deg;

use hephaestus_lib::{
    engine::Engine,
    world::World,
    light::DirectionalLight,
    logger::{ self, MessageEmitter },
    mesh_data::{ MeshType, PrimitiveType },
    entity::Logic,
    camera,
};

#[allow(unused)]
fn main() {
    let mut world = World::new();
    world.void_color = [0.01, 0.01, 0.01, 1.0];

    let camera = world.new_entity()
        .transform([0.0, 1.0, 0.0], [1.0; 3], [Deg(0.0); 3])
        .camera()
        .logic(Box::new(init), camera::logic::first_person::<1u8, 1u8>());
    world.add_entity(camera);

    let ground_plane = world.new_entity()
        .transform([0.0, -1.0, 0.0], [10.0; 3], [Deg(0.0), Deg(0.0), Deg(0.0)])
        .mesh(MeshType::Primitive(PrimitiveType::Plane))
        .material([0.8; 3]);
    world.add_entity(ground_plane);

    let monkey = world.new_entity()
        .transform([0.0, 0.0, 1.0], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Model("models/suzanne.obj".into()))
        .texture("models/textures/monkey_texture.png")
        .logic(Box::new(init), Logic::empty_update());
    world.add_entity(monkey);

    let cube = world.new_entity()
        .transform([2.0, 0.0, 0.0], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Primitive(PrimitiveType::Cube))
        // .texture("models/textures/color.png")
        .logic(Box::new(init), Logic::empty_update());
    world.add_entity(cube);

    let sphere = world.new_entity()
        .transform([2.0, 0.0, -2.0], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Primitive(PrimitiveType::Sphere(3)))
        .material([1.0, 0.5, 0.2])
        .logic(Box::new(init), Logic::empty_update());
    world.add_entity(sphere);
    
    let white_light = DirectionalLight::new([1.0, 2.0, 1.0, 1.0], [0.5, 0.5, 0.5]);
    world.add_light(white_light);

    logger::log_debug("debug debug debug", MessageEmitter::Engine);
    logger::log_info("info info info", MessageEmitter::Engine);
    logger::log_warning("warning warning warning", MessageEmitter::Engine);
    logger::log_error("error error error", MessageEmitter::Engine);

    let event_loop = EventLoop::with_user_event();
    let engine = Engine::initialize(world, &event_loop);
    engine.start(event_loop);
}

fn init(id: usize, _: &mut World) {
    logger::log_debug(&format!("{}: INIT!", id), MessageEmitter::Object(id.to_string()))
}

