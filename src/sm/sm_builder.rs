use crate::event::EnterSmEvent;
use crate::sm::sm::Sm;
use crate::state::{Cast, State, StateTrait};
use crate::state::{InitialPseudoState, SimpleVertex};
use crate::transition::Transition;
use crate::vertex::{PseudoState, PseudoStateKind, StateTrait, Vertex};
use crate::SmError;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct SmBuilder<State: ?Sized = dyn StateTrait> {
    states: Vec<SimpleVertex<State>>,
}

impl<State> SmBuilder<State>
where
    State: ?Sized + 'static,
{
    pub fn new<T>(initial: T) -> Self
    where
        State: Cast<T>,
        T: 'static,
    {
        let s = State::upcast(Box::new(initial));
        let states = vec![SimpleVertex::with_data::<T>(s)];
        SmBuilder {
            states,
        }
    }
    pub fn vertex(mut self, vertex: SimpleVertex<State>) -> Self {
        self.states.push(vertex);
        self
    }

    pub fn build(self) -> Result<Sm<State>, SmError<EnterSmEvent>> {
        Sm::new(self.states)
    }
}
