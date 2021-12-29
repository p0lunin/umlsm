use std::collections::HashMap;
use std::any::{TypeId, Any};
use std::marker::PhantomData;
use downcast_rs::Downcast;
use downcast_rs::impl_downcast;

pub trait Vertex: Downcast {
    fn entry(&self, _event: Box<dyn Any>);
    fn exit(&self, _event: Box<dyn Any>);
    fn data(self: Box<Self>) -> Box<dyn Any>;
    fn data_tid(&self) -> TypeId { self.as_any().type_id() }
}
impl_downcast!(Vertex);

pub struct InitialPseudostate;

impl Vertex for InitialPseudostate {
    fn entry(&self, _event: Box<dyn Any>) {}
    fn exit(&self, _event: Box<dyn Any>) {}
    fn data(self: Box<Self>) -> Box<dyn Any> { self }
}

#[repr(transparent)]
struct State<T>(T);

impl<T: Any> Vertex for State<T> {
    fn entry(&self, _event: Box<dyn Any>) {}
    fn exit(&self, _event: Box<dyn Any>) {}
    fn data(self: Box<Self>) -> Box<dyn Any> {
        unsafe {
            let b: Box<T> = std::mem::transmute(self);
            b
        }
    }
}

impl InitialPseudostate {
    fn boxed() -> Box<dyn Vertex> {
        Box::new(Self)
    }
}

pub trait Transition<Event> {
    fn transition(&self, from: Box<dyn Vertex>, event: Event) -> Result<Box<dyn Vertex>, TransitionError<Event>>;
    fn input_tid(&self) -> TypeId;
    fn output_tid(&self) -> TypeId;
}

pub struct TransitionError<Event> {
    from: Box<dyn Vertex>,
    event: Event,
    kind: TransitionErrorKind,
}

pub enum TransitionErrorKind {
    GuardErr,
}

pub struct FuncTransition<F, Args>(F, PhantomData<Args>);

fn ftrans<F: Into<FuncTransition<F, Args>>, Args>(f: F) -> FuncTransition<F, Args> {
    f.into()
}

impl<F, Input, Output, Event> From<F> for FuncTransition<F, (Input, Event)>
where
    F: Fn(Input, Event) -> Output,
    Input: Vertex,
    Output: Vertex,
{
    fn from(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Input, Output, Event> Transition<Event> for FuncTransition<F, (Input, Event)>
where
    F: Fn(Input, Event) -> Output,
    Input: Vertex,
    Output: Vertex,
{
    fn transition(&self, from: Box<dyn Vertex>, event: Event) -> Result<Box<dyn Vertex>, TransitionError<Event>> {
        // TODO: remove Any::is check because it must be done by caller.
        let input = from.data().downcast::<Input>()
            .unwrap_or_else(|_| panic!("It is caller task"));
        let out = (self.0)(*input, event);
        Ok(Box::new(out))
    }

    fn input_tid(&self) -> TypeId {
        TypeId::of::<Input>()
    }

    fn output_tid(&self) -> TypeId {
        TypeId::of::<Output>()
    }
}

pub struct StateMachine<Event> {
    state: Option<Box<dyn Vertex>>,
    vertexes: Vec<Box<dyn Vertex>>,
    transitions: HashMap<TypeId, Vec<Box<dyn Transition<Event>>>>,
}

impl<Event> StateMachine<Event> {
    pub fn new() -> Self {
        let state = Some(InitialPseudostate::boxed());
        let vertexes = vec![InitialPseudostate::boxed()];
        let transitions = HashMap::new();
        StateMachine { state, vertexes, transitions }
    }
    pub fn register_vertex(mut self, vertex: Box<dyn Vertex>) -> Self {
        self.vertexes.push(vertex);
        self
    }
    pub fn transition<T: Transition<Event> + 'static>(mut self, transition: T) -> Self {
        let trans = Box::new(transition);

        self.transitions.entry(trans.input_tid())
            .or_default()
            .push(trans);
        self
    }
    pub fn process(&mut self, mut event: Event) -> Result<(), StateMachineError> {
        let mut state = self.state.take().expect("It should be Some()");
        let state_tid = state.data_tid();

        let transitions = self.transitions.get(&state_tid).ok_or(StateMachineError::NoTransition)?;
        for transition in transitions {
            match transition.transition(state, event) {
                Ok(new_state) => {
                    self.state = Some(new_state);
                    return Ok(());
                }
                Err(e) => {
                    let TransitionError { from: from1, event: event1, kind } = e;
                    match kind {
                        TransitionErrorKind::GuardErr => {
                            state = from1;
                            event = event1;
                            continue
                        },
                    }
                }
            }
        }

        Err(StateMachineError::NoTransition)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StateMachineError {
    NoTransition,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        struct SomeState;

        let mut machine = StateMachine::new()
            .transition(ftrans(|_: InitialPseudostate, _event: i32| {
                State(SomeState)
            }));

        assert_eq!(machine.process(1), Ok(()));
    }
}
