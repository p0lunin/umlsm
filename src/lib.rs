pub use event::{EnterSmEvent, Event};
pub use sm::{Sm, SmBuilder, SmError};
pub use vertex::Vertex;

mod event;
pub mod guard;
mod macros;
mod sm;
pub mod state;
pub mod transition;
mod vertex;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guard::GuardedTransition;
    use crate::state::{InitialPseudoState, SimpleVertex};
    use crate::transition::ftrans;
    use std::any::Any;

    #[test]
    fn test1() {
        struct SomeState;
        struct SomeState2;

        let mut machine = SmBuilder::<dyn Any>::with_default_state(SomeState)
            .transition(ftrans(|_: SomeState, event: EnterSmEvent| SomeState))
            .register_vertex(SimpleVertex::<SomeState2>::new().to_vertex())
            .transition(ftrans(|_: SomeState, event: i32| SomeState2))
            .build()
            .unwrap();

        assert_eq!(machine.process(3), Ok(()));
        assert_eq!(
            machine.process(3),
            Err(SmError::NoTransitionsFromThisVertex(3))
        );
    }

    #[test]
    fn test_guards() {
        #[derive(Debug, PartialEq)]
        struct ChooseState;
        #[derive(Debug, PartialEq)]
        struct DivisibleBy2(u64);
        #[derive(Debug, PartialEq)]
        struct DivisibleBy3(u64);

        let make_machine = || {
            SmBuilder::<dyn Any>::new()
                .register_vertex(SimpleVertex::with_data(ChooseState).to_vertex())
                .register_vertex(SimpleVertex::<DivisibleBy2>::new().to_vertex())
                .register_vertex(SimpleVertex::<DivisibleBy3>::new().to_vertex())
                .transition(ftrans(|_: InitialPseudoState, _: EnterSmEvent| ChooseState))
                .transition(
                    GuardedTransition::new()
                        .guard(|event: &u64| event % 2 == 0)
                        .transition(ftrans(|_: ChooseState, number: u64| DivisibleBy2(number))),
                )
                .transition(
                    GuardedTransition::new()
                        .guard(|event: &u64| event % 3 == 0)
                        .transition(ftrans(|_: ChooseState, number: u64| DivisibleBy3(number))),
                )
                .build()
                .unwrap()
        };

        {
            let mut sm = make_machine();
            assert_eq!(sm.process(2_u64), Ok(()));
            assert_eq!(sm.current_state_concrete(), Some(&DivisibleBy2(2)));
        }
        {
            let mut sm = make_machine();
            assert_eq!(sm.process(3_u64), Ok(()));
            assert_eq!(sm.current_state_concrete(), Some(&DivisibleBy3(3)));
        }
        {
            let mut sm = make_machine();
            assert_eq!(sm.process(6_u64), Ok(()));
            assert_eq!(sm.current_state_concrete(), Some(&DivisibleBy2(6)));
        }
    }
}
