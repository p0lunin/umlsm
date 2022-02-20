use crate::event::Event;
use crate::state::Cast;
use crate::vertex::Vertex;
use std::any::{Any, TypeId};
use std::marker::PhantomData;

pub trait Transition<State: ?Sized = dyn Any> {
    fn transition(
        &self,
        from: &mut dyn Vertex<State>,
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

pub enum TransitionErrorKind {
    GuardErr,
    WrongEvent,
}

pub struct EmptyTransition;

impl<State: ?Sized> Transition<State> for EmptyTransition {
    fn transition(
        &self,
        _: &mut dyn Vertex<State>,
        _: Event,
    ) -> Result<TransitionOut<State>, TransitionError> {
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

impl<F, Input, Output, Event, Answer> From<F> for FuncTransition<F, (Input, Event)>
where
    F: Fn(Input, Event) -> (Output, Answer),
    Input: Any,
    Output: Any,
{
    fn from(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Input, Output, FEvent, State> Transition<State> for FuncTransition<F, (Input, FEvent)>
where
    Input: 'static,
    Output: 'static,
    FEvent: Any + 'static,
    F: Fn(Input, FEvent) -> Output,
    State: ?Sized + Cast<Input> + Cast<Output>,
{
    fn transition(
        &self,
        from: &mut dyn Vertex<State>,
        event: Event,
    ) -> Result<TransitionOut<State>, TransitionError> {
        let fevent = event.downcast::<FEvent>().map_err(|e| TransitionError {
            event: e,
            kind: TransitionErrorKind::WrongEvent,
        })?;
        from.exit();
        let input = from.get_data().downcast();
        let out = (self.0)(*input, *fevent);
        Ok(TransitionOut {
            state: State::upcast(Box::new(out)),
        })
    }

    fn input_tid(&self) -> TypeId {
        TypeId::of::<Input>()
    }

    fn output_tid(&self) -> TypeId {
        TypeId::of::<Output>()
    }
}
