use std::time::{ Duration, Instant };
use winit::event_loop::{ ControlFlow, EventLoop };
use winit::event::{ Event, WindowEvent };
use egui_winit_vulkano::Gui;

use crate::{
    world::*,
    renderer::Renderer,
    gui::DebugGui,
    entity::*
};

pub struct EngineTime {
    pub delta_time: f32,
    pub fps: f32,
    pub total_time_ms: f32,
    pub total_time_s: f32,
    pub last_60_frame_durations: Vec<f32>,
    start_time: Instant,
    start_of_last_frame: Instant,
    total_frames: u128,
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

pub struct FrameTimeBreakdown {
    pub start: Instant,
    pub setup: Duration,
    pub object_loop: Duration,
    pub ambient: Duration,
    pub directional: Duration,
    pub draw_call: Duration,
    temp_time: Instant
}

impl FrameTimeBreakdown {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            setup: Duration::default(),
            object_loop: Duration::default(),
            ambient: Duration::default(),
            directional: Duration::default(),
            draw_call: Duration::default(),
            temp_time: Instant::now(),
        }
    }

    pub fn restart(&mut self) {
        *self = Self::new();
    }

    pub fn update_setup(&mut self) {
        let now = Instant::now();
        self.setup = now - self.start;
        self.temp_time = now;
    }

    pub fn update_object_loop(&mut self) {
        let now = Instant::now();
        self.object_loop = now - self.temp_time;
        self.temp_time = now;
    }

    pub fn update_ambient(&mut self) {
        let now = Instant::now();
        self.ambient = now - self.temp_time;
        self.temp_time = now;
    }
    
    pub fn update_directional(&mut self) {
        let now = Instant::now();
        self.directional = now - self.temp_time;
        self.temp_time = now;
    }
    
    pub fn update_draw_call(&mut self) {
        let now = Instant::now();
        self.draw_call = now - self.temp_time;
        self.temp_time = now;
    }
}

pub struct Engine {
    pub world: World,
    pub renderer: Renderer,
    pub debug_gui: DebugGui,
    time: EngineTime,
}

impl Engine {
    pub fn initialize(world: World, event_loop: &EventLoop<()>) -> Self {
        let renderer = Renderer::new(event_loop, world.camera);

        let time = EngineTime::new();
        let debug_gui = DebugGui::new();

        Self {
            world,
            renderer,
            time,
            debug_gui
        }
    }

    pub fn start(mut self, event_loop: EventLoop<()>) {
        let mut gui = Gui::new(self.renderer.surface.clone(), self.renderer.queue.clone(), true);

        let mut previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>> = Some(Box::new(vulkano::sync::now(self.renderer.device.clone())));

        let mut frame_breakdown = FrameTimeBreakdown::new();

        // Initialize Entities
        let initial_world = self.world.clone();
        
        if let Some(logics) = initial_world.get_components_of_type::<Logic>() {
            for l in logics {
                (l.init)(*l.get_id(), &mut self.world)
            }
        }
        
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
                    self.debug_gui.show(&mut gui, &self.time, &frame_breakdown);
                    frame_breakdown.restart();

                    previous_frame_end.as_mut().take().unwrap().cleanup_finished();

                    self.renderer.start(self.world.void_color);
                    frame_breakdown.update_setup();
                    
                    let world_clone = self.world.clone();
                    let ids = world_clone.get_all_ids().unwrap_or(vec![]);
                    for id in ids {
                        if self.world.get_component_by_id::<Mesh>(*id).is_none() || self.world.get_component_by_id::<Transform>(*id).is_none() { return; }

                        self.renderer.geometry(
                            self.world.get_component_by_id::<Mesh>(*id).unwrap(), 
                            self.world.get_component_by_id::<Transform>(*id).unwrap(), 
                            self.world.get_component_by_id::<Material>(*id),
                            self.world.get_component_by_id::<Texture>(*id)
                        );

                        if let Some(logic) = world_clone.get_component_by_id::<Logic>(*id) {
                            (logic.update)(*logic.get_id(), &mut self.world)
                        }
                    }
                    frame_breakdown.update_object_loop();

                    self.renderer.ambient();
                    frame_breakdown.update_ambient();

                    for i in 0..self.world.lights.len() {
                        self.renderer.directional(&self.world.lights[i]);
                    }
                    frame_breakdown.update_directional();

                    self.renderer.finish(&mut previous_frame_end, &mut gui);
                    frame_breakdown.update_draw_call();
                    
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
