use std::collections::HashSet;
use winit::event::{ VirtualKeyCode, ElementState, WindowEvent };

pub struct Input {
    keyboard: HashSet<u32>,
    keyboard_prev: HashSet<u32>,
    mouse_pos: (u32, u32),
    window_size: (u32, u32)
}

impl Input {
    pub fn new(window_size: (u32, u32)) -> Self {
        Self {
            keyboard: HashSet::new(),
            keyboard_prev: HashSet::new(),
            mouse_pos: (0, 0),
            window_size
        }
    }

    pub fn update(&mut self) {
        self.keyboard_prev = self.keyboard.clone();
    }

    pub fn parse(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
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
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as u32, position.y as u32);
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

    pub fn mouse_pos(&self) -> (u32, u32) {
        self.mouse_pos
    }

    pub fn mouse_pos_rel(&self) -> (f32, f32) {
        let (m_x, m_y) = self.mouse_pos;
        let (w_x, w_y) = self.window_size;
        (m_x as f32 / w_x as f32, m_y as f32 / w_y as f32)
    }
}

