pub use sm::{Sm, SmBuilder, SmError};
pub use vertex::Vertex;
pub use event::{EnterSmEvent, Event};

mod event;
pub mod guard;
mod sm;
pub mod state;
pub mod transition;
mod vertex;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guard::GuardedTransition;
    use crate::state::InitialPseudostate;
    use crate::state::SimpleVertex;
    use crate::transition::ftrans;

    #[test]
    fn test1() {
        struct SomeState;
        struct SomeState2;

        let mut machine =
            StateMachine::with_default_state(SimpleVertex::with_data(SomeState).boxed())
                .register_vertex(SimpleVertex::<SomeState2>::new().boxed())
                .transition(ftrans(|_: SomeState, event: i32| (SomeState2, event * 2)));

        assert_eq!(machine.process(3), Ok(6));
        assert_eq!(
            machine.process(3),
            Err(SmError::NoTransitionsFromThisVertex(3))
        );
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
}
