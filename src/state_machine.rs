use crate::state::InitialPseudostate;
use crate::state::State;
use crate::transition::{Transition, TransitionError, TransitionErrorKind};
use crate::vertex::Vertex;
use std::any::TypeId;
use std::collections::HashMap;

pub struct StateMachine<Event, Answer> {
    state: usize,
    vertexes: Vec<Box<dyn Vertex>>,
    transitions: HashMap<TypeId, Vec<Box<dyn Transition<Event, Answer = Answer>>>>,
}

impl<Event, Answer> StateMachine<Event, Answer> {
    pub fn new() -> Self {
        let vertexes = vec![Box::new(State::new(InitialPseudostate)) as _];
        let transitions = HashMap::new();
        StateMachine {
            state: 0,
            vertexes,
            transitions,
        }
    }
    pub fn register_vertex(mut self, vertex: Box<dyn Vertex>) -> Self {
        self.vertexes.push(vertex);
        self
    }
    pub fn transition<T: Transition<Event, Answer = Answer> + 'static>(
        mut self,
        transition: T,
    ) -> Self {
        assert!(
            self.find_vertex_by_data_tid(transition.input_tid())
                .is_some(),
            "Not found input vertex!"
        );
        for tid in transition.output_tids() {
            assert!(
                self.find_vertex_by_data_tid(tid).is_some(),
                "Not found one of output vertices!"
            );
        }

        let trans = Box::new(transition);
        self.transitions
            .entry(trans.input_tid())
            .or_default()
            .push(trans);
        self
    }
    pub fn process(&mut self, mut event: Event) -> Result<Answer, StateMachineError> {
        let state = self.vertexes[self.state].as_mut();
        let state_tid = state.data_tid();

        let transitions = self
            .transitions
            .get(&state_tid)
            .ok_or(StateMachineError::NoTransitions)?;
        for transition in transitions {
            match transition.transition(state, event) {
                Ok((new_state, answer)) => {
                    let new_vertex = self
                        .find_vertex_by_data_tid(new_state.as_ref().type_id())
                        .expect("It should be checked in the `transition` function");
                    self.vertexes[new_vertex].set_data(new_state);
                    self.vertexes[new_vertex].entry();
                    self.state = new_vertex;
                    return Ok(answer);
                }
                Err(e) => {
                    let TransitionError {
                        event: event1,
                        kind,
                    } = e;
                    match kind {
                        TransitionErrorKind::GuardErr => {
                            event = event1;
                            continue;
                        }
                    }
                }
            }
        }

        Err(StateMachineError::NoTransition)
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
pub enum StateMachineError {
    NoTransitions,
    NoTransition,
}
