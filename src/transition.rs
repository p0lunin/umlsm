use crate::event::Event;
use crate::state::Cast;
use crate::vertex::{StateTrait, Vertex};
use crate::Sm;
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::process::Output;

pub struct Switch<From, Event, To> {
    to: To,
    _phantom: PhantomData<(Event, From)>,
}

impl<F, E, To: Clone> Switch<F, E, To> {
    pub fn new(to: To) -> Switch<F, E, To> {
        Switch {
            to,
            _phantom: PhantomData,
        }
    }
}

impl<DynData, From, E, To> Transition<DynData> for Switch<From, E, To>
where
    From: 'static,
    To: Clone + 'static,
    DynData: ?Sized + Cast<From> + Cast<To> + Cast<Sm<DynData>>,
    E: 'static,
{
    fn transition(
        &self,
        from: &mut Vertex<DynData>,
        event: Event,
    ) -> Result<TransitionOut<DynData>, TransitionError> {
        event.downcast::<E>().map_err(|e| TransitionError {
            event: e,
            kind: TransitionErrorKind::WrongEvent,
        })?;
        from.exit();
        from.get_data();
        Ok(TransitionOut {
            state: DynData::upcast(Box::new(self.to.clone())),
        })
    }

    fn input_tid(&self) -> TypeId {
        TypeId::of::<From>()
    }

    fn output_tid(&self) -> TypeId {
        TypeId::of::<To>()
    }
}

pub trait Transition<State: ?Sized = dyn Any> {
    fn transition(
        &self,
        from: &mut Vertex<State>,
        event: Event,
    ) -> Result<TransitionOut<State>, TransitionError>;
    fn input_tid(&self) -> TypeId;
    /// This function is used only in the initialization moment to check that state machine contains
    /// necessary output vertex.
    fn output_tid(&self) -> TypeId;
}

pub struct TransitionOut<State: ?Sized> {
    pub state: Box<State>,
}

pub struct TransitionError {
    pub event: Event,
    pub kind: TransitionErrorKind,
}

impl TransitionError {
    pub fn new(event: Event, kind: TransitionErrorKind) -> Self {
        TransitionError { event, kind }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TransitionErrorKind {
    GuardErr,
    WrongEvent,
}

pub struct EmptyTransition;

impl<DynData: ?Sized> Transition<DynData> for EmptyTransition {
    fn transition(
        &self,
        _: &mut Vertex<DynData>,
        _: Event,
    ) -> Result<TransitionOut<DynData>, TransitionError> {
        unreachable!("It seems you forgot to initialize transition for something.")
    }

    fn input_tid(&self) -> TypeId {
        unreachable!("It seems you forgot to initialize transition for something.")
    }

    fn output_tid(&self) -> TypeId {
        unreachable!("It seems you forgot to initialize transition for something.")
    }
}

pub struct FuncTransition<F, Args>(F, PhantomData<Args>);

pub fn ftrans<F: Into<FuncTransition<F, Args>>, Args>(f: F) -> FuncTransition<F, Args> {
    f.into()
}

impl<F, Input, Output, Event> From<F> for FuncTransition<F, (Input, Event)>
where
    F: Fn(Input, Event) -> Output,
    Input: Any,
    Output: Any,
{
    fn from(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Input, Output, FEvent, DynData> Transition<DynData> for FuncTransition<F, (Input, FEvent)>
where
    Input: 'static,
    Output: 'static,
    FEvent: Any + 'static,
    F: Fn(Input, FEvent) -> Output,
    DynData: ?Sized + Cast<Input> + Cast<Output> + Cast<Sm<DynData>>,
{
    fn transition(
        &self,
        from: &mut Vertex<DynData>,
        event: Event,
    ) -> Result<TransitionOut<DynData>, TransitionError> {
        let fevent = event.downcast::<FEvent>().map_err(|e| TransitionError {
            event: e,
            kind: TransitionErrorKind::WrongEvent,
        })?;
        from.exit();
        let input = from.get_data().downcast();
        let out = (self.0)(*input, *fevent);
        Ok(TransitionOut {
            state: DynData::upcast(Box::new(out)),
        })
    }

    fn input_tid(&self) -> TypeId {
        TypeId::of::<Input>()
    }

    fn output_tid(&self) -> TypeId {
        TypeId::of::<Output>()
    }
}
