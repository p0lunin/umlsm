use crate::state::Cast;
use crate::vertex::Vertex;
use std::any::{Any, TypeId};
use std::marker::PhantomData;

pub trait Transition<Event, State: ?Sized = dyn Any> {
    type Answer;
    fn transition(
        &self,
        from: &mut dyn Vertex<State>,
        event: Event,
    ) -> Result<TransitionOut<State, Self::Answer>, TransitionError<Event>>;
    fn input_tid(&self) -> TypeId;
    /// This function is used only in the initialization moment to check that state machine contains
    /// necessary output vertex.
    fn output_tid(&self) -> TypeId;
}

pub struct TransitionOut<State: ?Sized, A> {
    pub state: Box<State>,
    pub answer: A,
}

pub struct TransitionError<Event> {
    pub event: Event,
    pub kind: TransitionErrorKind,
}

impl<Event> TransitionError<Event> {
    pub fn new(event: Event, kind: TransitionErrorKind) -> Self {
        TransitionError { event, kind }
    }
}

pub enum TransitionErrorKind {
    GuardErr,
}

pub struct EmptyTransition;

impl<Event, State: ?Sized> Transition<Event, State> for EmptyTransition {
    type Answer = ();

    fn transition(
        &self,
        _: &mut dyn Vertex<State>,
        _: Event,
    ) -> Result<TransitionOut<State, Self::Answer>, TransitionError<Event>> {
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

impl<F, Input, Output, Answer, Event, State> Transition<Event, State>
    for FuncTransition<F, (Input, Event)>
where
    Input: 'static,
    Output: 'static,
    F: Fn(Input, Event) -> (Output, Answer),
    State: ?Sized + Cast<Input> + Cast<Output>,
{
    type Answer = Answer;

    fn transition(
        &self,
        from: &mut dyn Vertex<State>,
        event: Event,
    ) -> Result<TransitionOut<State, Self::Answer>, TransitionError<Event>> {
        from.exit();
        let input = from.get_data().downcast();
        let out = (self.0)(*input, event);
        Ok(TransitionOut {
            state: State::upcast(Box::new(out.0)),
            answer: out.1,
        })
    }

    fn input_tid(&self) -> TypeId {
        TypeId::of::<Input>()
    }

    fn output_tid(&self) -> TypeId {
        TypeId::of::<Output>()
    }
}
