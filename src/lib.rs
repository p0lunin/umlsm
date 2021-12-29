pub use state_machine::{StateMachine, StateMachineError};
pub use vertex::Vertex;

mod vertex;
pub mod state;
pub mod guard;
pub mod transition;
mod state_machine;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use crate::state::InitialPseudostate;
    use crate::transition::ftrans;
    use crate::guard::GuardedTransition;

    #[test]
    fn test1() {
        struct SomeState;

        let mut machine = StateMachine::new()
            .register_vertex(State::empty::<SomeState>().boxed())
            .transition(ftrans(|_: InitialPseudostate, event: i32| {
                (SomeState, event * 2)
            }));

        assert_eq!(machine.process(3), Ok(6));
        assert_eq!(machine.process(3), Err(StateMachineError::NoTransitions));
    }

    #[test]
    fn test_guards() {
        let mut machine = StateMachine::new()
            .transition(
                GuardedTransition::new()
                    .guard(|event: &i32| event % 2 == 0)
                    .transition(ftrans(|_: InitialPseudostate, event: i32| {
                        (InitialPseudostate, event * 2)
                    }))
            ).transition(
                GuardedTransition::new()
                    .guard(|event: &i32| event % 3 == 0)
                    .transition(ftrans(|_: InitialPseudostate, event: i32| {
                        (InitialPseudostate, event * 3)
                    }))
            );

        assert_eq!(machine.process(2), Ok(2*2));
        assert_eq!(machine.process(3), Ok(3*3));
        assert_eq!(machine.process(6), Ok(6*2));
    }
}
