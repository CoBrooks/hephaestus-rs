use std::time::Instant;
use std::borrow::Cow;
use winit::event_loop::{ ControlFlow, EventLoop };
use winit::event::{ Event, WindowEvent };
use egui_winit_vulkano::Gui;
use egui::{ FontDefinitions, FontFamily };

use crate::{
    world::*,
    renderer::Renderer,
    logger
};

pub struct EngineTime {
    pub delta_time: f32,
    pub fps: f32,
    pub total_time_ms: f32,
    pub total_time_s: f32,
    start_time: Instant,
    start_of_last_frame: Instant,
    total_frames: u128,
    last_60_frame_durations: Vec<f32>
}

impl EngineTime {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            delta_time: 0.0,
            fps: 0.0,
            total_time_ms: 0.0,
            total_time_s: 0.0,
            start_time: now,
            start_of_last_frame: now,
            total_frames: 0,
            last_60_frame_durations: Vec::new()
        }
    }

    pub fn update(&mut self) {
        self.total_frames += 1;

        self.total_time_ms = self.start_time.elapsed().as_millis() as f32;
        self.total_time_s = self.total_time_ms / 1000.0;

        let delta_time = (Instant::now() - self.start_of_last_frame).as_secs_f32();
        self.delta_time = delta_time as f32;

        if self.last_60_frame_durations.len() < 100 {
            self.last_60_frame_durations.push(self.delta_time);
        } else {
            self.last_60_frame_durations.reverse();
            self.last_60_frame_durations.pop();
            self.last_60_frame_durations.reverse();
            self.last_60_frame_durations.push(self.delta_time);
        }
        
        let avg_duration_of_last_60_s: f32 = self.last_60_frame_durations.iter().sum::<f32>() / self.last_60_frame_durations.len() as f32;
        self.fps = avg_duration_of_last_60_s.recip();

        self.start_of_last_frame = Instant::now();
    }
}

pub struct Engine {
    pub world: World,
    pub renderer: Renderer,
    time: EngineTime,
}

impl Engine {
    pub fn initialize(world: World, event_loop: &EventLoop<()>) -> Self {
        let renderer = Renderer::new(event_loop, world.camera);

        let time = EngineTime::new();

        Self {
            world,
            renderer,
            time
        }
    }

    pub fn start(mut self, event_loop: EventLoop<()>) {
        let mut gui = Gui::new(self.renderer.surface.clone(), self.renderer.queue.clone(), true);

        let mut previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>> = Some(Box::new(vulkano::sync::now(self.renderer.device.clone())));

        let mut show_debug_log = true;
        let mut prev_height = 350.0;

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
                        
                        let mut font_style = FontDefinitions::default();
                        font_style.font_data.insert("JetBrains Mono".into(), Cow::Borrowed(include_bytes!("../../fonts/JetBrainsMono-Regular.ttf")));
                        font_style.fonts_for_family.insert(FontFamily::Monospace, vec!["JetBrains Mono".into(), ]);
                        font_style.family_and_size.insert(egui::TextStyle::Body, (FontFamily::Monospace, 20.0));
                        font_style.family_and_size.insert(egui::TextStyle::Button, (FontFamily::Proportional, 16.0));
                        ctx.set_fonts(font_style);

                        egui::TopBottomPanel::bottom("Debug")
                            .default_height(350.0)
                            .resizable(true)
                            .max_height(500.0)
                            .show(&ctx, |ui| {
                                ui.label(format!("{} FPS", self.time.fps.round()));
                                if ui.button(if show_debug_log { "Hide Debug Log" } else { "Show Debug Log" }).clicked() {
                                    if !show_debug_log { ui.set_height(prev_height); }
                                    else { prev_height = ui.available_height() }

                                    show_debug_log = !show_debug_log;
                                }
                                
                                if show_debug_log { 
                                    // Snap to bottom once https://github.com/emilk/egui/pull/765 is approved
                                    egui::ScrollArea::auto_sized()
                                        .show(ui, |ui| {
                                            let messages = logger::get_messages();
                                            for message in messages {
                                                ui.colored_label(message.level.color(), message.formatted());
                                            }
                                    });
                                }
                        });
                    });

                    previous_frame_end.as_mut().take().unwrap().cleanup_finished();

                    self.renderer.start(self.world.void_color);
                    
                    let world_clone = self.world.clone();
                    
                    for i in 0..self.world.objects.len() {
                        self.world.objects[i].update(&world_clone, &self.time);
                        self.renderer.geometry(self.world.objects[i].as_ref());
                    }

                    self.renderer.ambient();

                    for i in 0..self.world.lights.len() {
                        self.renderer.directional(&self.world.lights[i]);
                    }

                    self.renderer.finish(&mut previous_frame_end, &mut gui);
                    
                    self.time.update();
                },
                Event::RedrawRequested(_) => {
                    self.renderer.surface.window().request_redraw();
                }
                _ => ()
            }
        })
    }
}
