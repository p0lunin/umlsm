use crate::vertex::Vertex;
use std::any::{Any, TypeId};

pub struct InitialPseudostate;

pub struct State<T, Entry, Exit> {
    data: Option<Box<T>>,
    entry: Entry,
    exit: Exit,
}

fn do_nothing<T>(_: &T) {}

impl<T> State<T, (), ()> {
    pub fn new(data: T) -> State<T, impl Fn(&T), impl Fn(&T)> {
        State { data: Some(Box::new(data)), entry: do_nothing, exit: do_nothing }
    }

    pub fn empty<T1>() -> State<T1, impl Fn(&T), impl Fn(&T)> {
        State { data: None, entry: do_nothing, exit: do_nothing }
    }
}

impl<T, Entry1, Exit> State<T, Entry1, Exit> {
    pub fn entry<Entry>(self, entry: Entry) -> State<T, Entry, Exit> {
        let State { data, exit, .. } = self;
        State { entry, data, exit }
    }
}

impl<T, Entry, Exit1> State<T, Entry, Exit1> {
    pub fn exit<Exit>(self, exit: Exit) -> State<T, Entry, Exit> {
        let State { data, entry, .. } = self;
        State { exit, data, entry }
    }
}

impl<T: 'static, Entry: 'static, Exit: 'static> State<T, Entry, Exit>
where
    Self: Vertex,
{
    pub fn boxed(self) -> Box<dyn Vertex> {
        Box::new(self)
    }
}

impl<T: Any, Entry, Exit> Vertex for State<T, Entry, Exit>
where
    Entry: Fn(&T) + 'static,
    Exit: Fn(&T) + 'static,
{
    fn entry(&self) { (self.entry)(&self.data.as_ref().expect("It must guaranteed by the caller").as_ref()); }
    fn exit(&self) { (self.exit)(&self.data.as_ref().expect("It must guaranteed by the caller").as_ref()); }
    fn get_data(&mut self) -> Box<dyn Any> {
        self.data.take().expect("This method must be called only once.")
    }
    fn set_data(&mut self, data: Box<dyn Any>) {
        self.data = Some(data.downcast().expect("It must guaranteed by the caller"))
    }
    fn data_tid(&self) -> TypeId { TypeId::of::<T>() }
}