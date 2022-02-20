use crate::event::Event;
use crate::state::SimpleVertex;
use crate::state::{Cast, InitialPseudostate};
use crate::transition::{Transition, TransitionError, TransitionErrorKind, TransitionOut};
use crate::vertex::Vertex;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct StateMachine<Answer, State: ?Sized = dyn Any> {
    state: usize,
    vertexes: Vec<Box<dyn Vertex<State>>>,
    transitions: HashMap<TypeId, Vec<Box<dyn Transition<State, Answer = Answer>>>>,
}

impl<Answer, State> StateMachine<Answer, State>
where
    State: ?Sized + 'static,
{
    pub fn new() -> Self
    where
        State: Cast<InitialPseudostate>,
    {
        let vertexes = vec![Box::new(SimpleVertex::with_data(InitialPseudostate)) as _];
        let transitions = HashMap::new();
        StateMachine {
            state: 0,
            vertexes,
            transitions,
        }
    }
    pub fn with_default_state(mut vertex: Box<dyn Vertex<State>>) -> Self {
        let data = vertex.get_data();
        vertex.set_data(data);

        let vertexes = vec![vertex];
        let transitions = HashMap::new();
        StateMachine {
            state: 0,
            vertexes,
            transitions,
        }
    }
    pub fn register_vertex(mut self, vertex: Box<dyn Vertex<State>>) -> Self {
        self.vertexes.push(vertex);
        self
    }
    pub fn transition<T: Transition<State, Answer = Answer> + 'static>(
        mut self,
        transition: T,
    ) -> Self {
        assert!(
            self.find_vertex_by_data_tid(transition.input_tid())
                .is_some(),
            "Not found input vertex!"
        );
        assert!(
            self.find_vertex_by_data_tid(transition.output_tid())
                .is_some(),
            "Not found output vertex!"
        );

        let trans = Box::new(transition);
        self.transitions
            .entry(trans.input_tid())
            .or_default()
            .push(trans);
        self
    }

    pub fn process<E: Any + 'static>(&mut self, event: E) -> Result<Answer, SmError<E>> {
        self.process_boxed(Box::new(event)).map_err(|e| match e {
            SmError::NoTransitionSatisfyingEvent(e) => {
                SmError::NoTransitionSatisfyingEvent(*e.downcast().unwrap())
            }
            SmError::NoTransitionsFromThisVertex(e) => {
                SmError::NoTransitionsFromThisVertex(*e.downcast().unwrap())
            }
        })
    }

    pub fn process_boxed(&mut self, event: Event) -> Result<Answer, SmError<Event>> {
        let state = self.vertexes[self.state].as_mut();
        let state_tid = state.data_tid();

        let transitions = match self.transitions.get(&state_tid) {
            Some(ts) => ts,
            None => return Err(SmError::NoTransitionsFromThisVertex(event)),
        };
        let mut event = event;
        for transition in transitions {
            match transition.transition(state, event) {
                Ok(TransitionOut {
                    state: new_state,
                    answer,
                }) => {
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
