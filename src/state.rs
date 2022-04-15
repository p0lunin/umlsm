use crate::vertex::{Vertex};
use std::any::{Any, TypeId};
use crate::{Event, SmError};

pub type State = Box<dyn StateTrait>;

pub trait StateTrait {
    fn transition(&self, event: Event) -> Result<(), SmError<Event>>;
}

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
    fn downcast_ref(self: &Self) -> Option<&From> {
        if self.concrete_tid() == TypeId::of::<From>() {
            unsafe {
                let rf = &*(self as *const Self as *const From);
                Some(rf)
            }
        } else {
            None
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

pub struct SimpleVertex<State> {
    data: Option<Box<State>>,
    tid: TypeId,
}

impl<State> SimpleVertex<State> {
    pub fn new<T: 'static>() -> SimpleVertex<State> {
        SimpleVertex {
            data: None,
            tid: TypeId::of::<T>()
        }
    }
    pub fn with_data<T: 'static>(data: Box<State>) -> SimpleVertex<State> {
        SimpleVertex {
            data: Some(data),
            tid: TypeId::of::<T>()
        }
    }
}
