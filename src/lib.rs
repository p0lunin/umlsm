use std::collections::HashMap;
use std::any::{TypeId, Any};
use std::marker::PhantomData;
use downcast_rs::Downcast;
use downcast_rs::impl_downcast;

pub trait Vertex: Downcast {
    fn entry(&self);
    fn exit(&self);
    fn get_data(&mut self) -> Box<dyn Any>;
    fn set_data(&mut self, data: Box<dyn Any>);
    fn data_tid(&self) -> TypeId { self.as_any().type_id() }
}
impl_downcast!(Vertex);

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

pub struct InitialPseudostate;

pub struct State<T, Entry, Exit> {
    data: Option<Box<T>>,
    entry: Entry,
    exit: Exit,
}

fn do_nothing<T>(_: &T) {}

impl<T> State<T, (), ()> {
    pub fn new(data: T) -> State<T, impl Fn(&T), impl Fn(&T)> {
        State { data: Some(Box::new(data)), entry: do_nothing, exit: do_nothing }
    }

    pub fn empty<T1>() -> State<T1, impl Fn(&T), impl Fn(&T)> {
        State { data: None, entry: do_nothing, exit: do_nothing }
    }
}

impl<T, Entry1, Exit> State<T, Entry1, Exit> {
    pub fn entry<Entry>(self, entry: Entry) -> State<T, Entry, Exit> {
        let State { data, exit, .. } = self;
        State { entry, data, exit }
    }
}

impl<T, Entry, Exit1> State<T, Entry, Exit1> {
    pub fn exit<Exit>(self, exit: Exit) -> State<T, Entry, Exit> {
        let State { data, entry, .. } = self;
        State { exit, data, entry }
    }
}

impl<T, Entry, Exit1> State<T, Entry, Exit1>
where
    Self: Vertex,
{
    pub fn boxed(self) -> Box<dyn Vertex> {
        Box::new(self)
    }
}

impl<T: Any, Entry, Exit> Vertex for State<T, Entry, Exit>
where
    Entry: Fn(&T) + 'static,
    Exit: Fn(&T) + 'static,
{
    fn entry(&self) { (self.entry)(&self.data.as_ref().expect("It must guaranteed by the caller").as_ref()); }
    fn exit(&self) { (self.exit)(&self.data.as_ref().expect("It must guaranteed by the caller").as_ref()); }
    fn get_data(&mut self) -> Box<dyn Any> {
        self.data.take().expect("This method must be called only once.")
    }
    fn set_data(&mut self, data: Box<dyn Any>) {
        self.data = Some(data.downcast().expect("It must guaranteed by the caller"))
    }
    fn data_tid(&self) -> TypeId { TypeId::of::<T>() }
}

pub trait Transition<Event> {
    type Answer;
    fn transition(&self, from: &mut dyn Vertex, event: Event) -> Result<(Box<dyn Any>, Self::Answer), TransitionError<Event>>;
    fn input_tid(&self) -> TypeId;
    fn output_tid(&self) -> TypeId;
}

pub struct TransitionError<Event> {
    event: Event,
    kind: TransitionErrorKind,
}

impl<Event> TransitionError<Event> {
    pub fn new(event: Event, kind: TransitionErrorKind) -> Self {
        TransitionError { event, kind }
    }
}

pub enum TransitionErrorKind {
    GuardErr,
}

pub struct FuncTransition<F, Args>(F, PhantomData<Args>);

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

pub fn ftrans<F: Into<FuncTransition<F, Args>>, Args>(f: F) -> FuncTransition<F, Args> {
    f.into()
}

impl<F, Input, Output, Event, Answer> From<F> for FuncTransition<F, (Input, Event)>
where
    F: Fn(Input, Event) -> (Output, Answer),
    Input: Any,
    Output: Any,
{
    fn from(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Input, Output, Answer, Event> Transition<Event> for FuncTransition<F, (Input, Event)>
where
    F: Fn(Input, Event) -> (Output, Answer),
    Input: Any,
    Output: Any,
{
    type Answer = Answer;

    fn transition(&self, from: &mut dyn Vertex, event: Event) -> Result<(Box<dyn Any>, Answer), TransitionError<Event>> {
        from.exit();
        // TODO: remove Any::is check because it must be done by caller.
        let input = from.get_data().downcast::<Input>()
            .unwrap_or_else(|_| panic!("It is caller task"));
        let out = (self.0)(*input, event);
        Ok((Box::new(out.0), out.1))
    }

    fn input_tid(&self) -> TypeId {
        TypeId::of::<Input>()
    }

    fn output_tid(&self) -> TypeId {
        TypeId::of::<Output>()
    }
}

pub struct StateMachine<Event, Answer> {
    state: usize,
    vertexes: Vec<Box<dyn Vertex>>,
    transitions: HashMap<TypeId, Vec<Box<dyn Transition<Event, Answer = Answer>>>>,
}

impl<Event, Answer> StateMachine<Event, Answer> {
    pub fn new() -> Self {
        let vertexes = vec![Box::new(State::new(InitialPseudostate)) as _];
        let transitions = HashMap::new();
        StateMachine { state: 0, vertexes, transitions }
    }
    pub fn register_vertex(mut self, vertex: Box<dyn Vertex>) -> Self {
        self.vertexes.push(vertex);
        self
    }
    pub fn transition<T: Transition<Event, Answer = Answer> + 'static>(mut self, transition: T) -> Self {
        assert!(self.find_vertex_by_data_tid(transition.input_tid()).is_some(), "Not found input vertex!");
        assert!(self.find_vertex_by_data_tid(transition.output_tid()).is_some(), "Not found output vertex!");

        let trans = Box::new(transition);
        self.transitions.entry(trans.input_tid())
            .or_default()
            .push(trans);
        self
    }
    pub fn process(&mut self, mut event: Event) -> Result<Answer, StateMachineError> {
        let state = self.vertexes[self.state].as_mut();
        let state_tid = state.data_tid();

        let transitions = self.transitions.get(&state_tid).ok_or(StateMachineError::NoTransition)?;
        for transition in transitions {
            match transition.transition(state, event) {
                Ok((new_state, answer)) => {
                    let new_vertex = self.find_vertex_by_data_tid(new_state.as_ref().type_id()).expect("It should be checked in the `transition` function");
                    self.vertexes[new_vertex].set_data(new_state);
                    self.vertexes[new_vertex].entry();
                    self.state = new_vertex;
                    return Ok(answer);
                }
                Err(e) => {
                    let TransitionError { event: event1, kind } = e;
                    match kind {
                        TransitionErrorKind::GuardErr => {
                            event = event1;
                            continue
                        },
                    }
                }
            }
        }

        Err(StateMachineError::NoTransition)
    }
    fn find_vertex_by_data_tid(&self, tid: TypeId) -> Option<usize> {
        self.vertexes.iter().enumerate().find(|(_, x)| x.data_tid() == tid).map(|(x, _)| x)
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
            .register_vertex(State::empty::<SomeState>().boxed())
            .transition(ftrans(|_: InitialPseudostate, event: i32| {
                (SomeState, event * 2)
            }));

        assert_eq!(machine.process(3), Ok(6));
        assert_eq!(machine.process(3), Err(StateMachineError::NoTransition));
    }

    #[test]
    fn test_guards() {
        let mut machine = StateMachine::new()
            .transition(
                GuardedTransition::new(
                    ftrans(|_: InitialPseudostate, event: i32| {
                        (InitialPseudostate, event * 2)
                    })
                )
                    .guard(|event: &i32| event % 2 == 0)
            ).transition(
                GuardedTransition::new(
                    ftrans(|_: InitialPseudostate, event: i32| {
                        (InitialPseudostate, event * 3)
                    })
                )
                    .guard(|event: &i32| event % 3 == 0)
            );

        assert_eq!(machine.process(2), Ok(2*2));
        assert_eq!(machine.process(3), Ok(3*3));
        assert_eq!(machine.process(6), Ok(6*2));
    }
}
