use winit::event_loop::EventLoop;
use winit::event::VirtualKeyCode;
use cgmath::{ Quaternion, Euler, Deg, Rad };

use hephaestus_lib::{
    engine::Engine,
    world::World,
    camera::Camera,
    light::DirectionalLight,
    logger::{ self, MessageEmitter },
    mesh_data::{ MeshType, PrimitiveType },
    entity::{ Transform, UpdateData },
};

#[allow(unused)]
fn main() {
    let mut world = World::new(Camera::default([0.0, 2.0, 0.5]));
    world.void_color = [0.01, 0.01, 0.01, 1.0];
    
    let white_light = DirectionalLight::new([1.0, 2.0, 1.0, 1.0], [0.5, 0.5, 0.5]);

    let monkey = world.new_entity()
        .transform([0.0; 3], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Model("models/suzanne.obj".into()))
        .texture("models/textures/monkey_texture.png")
        .logic(Box::new(init), Box::new(update));

    let cube = world.new_entity()
        .transform([0.0, 0.0, 1.0], [0.2; 3], [Deg(0.0); 3])
        .mesh(MeshType::Primitive(PrimitiveType::Cube))
        // .texture("models/textures/color.png")
        .logic(Box::new(init), Box::new(update));

    let sphere = world.new_entity()
        .transform([0.0, 1.0, 0.0], [0.4; 3], [Deg(0.0); 3])
        .mesh(MeshType::Primitive(PrimitiveType::Sphere(3)))
        .material([1.0, 0.5, 0.2])
        .logic(Box::new(init), Box::new(update));

    logger::log_debug("debug debug debug", MessageEmitter::Engine);
    logger::log_info("info info info", MessageEmitter::Engine);
    logger::log_warning("warning warning warning", MessageEmitter::Engine);
    logger::log_error("error error error", MessageEmitter::Engine);

    // world.add_entity(cube);
    // world.add_entity(sphere);
    world.add_entity(monkey);

    world.add_light(white_light);

    let event_loop = EventLoop::with_user_event();
    let engine = Engine::initialize(world, &event_loop);
    engine.start(event_loop);
}

fn init(id: usize, _: &mut World) {
    logger::log_debug(&format!("{}: INIT!", id), MessageEmitter::Object(id.to_string()))
}

fn update(id: usize, data: &mut UpdateData) {
    let (mouse_x, mouse_y) = data.input.mouse_pos_rel();
    let rot_x: f32 = -(mouse_y * 2.0 - 1.0).atan();
    let rot_y: f32 = (mouse_x * 2.0 - 1.0).atan();

    let transform = data.world.get_component_by_id_mut::<Transform>(id).unwrap();
    transform.rotation = Quaternion::from(Euler::new(Rad(rot_x), Rad(0.0), Rad(rot_y)));

    if data.input.get_key(VirtualKeyCode::Space) {
        transform.rotate([Deg(0.0), Deg(0.0), Deg(60.0 * data.time.delta_time)]);
    }

    if data.input.get_key_down(VirtualKeyCode::A) {
        logger::log_info("User pressed 'A'", MessageEmitter::Object(id.to_string()));
    }
}

