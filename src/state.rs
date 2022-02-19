use crate::vertex::Vertex;
use std::any::{Any, TypeId};

pub struct InitialPseudostate;

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

impl<T: 'static> SimpleVertex<T>
where
    Self: Vertex,
{
    pub fn boxed(self) -> Box<dyn Vertex> {
        Box::new(self)
    }
}

impl<T: Any> Vertex for SimpleVertex<T> {
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
    fn get_data(&mut self) -> Box<dyn Any> {
        self.data
            .take()
            .expect("This method must be called only once.")
    }
    fn set_data(&mut self, data: Box<dyn Any>) {
        self.data = Some(data.downcast().expect("It must guaranteed by the caller"))
    }
    fn data_tid(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
