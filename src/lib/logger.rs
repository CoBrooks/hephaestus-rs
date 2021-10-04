use std::collections::HashMap;
use std::cell::RefCell;
use std::time::SystemTime;
use chrono::{ DateTime, Local };
use egui::Color32;
use colored::Colorize;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn color(&self) -> Color32 {
        match self {
            LogLevel::Debug => {
                Color32::from_gray(128)
            },
            LogLevel::Info => {
                Color32::from_gray(255)
            },
            LogLevel::Warning => {
                Color32::from_rgb(255, 200, 64)
            },
            LogLevel::Error => {
                Color32::from_rgb(255, 64, 64)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum MessageEmitter {
    Object(String),
    Engine,
    Renderer,
    World
}

#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub time: SystemTime,
    pub level: LogLevel,
    pub emitter: MessageEmitter
}

impl Message {
    pub fn new(content: String, level: LogLevel, emitter: MessageEmitter) -> Self {
        Self {
            content, 
            time: SystemTime::now(), 
            level,
            emitter
        }
    }

    pub fn print(&self) {
        match self.level {
            LogLevel::Debug => { 
                println!("{}", self.formatted().dimmed())
            },
            LogLevel::Info => { 
                println!("{}", self.formatted().normal())
            },
            LogLevel::Warning => { 
                println!("{}", self.formatted().yellow())
            },
            LogLevel::Error => { 
                println!("{}", self.formatted().red())
            },
        }
    }

    pub fn formatted(&self) -> String {
        let emitter = match &self.emitter {
            MessageEmitter::Object(e) => e,
            MessageEmitter::Engine => "Engine",
            MessageEmitter::Renderer => "Renderer",
            MessageEmitter::World => "World"
        };

        let time: DateTime<Local> = self.time.into();
        let time_str = time.format("%H:%M:%S%.3f");

        match self.level {
            LogLevel::Debug => { 
                format!("DEBUG [{} | {}]: {}", emitter, time_str, self.content)
            },
            LogLevel::Info => { 
                format!("INFO  [{} | {}]: {}", emitter, time_str, self.content)
            },
            LogLevel::Warning => { 
                format!("WARN  [{} | {}]: {}", emitter, time_str, self.content)
            },
            LogLevel::Error => { 
                format!("ERROR [{} | {}]: {}", emitter, time_str, self.content)
            },
        }
    }
}

pub struct Log {
    messages: Option<HashMap<SystemTime, Message>>,
}

trait Logger {
    fn log_debug(&self, content: &str, emitter: MessageEmitter);
    fn log_info(&self, content: &str, emitter: MessageEmitter);
    fn log_warning(&self, content: &str, emitter: MessageEmitter);
    fn log_error(&self, content: &str, emitter: MessageEmitter);
    fn log(&self, message: Message);
    fn get_level(&self, level: LogLevel) -> Option<Vec<Message>>;
    fn filter_messages(&self, filter: &dyn Fn(&&Message) -> bool) -> Option<Vec<Message>>;
    fn get_all_messages(&self) -> Option<Vec<Message>>;
}

impl Logger for RefCell<Log> {
    fn log_debug(&self, content: &str, emitter: MessageEmitter) {
        let msg = Message::new(content.into(), LogLevel::Debug, emitter);
        self.log(msg);
    }
    
    fn log_info(&self, content: &str, emitter: MessageEmitter) {
        let msg = Message::new(content.into(), LogLevel::Info, emitter);
        self.log(msg);
    }
    
    fn log_warning(&self, content: &str, emitter: MessageEmitter) {
        let msg = Message::new(content.into(), LogLevel::Warning, emitter);
        self.log(msg);
    }

    fn log_error(&self, content: &str, emitter: MessageEmitter) {
        let msg = Message::new(content.into(), LogLevel::Error, emitter);
        self.log(msg);
    }

    fn get_level(&self, level: LogLevel) -> Option<Vec<Message>> {
        if self.borrow().messages.is_some() {
            let messages = self.borrow().messages.as_ref().unwrap().clone();
            Some(messages.values()
                .filter(|m| m.level == level)
                .map(|m| m.to_owned())
                .collect())
        } else {
            None
        }
    }

    fn filter_messages(&self, filter: &dyn Fn(&&Message) -> bool) -> Option<Vec<Message>> {
        if self.borrow().messages.is_some() {
            let messages = self.borrow().messages.as_ref().unwrap().clone();
            Some(messages.values()
                .filter(filter)
                .map(|m| m.to_owned())
                .collect())
        } else {
            None
        }
    }

    fn log(&self, mut message: Message) {
        let time = SystemTime::now();
        message.time = time;

        message.print();

        if self.borrow().messages.is_some() {
            let mut messages = self.borrow_mut().messages.take().unwrap();
            messages.insert(time, message);

            self.borrow_mut().messages = Some(messages);
        } else {
            let mut messages = HashMap::new();
            messages.insert(time, message);
            
            self.borrow_mut().messages = Some(messages);
        }
    }

    fn get_all_messages(&self) -> Option<Vec<Message>> {
        if self.borrow().messages.is_some() {
            let mut messages: Vec<_> = self.borrow().messages.as_ref().unwrap().clone().into_iter().collect();
            messages.sort_by(|x, y| x.0.cmp(&y.0));
            
            Some(messages.into_iter().map(|(_, v)| v).collect())
        } else {
            None
        }
    }
}

thread_local! {
    pub static APP_LOGGER: RefCell<Log> = RefCell::new(Log { messages: None });
}

pub fn log_debug(content: &str, emitter: MessageEmitter) {
    APP_LOGGER.with(|logger| logger.log_debug(content, emitter));
}

pub fn log_info(content: &str, emitter: MessageEmitter) {
    APP_LOGGER.with(|logger| logger.log_info(content, emitter));
}

pub fn log_warning(content: &str, emitter: MessageEmitter) {
    APP_LOGGER.with(|logger| logger.log_warning(content, emitter));
}

pub fn log_error(content: &str, emitter: MessageEmitter) {
    APP_LOGGER.with(|logger| logger.log_error(content, emitter));
}

pub fn get_messages() -> Vec<Message> {
    APP_LOGGER.with(|logger| logger.get_all_messages().unwrap_or(Vec::new()))
}
