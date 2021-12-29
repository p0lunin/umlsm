use std::any::{TypeId, Any};
use crate::vertex::Vertex;
use std::marker::PhantomData;

pub trait Transition<Event> {
    type Answer;
    fn transition(&self, from: &mut dyn Vertex, event: Event) -> Result<(Box<dyn Any>, Self::Answer), TransitionError<Event>>;
    fn input_tid(&self) -> TypeId;
    fn output_tid(&self) -> TypeId;
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

    fn transition(&self, from: &mut dyn Vertex, event: Event) -> Result<(Box<dyn Any>, Answer), TransitionError<Event>> {
        from.exit();
        // TODO: remove Any::is check because it must be done by caller.
        let input = from.get_data().downcast::<Input>()
            .unwrap_or_else(|_| panic!("It is caller task"));
        let out = (self.0)(*input, event);
        Ok((Box::new(out.0), out.1))
    }

    fn input_tid(&self) -> TypeId {
        TypeId::of::<Input>()
    }

    fn output_tid(&self) -> TypeId {
        TypeId::of::<Output>()
    }
}
