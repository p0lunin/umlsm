use std::any::{Any, TypeId};

pub trait Vertex {
    fn entry(&self);
    fn exit(&self);
    fn get_data(&mut self) -> Box<dyn Any>;
    fn set_data(&mut self, data: Box<dyn Any>);
    fn data_tid(&self) -> TypeId;
}