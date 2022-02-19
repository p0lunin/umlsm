use crate::vertex::Vertex;
use std::any::{Any, TypeId};
use std::marker::PhantomData;

pub trait Transition<Event> {
    type Answer;
    fn transition(
        &self,
        from: &mut dyn Vertex,
        event: Event,
    ) -> Result<TransitionOut<Self::Answer>, TransitionError<Event>>;
    fn input_tid(&self) -> TypeId;
    /// This function is used only in the initialization moment to check that state machine contains
    /// necessary output vertex.
    fn output_tid(&self) -> TypeId;
}

pub struct TransitionOut<A> {
    pub state: Box<dyn Any>,
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

impl<Event> Transition<Event> for EmptyTransition {
    type Answer = ();

    fn transition(
        &self,
        _: &mut dyn Vertex,
        _: Event,
    ) -> Result<TransitionOut<Self::Answer>, TransitionError<Event>> {
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

impl<F, Input, Output, Answer, Event> Transition<Event> for FuncTransition<F, (Input, Event)>
where
    F: Fn(Input, Event) -> (Output, Answer),
    Input: Any,
    Output: Any,
{
    type Answer = Answer;

    fn transition(
        &self,
        from: &mut dyn Vertex,
        event: Event,
    ) -> Result<TransitionOut<Self::Answer>, TransitionError<Event>> {
        from.exit();
        let input = from
            .get_data()
            .downcast::<Input>()
            .unwrap_or_else(|_| panic!("It is caller task"));
        let out = (self.0)(*input, event);
        Ok(TransitionOut {
            state: Box::new(out.0),
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

pub struct Switch<From, To, Answer> {
    answer: Answer,
    _phantom: PhantomData<(From, To)>,
}

impl<From, To: Default + Any, Answer> Switch<From, To, Answer> {
    pub fn new() -> Switch<From, To, ()> {
        Switch {
            answer: (),
            _phantom: PhantomData,
        }
    }

    pub fn with_answer<A: Clone>(answer: A) -> Switch<From, To, A> {
        Switch {
            answer,
            _phantom: PhantomData,
        }
    }
}

impl<Event, From: 'static, To: Default + Any, A: Clone> Transition<Event> for Switch<From, To, A> {
    type Answer = A;

    fn transition(
        &self,
        from: &mut dyn Vertex,
        _: Event,
    ) -> Result<TransitionOut<Self::Answer>, TransitionError<Event>> {
        from.exit();
        let _ = from
            .get_data()
            .downcast_mut::<From>()
            .unwrap_or_else(|| panic!("It is caller task"));
        Ok(TransitionOut {
            state: Box::new(To::default()),
            answer: self.answer.clone(),
        })
    }

    fn input_tid(&self) -> TypeId {
        todo!()
    }

    fn output_tid(&self) -> TypeId {
        todo!()
    }
}
