use std::collections::HashMap;
use std::cell::RefCell;
use std::time::SystemTime;
use colored::*;

#[derive(PartialEq, Eq, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
pub enum MessageEmitter {
    Object(String),
    Engine,
    Renderer
}

#[derive(Clone)]
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
        let emitter = match &self.emitter {
            MessageEmitter::Object(e) => e,
            MessageEmitter::Engine => "Engine",
            MessageEmitter::Renderer => "Renderer"
        };

        match self.level {
            LogLevel::Debug => { 
                let msg = format!("DEBUG [{} | {:?}]: {}", emitter, self.time, self.content).dimmed();
                println!("{}", msg);
            },
            LogLevel::Info => { 
                let msg = format!("INFO [{} | {:?}]: {}", emitter, self.time, self.content);
                println!("{}", msg);
            },
            LogLevel::Warning => { 
                let msg = format!("WARNING [{} | {:?}]: {}", emitter, self.time, self.content).yellow();
                println!("{}", msg);
            },
            LogLevel::Error => { 
                let msg = format!("ERROR [{} | {:?}]: {}", emitter, self.time, self.content).red();
                println!("{}", msg);
            },
        }
    }
}

pub struct Log {
    messages: Option<HashMap<SystemTime, Message>>,
}

pub trait Logger {
    fn log_debug(&self, content: String, emitter: MessageEmitter);
    fn log_info(&self, content: String, emitter: MessageEmitter);
    fn log_warning(&self, content: String, emitter: MessageEmitter);
    fn log_error(&self, content: String, emitter: MessageEmitter);
    fn log(&self, message: Message);
    fn get_level(&self, level: LogLevel) -> Option<Vec<Message>>;
    fn filter_messages(&self, filter: &dyn Fn(&&Message) -> bool) -> Option<Vec<Message>>;
}

impl Logger for RefCell<Log> {
    fn log_debug(&self, content: String, emitter: MessageEmitter) {
        let msg = Message::new(content, LogLevel::Debug, emitter);
        self.log(msg);
    }
    
    fn log_info(&self, content: String, emitter: MessageEmitter) {
        let msg = Message::new(content, LogLevel::Info, emitter);
        self.log(msg);
    }
    
    fn log_warning(&self, content: String, emitter: MessageEmitter) {
        let msg = Message::new(content, LogLevel::Warning, emitter);
        self.log(msg);
    }

    fn log_error(&self, content: String, emitter: MessageEmitter) {
        let msg = Message::new(content, LogLevel::Error, emitter);
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
}

pub const APP_LOGGER: RefCell<Log> = RefCell::new(Log { messages: None });

