use std::any::Any;

pub type Event = Box<dyn Any>;

#[derive(Debug, PartialEq, Clone)]
pub struct EnterSmEvent;
