use crate::transition::{TransitionError, TransitionErrorKind, Transition};
use std::any::{TypeId, Any};
use crate::Vertex;

pub trait Guard<Event> {
    fn check(&self, input: &Event) -> bool;
}

impl<F, Event> Guard<Event> for F
where
    F: Fn(&Event) -> bool
{
    fn check(&self, input: &Event) -> bool {
        self(input)
    }
}

pub struct GuardedTransition<Event, Tr> {
    guards: Vec<Box<dyn Guard<Event>>>,
    transition: Tr,
}

impl<Event, Tr> GuardedTransition<Event, Tr> {
    pub fn new(transition: Tr) -> Self {
        GuardedTransition { guards: vec![], transition }
    }

    pub fn guard<G: Guard<Event> + 'static>(mut self, guard: G) -> Self {
        self.guards.push(Box::new(guard));
        self
    }
}

impl<Event, Tr> Transition<Event> for GuardedTransition<Event, Tr>
where
    Tr: Transition<Event>,
{
    type Answer = Tr::Answer;

    fn transition(&self, from: &mut dyn Vertex, event: Event) -> Result<(Box<dyn Any>, Self::Answer), TransitionError<Event>> {
        match self.guards.iter().map(|g| g.check(&event)).all(|x| x) {
            true => {
                self.transition.transition(from, event)
            }
            false => {
                Err(TransitionError::new(event, TransitionErrorKind::GuardErr))
            }
        }
    }
    fn input_tid(&self) -> TypeId {
        self.transition.input_tid()
    }
    fn output_tid(&self) -> TypeId {
        self.transition.output_tid()
    }
}
