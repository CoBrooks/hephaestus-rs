use std::collections::HashSet;
use winit::event::{ VirtualKeyCode, ElementState, DeviceEvent };

pub struct Input {
    keyboard: HashSet<u32>,
    keyboard_prev: HashSet<u32>,
    mouse_pos: (f32, f32),
    mouse_delta: (f32, f32),
    window_size: (u32, u32)
}

impl Input {
    pub fn new(window_size: (u32, u32)) -> Self {
        Self {
            keyboard: HashSet::new(),
            keyboard_prev: HashSet::new(),
            mouse_pos: (window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0),
            mouse_delta: (0.0, 0.0),
            window_size
        }
    }

    pub fn update(&mut self) {
        self.keyboard_prev = self.keyboard.clone();
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn parse(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::Key(input) => {
                if input.state == ElementState::Pressed {
                    if let Some(vkey) = input.virtual_keycode {
                        self.keyboard.insert(vkey as u32);
                    } else {
                        self.keyboard.insert(input.scancode);
                    }
                } else {
                    if let Some(vkey) = input.virtual_keycode {
                        self.keyboard.remove(&(vkey as u32));
                    } else {
                        self.keyboard.remove(&input.scancode);
                    }
                }
            },
            DeviceEvent::MouseMotion { delta, .. } => {
                self.mouse_pos.0 += delta.0 as f32;
                self.mouse_pos.1 += delta.1 as f32;
                self.mouse_delta = (delta.0 as f32, delta.1 as f32);
            },
            _ => { }
        }
    }

    pub fn get_key(&self, key: VirtualKeyCode) -> bool {
        self.keyboard.contains(&(key as u32))
    }

    pub fn get_key_down(&self, key: VirtualKeyCode) -> bool { 
        self.keyboard.contains(&(key as u32)) && !self.keyboard_prev.contains(&(key as u32))
    }

    pub fn get_key_up(&self, key: VirtualKeyCode) -> bool {
        !self.keyboard.contains(&(key as u32)) && self.keyboard_prev.contains(&(key as u32))
    }

    pub fn mouse_pos(&self) -> (f32, f32) {
        self.mouse_pos
    }

    pub fn mouse_pos_rel(&self) -> (f32, f32) {
        let (m_x, m_y) = self.mouse_pos;
        let (w_x, w_y) = self.window_size;
        (m_x / w_x as f32, m_y / w_y as f32)
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }
}

