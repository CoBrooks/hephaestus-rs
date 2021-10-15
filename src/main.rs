use winit::event_loop::EventLoop;
use cgmath::Deg;

use hephaestus_lib::{
    engine::Engine,
    world::World,
    camera::Camera,
    light::DirectionalLight,
    logger::{ self, MessageEmitter },
    mesh_data::MeshType,
    entity::Transform
};

fn main() {
    let mut world = World::new(Camera::default([2.0, 2.0, 0.5]));
    world.void_color = [0.01, 0.01, 0.01, 1.0];
    
    let white_light = DirectionalLight::new([1.0, 2.0, 1.0, 1.0], [0.5, 0.5, 0.5]);

    let blank = world.new_entity();

    let monkey = world.new_entity()
        .transform([0.0; 3], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Model("models/suzanne.obj".into()))
        // .texture("models/textures/monkey_texture.png")
        .logic(Box::new(init), Box::new(update));

    logger::log_debug("debug debug debug", MessageEmitter::Engine);
    logger::log_info("info info info", MessageEmitter::Engine);
    logger::log_warning("warning warning warning", MessageEmitter::Engine);
    logger::log_error("error error error", MessageEmitter::Engine);

    world.add_entity(blank.clone());
    world.add_entity(blank.clone());
    world.add_entity(blank.clone());
    world.add_entity(monkey);
    world.add_light(white_light);

    let event_loop = EventLoop::with_user_event();
    let engine = Engine::initialize(world, &event_loop);
    engine.start(event_loop);
}

fn init(id: usize, _: &mut World) {
    logger::log_debug(&format!("{}: INIT!", id), MessageEmitter::Object(id.to_string()))
}

fn update(id: usize, world: &mut World) {
    let transform = world.get_component_by_id_mut::<Transform>(id).expect("");
    transform.rotate([Deg(0.0), Deg(0.0), Deg(1.0)]);
}

