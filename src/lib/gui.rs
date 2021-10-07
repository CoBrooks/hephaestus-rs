use std::borrow::Cow;
use egui::{
    Color32,
    CtxRef,
    FontDefinitions,
    FontFamily,
    Ui
};
use egui_winit_vulkano::Gui;

use crate::{ 
    logger,
    engine::{ EngineTime, FrameTimeBreakdown }
};

pub struct DebugGui { 
    show_debug_log: bool,
}

impl DebugGui {
    pub fn new() -> Self {
        Self {
            show_debug_log: true,
        }
    }

    pub fn show(&mut self, gui: &mut Gui, time: &EngineTime, frame_breakdown: &FrameTimeBreakdown) {
        gui.immediate_ui(|gui| {
            let mut ctx = gui.context();
            self.configure_fonts(&mut ctx);

            egui::TopBottomPanel::bottom("Debug")
                .default_height(350.0)
                .resizable(true)
                .max_height(500.0)
                .show(&ctx, |mut ui| {
                    self.debug_log_menu(&mut ui, time);
                    
                    if self.show_debug_log {
                        ui.separator();

                        ui.columns(2, |columns| {
                            self.debug_log(&mut columns[0]);
                            self.frame_breakdown(&mut columns[1], time, frame_breakdown);
                        });
                    }
                });
        });
    }

    fn configure_fonts(&mut self, ctx: &mut CtxRef) {
        let mut font_style = FontDefinitions::default();
        font_style.font_data.insert("JetBrains Mono".into(), Cow::Borrowed(include_bytes!("../../fonts/JetBrainsMono-Regular.ttf")));
        font_style.fonts_for_family.insert(FontFamily::Monospace, vec!["JetBrains Mono".into(), ]);
        font_style.family_and_size.insert(egui::TextStyle::Body, (FontFamily::Monospace, 20.0));
        font_style.family_and_size.insert(egui::TextStyle::Button, (FontFamily::Proportional, 16.0));
        ctx.set_fonts(font_style);
    }

    fn debug_log_menu(&mut self, ui: &mut Ui, time: &EngineTime) {
        ui.horizontal_top(|ui| {
            if ui.button(if self.show_debug_log { "Hide Debug Log" } else { "Show Debug Log"}).clicked() {
                if !self.show_debug_log { ui.shrink_height_to_current(); }

                self.show_debug_log = !self.show_debug_log;
            }
            
            ui.add_space(ui.available_size_before_wrap().x - ui.fonts().glyph_width(egui::TextStyle::Monospace, '0') * 10.0);

            ui.label(format!("{} FPS", time.fps.round()));
        });
    }

    fn debug_log(&mut self, ui: &mut Ui) {
        // Snap to bottom once https://github.com/emilk/egui/pull/765 is approved.
        egui::ScrollArea::auto_sized()
            .show(ui, |ui| {
                let messages = logger::get_messages();
                for message in messages {
                    ui.colored_label(message.level.color(), message.formatted());
                    ui.separator();
                }
        });
    }

    fn frame_breakdown(&mut self, ui: &mut Ui, time: &EngineTime, frame_breakdown: &FrameTimeBreakdown) {
        ui.vertical(|ui| {
            ui.heading("Frame Breakdown:");
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Setup:");
                ui.add_space(ui.available_size_before_wrap().x - ui.fonts().glyph_width(egui::TextStyle::Monospace, '0') * 10.0);
                ui.label(format!("{:.2}ms", frame_breakdown.setup.as_secs_f32() * 1000.0));
            });

            ui.horizontal(|ui| {
                ui.label("Object Loop:");
                ui.add_space(ui.available_size_before_wrap().x - ui.fonts().glyph_width(egui::TextStyle::Monospace, '0') * 10.0);
                ui.label(format!("{:.2}ms", frame_breakdown.object_loop.as_secs_f32() * 1000.0));
            });

            ui.horizontal(|ui| {
                ui.label("Ambient Lighting:");
                ui.add_space(ui.available_size_before_wrap().x - ui.fonts().glyph_width(egui::TextStyle::Monospace, '0') * 10.0);
                ui.label(format!("{:.2}ms", frame_breakdown.ambient.as_secs_f32() * 1000.0));
            });

            ui.horizontal(|ui| {
                ui.label("Directional Lighting:");
                ui.add_space(ui.available_size_before_wrap().x - ui.fonts().glyph_width(egui::TextStyle::Monospace, '0') * 10.0);
                ui.label(format!("{:.2}ms", frame_breakdown.directional.as_secs_f32() * 1000.0));
            });

            ui.horizontal(|ui| {
                ui.label("GPU Dispatch (Draw Call):");
                ui.add_space(ui.available_size_before_wrap().x - ui.fonts().glyph_width(egui::TextStyle::Monospace, '0') * 10.0);
                ui.label(format!("{:.2}ms", frame_breakdown.draw_call.as_secs_f32() * 1000.0));
            });

            ui.horizontal(|ui| {
                ui.label("Total:");
                ui.add_space(ui.available_size_before_wrap().x - ui.fonts().glyph_width(egui::TextStyle::Monospace, '0') * 10.0);
                ui.label(format!("{:.2}ms", time.last_60_frame_durations.last().unwrap_or(&0.0) * 1000.0));
            });
        });
        
        ui.separator();
        
        let line: Vec<egui::plot::Value> = time.last_60_frame_durations.iter()
            .enumerate()
            .map(|(x, &f)| egui::plot::Value::new(x as f32 / 100.0, f))
            .collect();
        ui.add(
            egui::plot::Plot::new("Benchmarking")
                .line(egui::plot::Line::new(egui::plot::Values::from_values(line)))
                .hline(egui::plot::HLine::new(0.0).color(Color32::TRANSPARENT))
                .hline(egui::plot::HLine::new(1.0 / 60.0).stroke(egui::Stroke::new(0.01, Color32::GREEN)))
                .hline(egui::plot::HLine::new(1.0 / 30.0).color(Color32::RED))
                .include_x(1.0)
                .show_x(false)
                .allow_drag(false)
                .allow_zoom(false)
                .show_axes([false; 2])
                .view_aspect(30.0)
        );
    }
}
