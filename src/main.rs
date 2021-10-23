use winit::event_loop::EventLoop;
use cgmath::{ Deg, Rad };

use hephaestus_lib::{
    engine::Engine,
    world::World,
    light::DirectionalLight,
    logger::{ self, MessageEmitter },
    mesh_data::{ MeshType, PrimitiveType },
    entity::{ Transform, UpdateData },
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
        .transform([0.0; 3], [10.0; 3], [Deg(0.0), Deg(0.0), Deg(180.0)])
        .mesh(MeshType::Primitive(PrimitiveType::Plane));
    //world.add_entity(ground_plane);

    let monkey = world.new_entity()
        .transform([0.0, 0.0, 1.0], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Model("models/suzanne.obj".into()))
        .texture("models/textures/monkey_texture.png")
        .logic(Box::new(init), Box::new(update));
    world.add_entity(monkey);

    let cube = world.new_entity()
        .transform([2.0, 0.0, 0.0], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Primitive(PrimitiveType::Cube))
        // .texture("models/textures/color.png")
        .logic(Box::new(init), Box::new(update));
    world.add_entity(cube);

    let sphere = world.new_entity()
        .transform([2.0, 0.0, -2.0], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Primitive(PrimitiveType::Sphere(3)))
        .material([1.0, 0.5, 0.2])
        .logic(Box::new(init), Box::new(update));
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

fn update(id: usize, data: &mut UpdateData) {
    // let transform = data.world.get_component_by_id_mut::<Transform>(id).unwrap();
    // transform.rotate([Rad(0.0), Rad(data.time.delta_time), Rad(0.0)]);
    // transform.rotate_local([Rad(data.time.delta_time), Rad(0.0), Rad(0.0)]);

    // transform.translate_local([0.0, data.time.delta_time, 0.0]);

    // if data.input.get_key_down(VirtualKeyCode::A) {
    //     logger::log_info(&format!("{:?}", transform.translation), MessageEmitter::Object("obj".into()));
    // }
}

