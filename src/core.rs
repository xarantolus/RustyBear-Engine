
use crate::utils::Timestep;
use crate::window::Window;
use crate::event::{EventSubscriber, EventStack, Event};

use std::string::String;

pub trait Module {
    fn init(&mut self);
    fn update(&mut self, delta: &Timestep);
    fn quit(&mut self);
}

pub trait Windowable {
    fn create_window(&mut self, name: String) -> Window;
} 

pub trait Application {
    fn init(&mut self, config_json: String, stack: &mut ApplicationStack);
    fn update(&mut self, delta: &Timestep);
    fn quit(&mut self);
}

pub struct ApplicationStack<'a> {
    events: EventStack<'a>,
    modules: Vec<Box<dyn Module + 'a>>,
}

impl<'a> ApplicationStack<'a> {

    pub fn new() -> ApplicationStack<'a>
    {
        ApplicationStack { events: EventStack::new(), modules: Vec::new() }
    }

    pub fn push(&mut self, module: impl Module + 'a)
    {
        self.modules.push(Box::new(module));
    }

    fn update(&mut self, ts: &Timestep)
    {
        for mods in &mut self.modules
        {
            mods.update(ts);
        }
    } 

    pub fn dispatch_event(&mut self, input_stack: bool, event: &Event)
    {
        self.events.propagate_event(input_stack, event);
    }

    pub fn subscribe(&mut self, input_stack: bool, mut subscriber: impl EventSubscriber + 'a) 
    {
        self.events.push(input_stack, move |event: &Event| { subscriber.on_event(event) });
    }
}

