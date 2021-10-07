use winit::event_loop::EventLoop;
use cgmath::Deg;

use hephaestus_lib::{
    engine::{ Engine, EngineTime },
    world::World,
    object::*,
    camera::Camera,
    light::DirectionalLight,
    logger::{ self, MessageEmitter }
};

fn main() {
    let mut world = World::new(Camera::default([2.0, 2.0, 0.5]));
    world.void_color = [0.01, 0.01, 0.01, 1.0];
    
    let mut teapot = Object::new([0.0, 0.0, 0.0], [0.2; 3], [1.0; 3], "models/teapot.obj".into());
    teapot.add_update(Box::new(update_loop));
    
    let white_light = DirectionalLight::new([1.0, 2.0, 1.0, 1.0], [0.5, 0.5, 0.5]);

    logger::log_debug("debug debug debug", MessageEmitter::Engine);
    logger::log_info("info info info", MessageEmitter::Engine);
    logger::log_warning("warning warning warning", MessageEmitter::Engine);
    logger::log_error("error error error", MessageEmitter::Engine);

    world.add_object("Utah Teapot", Box::new(teapot));
    world.add_light(white_light);

    let event_loop = EventLoop::with_user_event();
    let engine = Engine::initialize(world, &event_loop);
    engine.start(event_loop);
}

fn update_loop(object: &mut Object, _world: &World, time: &EngineTime) {
    object.transform.rotate([Deg(0.0), Deg(0.0), Deg(1.0)]);

    let r = time.total_time_s.sin().abs();
    let g = (time.total_time_s + 2.09).sin().abs();
    let b = (time.total_time_s + 4.18).sin().abs();
    let new_color = [r, g, b];
    object.material.set_color(new_color);
}
