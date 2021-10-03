use std::time::Instant;
use winit::event_loop::{ ControlFlow, EventLoop };
use winit::event::{ Event, WindowEvent };
use egui_winit_vulkano::Gui;

use crate::{
    world::*,
    renderer::Renderer,
    logger::{ APP_LOGGER, Logger, MessageEmitter }
};

#[allow(dead_code)]
pub struct Engine {
    pub world: World,
    pub renderer: Renderer,
    start_time: Instant,
    start_of_last_frame: Instant,
    delta_time: f32,
    total_time: f32,
}

pub struct EngineTime {
    pub delta_time: f32,
    pub total_time: f32
}

impl Engine {
    pub fn initialize(world: World, event_loop: &EventLoop<()>) -> Self {
        let start_time = Instant::now();
        let start_of_last_frame = Instant::now();
    
        let renderer = Renderer::new(event_loop, world.camera);

        let delta_time = 0.0;
        let total_time = 0.0;

        Self {
            world,
            renderer,
            start_time,
            start_of_last_frame,
            delta_time,
            total_time
        }
    }

    pub fn start(mut self, event_loop: EventLoop<()>) {
        let mut gui = Gui::new(self.renderer.surface.clone(), self.renderer.queue.clone(), true);

        let mut previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>> = Some(Box::new(vulkano::sync::now(self.renderer.device.clone())));

        event_loop.run(move |event, _, control_flow| {
            gui.update(&event);

            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit;
                },
                Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                    self.renderer.recreate_swapchain();
                },
                Event::MainEventsCleared => {
                    gui.immediate_ui(|gui| {
                        let ctx = gui.context();

                        egui::TopBottomPanel::bottom("Bottom Panel").show(&ctx, |ui| {
                            if ui.button("Button").clicked(){
                                APP_LOGGER.log_info("Button clicked", MessageEmitter::Engine);
                            }
                        });

                        egui::Window::new("Window").show(&ctx, |ui| {
                            ui.label("A Label inside of a window.")
                        });
                    });

                    previous_frame_end.as_mut().take().unwrap().cleanup_finished();

                    self.renderer.start(self.world.void_color);
                    
                    let world_clone = self.world.clone();
                    let time = EngineTime { 
                        delta_time: self.delta_time,
                        total_time: self.total_time
                    };

                    for i in 0..self.world.objects.len() {
                        self.world.objects[i].update(&world_clone, &time);
                        self.renderer.geometry(self.world.objects[i].as_ref());
                    }

                    self.renderer.ambient();

                    for i in 0..self.world.lights.len() {
                        self.renderer.directional(&self.world.lights[i]);
                    }

                    self.renderer.finish(&mut previous_frame_end, &mut gui);


                    self.delta_time = (Instant::now() - self.start_of_last_frame).as_secs_f32();

                    self.start_of_last_frame = Instant::now();
                    self.total_time = self.start_time.elapsed().as_secs_f32();
                },
                Event::RedrawRequested(_) => {
                    self.renderer.surface.window().request_redraw();
                }
                _ => ()
            }
        })
    }
}
