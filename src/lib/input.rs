use std::collections::{ HashSet, HashMap };
use winit::event::{ VirtualKeyCode, ElementState, DeviceEvent, ButtonId, MouseScrollDelta };

pub struct Input {
    keyboard: HashSet<u32>,
    keyboard_prev: HashSet<u32>,
    buttons: HashSet<u32>,
    buttons_prev: HashSet<u32>,
    axes: HashMap<String, f32>,
    mouse_pos: (f32, f32),
    mouse_delta: (f32, f32),
    scroll_wheel: f32,
    window_size: (u32, u32)
}

impl Input {
    pub fn new(window_size: (u32, u32)) -> Self {
        let mut axes = HashMap::new();
        axes.insert("horizontal".into(), 0.0);
        axes.insert("vertical".into(), 0.0);

        Self {
            keyboard: HashSet::new(),
            keyboard_prev: HashSet::new(),
            buttons: HashSet::new(),
            buttons_prev: HashSet::new(),
            axes,
            mouse_pos: (window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0),
            mouse_delta: (0.0, 0.0),
            scroll_wheel: 0.0,
            window_size
        }
    }

    pub fn update(&mut self) {
        self.keyboard_prev = self.keyboard.clone();
        self.buttons_prev = self.buttons.clone();
        self.mouse_delta = (0.0, 0.0);
        self.scroll_wheel = 0.0;

        if self.get_key(VirtualKeyCode::W) || self.get_key(VirtualKeyCode::Up) {
            *self.axes.get_mut("vertical").unwrap() = 1.0;
        } else if self.get_key(VirtualKeyCode::S) || self.get_key(VirtualKeyCode::Down) {
            *self.axes.get_mut("vertical").unwrap() = -1.0;
        } else {
            *self.axes.get_mut("vertical").unwrap() = 0.0;
        }
        
        if self.get_key(VirtualKeyCode::D) || self.get_key(VirtualKeyCode::Right) {
            *self.axes.get_mut("horizontal").unwrap() = 1.0;
        } else if self.get_key(VirtualKeyCode::A) || self.get_key(VirtualKeyCode::Left) {
            *self.axes.get_mut("horizontal").unwrap() = -1.0;
        } else {
            *self.axes.get_mut("horizontal").unwrap() = 0.0;
        }
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
            DeviceEvent::Button { button, state } => {
                if state == &ElementState::Pressed {
                    self.buttons.insert(*button);
                } else {
                    self.buttons.remove(button);
                }
            },
            DeviceEvent::MouseWheel { delta } => {
                if let MouseScrollDelta::LineDelta(_, y) = delta {
                    self.scroll_wheel = -y.signum();
                }
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

    pub fn get_button(&self, button: ButtonId) -> bool {
        self.buttons.contains(&button)
    }

    pub fn get_button_down(&self, button: ButtonId) -> bool { 
        self.buttons.contains(&button) && !self.buttons_prev.contains(&button)
    }

    pub fn get_button_up(&self, button: ButtonId) -> bool {
        !self.buttons.contains(&button) && self.buttons_prev.contains(&button)
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

    pub fn scroll_wheel(&self) -> f32 {
        self.scroll_wheel
    }

    pub fn get_axis(&self, axis: &str) -> Option<f32> {
        self.axes.get(&axis.to_lowercase()).cloned()
    }
}

