use crate::sm::sm::Sm;
use crate::state::{InitialPseudoState, SimpleVertex};
use crate::state::{Cast};
use crate::transition::Transition;
use crate::vertex::{PseudoState, PseudoStateKind, StateTrait, Vertex};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::event::EnterSmEvent;
use crate::SmError;

pub struct SmBuilder<DynData: ?Sized = dyn Any> {
    vertexes: Vec<Vertex<DynData>>,
    transitions: HashMap<TypeId, Vec<Box<dyn Transition<DynData>>>>,
}

impl<DynData> SmBuilder<DynData>
where
    DynData: ?Sized + 'static,
{
    pub fn new() -> Self
    where
        DynData: Cast<InitialPseudoState>
    {
        let vertexes = vec![Vertex::PseudoState(PseudoState::new(
            Some(Box::new(InitialPseudoState)),
            PseudoStateKind::Initial,
        ))];
        let transitions = HashMap::new();
        SmBuilder {
            vertexes,
            transitions,
        }
    }
    pub fn register_vertex(mut self, vertex: Vertex<DynData>) -> Self {
        self.vertexes.push(vertex);
        self
    }
    pub fn transition<T: Transition<DynData> + 'static>(mut self, transition: T) -> Self
    where
        DynData: Cast<Sm<DynData>>,
    {
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

    pub fn build(self) -> Result<Sm<DynData>, SmError<EnterSmEvent>>
    where
        DynData: Cast<Sm<DynData>>
    {
        Sm::new(self.vertexes, self.transitions)
    }

    fn find_vertex_by_data_tid(&self, tid: TypeId) -> Option<usize>
    where
        DynData: Cast<Sm<DynData>>,
    {
        self.vertexes
            .iter()
            .enumerate()
            .find(|(_, x)| x.data_tid() == tid)
            .map(|(x, _)| x)
    }
}
