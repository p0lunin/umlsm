use crate::event::Event;
use crate::sm::sm::Sm;
use crate::state::SimpleVertex;
use crate::state::{Cast, InitialPseudostate};
use crate::transition::{Transition, TransitionError, TransitionErrorKind, TransitionOut};
use crate::vertex::Vertex;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct SmBuilder<Answer, State: ?Sized = dyn Any> {
    vertexes: Vec<Box<dyn Vertex<State>>>,
    transitions: HashMap<TypeId, Vec<Box<dyn Transition<State, Answer = Answer>>>>,
}

impl<Answer, State> SmBuilder<Answer, State>
where
    State: ?Sized + 'static,
{
    pub fn new() -> Self
    where
        State: Cast<InitialPseudostate>,
    {
        let vertexes = vec![Box::new(SimpleVertex::with_data(InitialPseudostate)) as _];
        let transitions = HashMap::new();
        SmBuilder {
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

    pub fn build(self) -> Sm<Answer, State> {
        Sm::new(self.vertexes, self.transitions)
    }

    fn find_vertex_by_data_tid(&self, tid: TypeId) -> Option<usize> {
        self.vertexes
            .iter()
            .enumerate()
            .find(|(_, x)| x.data_tid() == tid)
            .map(|(x, _)| x)
    }
}
