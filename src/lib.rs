pub use choice::Choice;
pub use state_machine::{StateMachine, StateMachineError};
pub use vertex::Vertex;

mod choice;
pub mod guard;
pub mod state;
mod state_machine;
pub mod transition;
mod vertex;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guard::GuardedTransition;
    use crate::state::InitialPseudostate;
    use crate::state::State;
    use crate::transition::ftrans;

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
                    })),
            )
            .transition(
                GuardedTransition::new()
                    .guard(|event: &i32| event % 3 == 0)
                    .transition(ftrans(|_: InitialPseudostate, event: i32| {
                        (InitialPseudostate, event * 3)
                    })),
            );

        assert_eq!(machine.process(2), Ok(2 * 2));
        assert_eq!(machine.process(3), Ok(3 * 3));
        assert_eq!(machine.process(6), Ok(6 * 2));
    }

    #[test]
    fn test_choice() {
        let mut machine = StateMachine::new().transition(
            Choice::if_(|x: &i32| x.is_positive())
                .then(ftrans(|s: InitialPseudostate, x| (s, x * 2)))
                .else_(
                    Choice::if_(|x: &i32| *x > -10)
                        .then(ftrans(|s: InitialPseudostate, x| (s, x * 3)))
                        .else_(ftrans(|s: InitialPseudostate, x| (s, x * 4))),
                ),
        );

        assert_eq!(machine.process(2), Ok(2 * 2));
        assert_eq!(machine.process(-2), Ok(-2 * 3));
        assert_eq!(machine.process(-20), Ok(-20 * 4));
    }
}
