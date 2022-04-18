pub trait Transition<Event> {
    type State: Sized;
    fn transition(self, event: Event) -> Result<Self::State, TransitionError<Self::State, Event>>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum TransitionError<State, Event> {
    NoTransition(State, Event),
}
