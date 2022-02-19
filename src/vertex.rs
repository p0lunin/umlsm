use std::any::TypeId;

pub trait Vertex<State: ?Sized> {
    fn entry(&self);
    fn exit(&self);
    fn get_data(&mut self) -> Box<State>;
    fn get_data_as_ref(&self) -> &State;
    fn set_data(&mut self, data: Box<State>);
    fn data_tid(&self) -> TypeId;
}
