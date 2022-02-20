use crate::event::Event;
use crate::transition::{Transition, TransitionError, TransitionErrorKind, TransitionOut};
use crate::vertex::Vertex;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Sm<State: ?Sized = dyn Any> {
    state: usize,
    vertexes: Vec<Box<dyn Vertex<State>>>,
    transitions: HashMap<TypeId, Vec<Box<dyn Transition<State>>>>,
}

impl<State> Sm<State>
where
    State: ?Sized + 'static,
{
    /// Creates a new state machine.
    ///
    /// Note that first vertex in the list must be default, otherwise behaviour is unspecified.
    /// Probably sm will fail on first `sm.process()` call.
    pub fn new(
        vertexes: Vec<Box<dyn Vertex<State>>>,
        transitions: HashMap<TypeId, Vec<Box<dyn Transition<State>>>>,
    ) -> Self {
        Sm {
            state: 0,
            vertexes,
            transitions,
        }
    }

    pub fn process<E: Any + 'static>(&mut self, event: E) -> Result<(), SmError<E>> {
        self.process_boxed(Box::new(event)).map_err(|e| match e {
            SmError::NoTransitionSatisfyingEvent(e) => {
                SmError::NoTransitionSatisfyingEvent(*e.downcast().unwrap())
            }
            SmError::NoTransitionsFromThisVertex(e) => {
                SmError::NoTransitionsFromThisVertex(*e.downcast().unwrap())
            }
        })
    }

    pub fn process_boxed(&mut self, event: Event) -> Result<(), SmError<Event>> {
        let state = self.vertexes[self.state].as_mut();
        let state_tid = state.data_tid();

        let transitions = match self.transitions.get(&state_tid) {
            Some(ts) => ts,
            None => return Err(SmError::NoTransitionsFromThisVertex(event)),
        };
        let mut event = event;
        for transition in transitions {
            match transition.transition(state, event) {
                Ok(TransitionOut { state: new_state }) => {
                    let new_vertex = self
                        .find_vertex_by_data_tid(new_state.as_ref().type_id())
                        .expect("It should be checked in the `transition` function");
                    self.vertexes[new_vertex].set_data(new_state);
                    self.vertexes[new_vertex].entry();
                    self.state = new_vertex;
                    return Ok(());
                }
                Err(e) => {
                    let TransitionError {
                        event: event1,
                        kind,
                    } = e;
                    match kind {
                        TransitionErrorKind::GuardErr | TransitionErrorKind::WrongEvent => {
                            event = event1;
                            continue;
                        }
                    }
                }
            }
        }

        Err(SmError::NoTransitionSatisfyingEvent(event))
    }

    pub fn current_state(&self) -> &State {
        self.vertexes[self.state].get_data_as_ref()
    }

    fn find_vertex_by_data_tid(&self, tid: TypeId) -> Option<usize> {
        self.vertexes
            .iter()
            .enumerate()
            .find(|(_, x)| x.data_tid() == tid)
            .map(|(x, _)| x)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SmError<Event> {
    NoTransitionsFromThisVertex(Event),
    NoTransitionSatisfyingEvent(Event),
}
