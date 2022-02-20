use crate::event::Event;
use crate::transition::{
    EmptyTransition, Transition, TransitionError, TransitionErrorKind, TransitionOut,
};
use crate::Vertex;
use std::any::{Any, TypeId};

pub trait Guard<Event> {
    fn check(&self, input: &Event) -> bool;
}

impl<F, Event> Guard<Event> for F
where
    F: Fn(&Event) -> bool,
{
    fn check(&self, input: &Event) -> bool {
        self(input)
    }
}

pub struct GuardedTransition<FEvent, Tr> {
    guards: Vec<Box<dyn Guard<FEvent>>>,
    transition: Tr,
}

impl<Event> GuardedTransition<Event, EmptyTransition> {
    pub fn new() -> Self {
        GuardedTransition {
            guards: vec![],
            transition: EmptyTransition,
        }
    }

    pub fn guard<G: Guard<Event> + 'static>(mut self, guard: G) -> Self {
        self.guards.push(Box::new(guard));
        self
    }

    pub fn transition<NewTr>(self, transition: NewTr) -> GuardedTransition<Event, NewTr> {
        let Self { guards, .. } = self;
        GuardedTransition { guards, transition }
    }
}

impl<FEvent, Tr, State: ?Sized> Transition<State> for GuardedTransition<FEvent, Tr>
where
    FEvent: Any + 'static,
    Tr: Transition<State>,
{
    type Answer = Tr::Answer;

    fn transition(
        &self,
        from: &mut dyn Vertex<State>,
        event: Event,
    ) -> Result<TransitionOut<State, Self::Answer>, TransitionError> {
        let event = event.downcast().map_err(|event| TransitionError {
            event,
            kind: TransitionErrorKind::WrongEvent,
        })?;
        match self.guards.iter().map(|g| g.check(&event)).all(|x| x) {
            true => self.transition.transition(from, event),
            false => Err(TransitionError::new(event, TransitionErrorKind::GuardErr)),
        }
    }
    fn input_tid(&self) -> TypeId {
        self.transition.input_tid()
    }
    fn output_tid(&self) -> TypeId {
        self.transition.output_tid()
    }
}
