use crate::state::{Cast, SimpleVertex};
use crate::Sm;
use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};

/// Possible vertexes.
///
/// Default ABI of states (unless otherwise specified):
/// 1. There are can be multiple instances of the state.
/// 2. There are can be multiple transitions *to* this state.
/// 3. There are can be multiple transitions *from* this state.
pub enum Vertex<DynData: ?Sized> {
    State(Box<dyn StateTrait<DynData>>),
    SubMachineState(SimpleVertex<Sm<DynData>>),
    PseudoState(PseudoState<DynData>),
}

impl<DynData> Debug for Vertex<DynData> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vertex").finish()
    }
}

impl<DynData: ?Sized> StateTrait<DynData> for Vertex<DynData>
where
    DynData: Cast<Sm<DynData>>,
{
    fn entry(&self) {
        match self {
            Vertex::State(s) => s.entry(),
            Vertex::SubMachineState(sm) => sm.entry(),
            Vertex::PseudoState(ps) => ps.entry(),
        }
    }
    fn exit(&self) {
        match self {
            Vertex::State(s) => s.exit(),
            Vertex::SubMachineState(sm) => sm.exit(),
            Vertex::PseudoState(ps) => ps.exit(),
        }
    }

    fn get_data(&mut self) -> Box<DynData> {
        match self {
            Vertex::State(s) => s.get_data(),
            Vertex::SubMachineState(_) => unreachable!(),
            Vertex::PseudoState(ps) => ps.get_data(),
        }
    }

    fn get_data_as_ref(&self) -> &DynData {
        match self {
            Vertex::State(s) => s.get_data_as_ref(),
            Vertex::SubMachineState(_) => unreachable!(),
            Vertex::PseudoState(ps) => ps.get_data_as_ref(),
        }
    }

    fn set_data(&mut self, data: Box<DynData>) {
        match self {
            Vertex::State(s) => s.set_data(data),
            Vertex::SubMachineState(_) => unreachable!(),
            Vertex::PseudoState(ps) => ps.set_data(data),
        }
    }

    fn data_tid(&self) -> TypeId {
        match self {
            Vertex::State(s) => s.data_tid(),
            Vertex::SubMachineState(sm) => unreachable!(),
            Vertex::PseudoState(ps) => ps.data_tid(),
        }
    }
}

pub struct PseudoState<DynData: ?Sized> {
    pub(crate) data: Option<Box<DynData>>,
    pub(crate) data_tid: TypeId,
    pub(crate) kind: PseudoStateKind,
}

impl<DynData: ?Sized> PseudoState<DynData> {
    pub fn new<T: 'static>(data: Option<Box<T>>, kind: PseudoStateKind) -> Self
    where
        DynData: Cast<T>,
    {
        PseudoState {
            data: data.map(|x| DynData::upcast(x)),
            data_tid: TypeId::of::<T>(),
            kind,
        }
    }
}

impl<DynData: ?Sized> StateTrait<DynData> for PseudoState<DynData> {
    fn entry(&self) {
        match &self.kind {
            PseudoStateKind::Initial => {}
            PseudoStateKind::Terminate => {}
            PseudoStateKind::Entry(action) => action.perform_action(),
            PseudoStateKind::Exit(_) => {}
        }
    }

    fn exit(&self) {
        match &self.kind {
            PseudoStateKind::Initial => {}
            PseudoStateKind::Terminate => {}
            PseudoStateKind::Entry(_) => {}
            PseudoStateKind::Exit(action) => action.perform_action(),
        }
    }

    fn get_data(&mut self) -> Box<DynData> {
        self.data
            .take()
            .expect("This method must be called only once.")
    }

    fn get_data_as_ref(&self) -> &DynData {
        self.data
            .as_ref()
            .expect("This method must be called only once.")
    }

    fn set_data(&mut self, data: Box<DynData>) {
        self.data = Some(data)
    }
    fn data_tid(&self) -> TypeId {
        self.data_tid
    }
}

pub enum PseudoStateKind {
    /// Points to the initial state of the State machine.
    ///
    /// ABI:
    /// 1. There are can be only one `Initial` state.
    /// 2. There are can be only one transition `Switch` *from* state.
    Initial,
    /// State that means that state machine is exited and cannot be used.
    ///
    /// ABI:
    /// 1. Only 0 transitions from this state is allowed.
    Terminate,
    /// Contains action that will be called when transition enters this pseudo-state.
    ///
    /// ABI:
    /// 1. There are can be only one transition *from* this state.
    Entry(Box<dyn ActionPoint>),
    /// Contains action that will be called when transition exited this pseudo-state.
    ///
    /// ABI:
    /// 1. There are can be only one transition *from* this state.
    Exit(Box<dyn ActionPoint>),
}

pub trait StateTrait<DynData: ?Sized> {
    fn entry(&self);
    fn exit(&self);
    fn get_data(&mut self) -> Box<DynData>;
    fn get_data_as_ref(&self) -> &DynData;
    fn set_data(&mut self, data: Box<DynData>);
    fn data_tid(&self) -> TypeId;
}

pub trait ActionPoint {
    fn perform_action(&self);
}
