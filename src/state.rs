use crate::vertex::{StateTrait, Vertex};
use std::any::{Any, TypeId};

pub trait Cast<From: 'static>: Any {
    fn upcast(from: Box<From>) -> Box<Self>;
    fn upcast_ref(from: &From) -> &Self;
    fn concrete_tid(&self) -> TypeId;
    fn downcast(self: Box<Self>) -> Box<From> {
        if self.concrete_tid() == TypeId::of::<From>() {
            unsafe {
                let raw: *mut Self = Box::<Self>::into_raw(self);
                Box::from_raw(raw as *mut From)
            }
        } else {
            panic!("Must be guaranteed by the caller.")
        }
    }
}

impl<T: Any> Cast<T> for dyn Any {
    fn upcast(from: Box<T>) -> Box<Self> {
        from
    }

    fn upcast_ref(from: &T) -> &Self {
        from
    }

    fn concrete_tid(&self) -> TypeId {
        self.type_id()
    }

    fn downcast(self: Box<Self>) -> Box<T> {
        self.downcast().expect("Must be guaranteed by caller.")
    }
}

#[derive(Debug, PartialEq)]
pub struct InitialPseudoState;

pub struct SimpleVertex<T> {
    data: Option<Box<T>>,
    entry: Box<dyn for<'a> Fn(&'a T)>,
    exit: Box<dyn for<'a> Fn(&'a T)>,
}

fn do_nothing<T>(_: &T) {}

impl<T: 'static> SimpleVertex<T> {
    pub fn new() -> SimpleVertex<T> {
        SimpleVertex {
            data: None,
            entry: Box::new(do_nothing),
            exit: Box::new(do_nothing),
        }
    }

    pub fn with_data(data: T) -> SimpleVertex<T> {
        SimpleVertex {
            data: Some(Box::new(data)),
            entry: Box::new(do_nothing),
            exit: Box::new(do_nothing),
        }
    }
}

impl<T> SimpleVertex<T> {
    pub fn with_entry(self, entry: impl for<'a> Fn(&'a T) + 'static) -> SimpleVertex<T> {
        SimpleVertex {
            entry: Box::new(entry),
            ..self
        }
    }
}

impl<T> SimpleVertex<T> {
    pub fn with_exit(self, exit: impl for<'a> Fn(&'a T) + 'static) -> SimpleVertex<T> {
        SimpleVertex {
            exit: Box::new(exit),
            ..self
        }
    }
}

impl<T: 'static> SimpleVertex<T> {
    pub fn to_vertex<DynData: Cast<T> + ?Sized>(self) -> Vertex<DynData> {
        Vertex::State(Box::new(self))
    }
}

impl<T, DynData> StateTrait<DynData> for SimpleVertex<T>
where
    T: 'static,
    DynData: Cast<T> + ?Sized,
{
    fn entry(&self) {
        (self.entry)(
            &self
                .data
                .as_ref()
                .expect("It must be guaranteed by the caller")
                .as_ref(),
        );
    }
    fn exit(&self) {
        (self.exit)(
            &self
                .data
                .as_ref()
                .expect("It must be guaranteed by the caller")
                .as_ref(),
        );
    }
    fn get_data(&mut self) -> Box<DynData> {
        self.data
            .take()
            .map(|x| DynData::upcast(x))
            .expect("This method must be called only once.")
    }

    fn get_data_as_ref(&self) -> &DynData {
        self.data
            .as_ref()
            .map(|x| DynData::upcast_ref(x))
            .expect("This method must be called only once.")
    }

    fn set_data(&mut self, data: Box<DynData>) {
        self.data = Some(data.downcast())
    }
    fn data_tid(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
