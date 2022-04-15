#[macro_export]
macro_rules! transitions {
    (
        event : $event:ident;
        match $state:ident {
            $($tokens1:tt)*
        }
    ) => {
        $crate::transitions! {
            @inner
            event : $event;
            match $state {
                $($tokens1)*
            }
            []
        }
    };

    (
        @inner
        event : $event:ident;
        match $state:ident {
            $state_v:ident => match event {
                $($pe:ident => $out:ident;)*
            }
            $($tokens1:tt)*
        }
        [$($idents:ident),*]
    ) => {
        impl $state_v {
            #[allow(unreachable_patterns)]
            pub fn transition(self, event: $event) -> Result<$state, $crate::TransitionError<$state, $event>> {
                match event {
                    $(
                        $event::$pe(_) => Ok($state::$out($out)),
                    )*
                    _ => Err(TransitionError::NoTransition($state::$state_v(self), event))
                }
            }
        }

        $crate::transitions! {
            @inner
            event : $event;
            match $state {
                $($tokens1)*
            }
            [$($idents,)* $state_v]
        }
    };

    (
        @inner
        event : $event:ident;
        match $state:ident {}
        [$($id:ident),*]
    ) => {
        impl $state {
            #[allow(unreachable_patterns)]
            pub fn transition(self, event: $event) -> Result<$state, $crate::TransitionError<$state, $event>> {
                match self {
                    $(
                        $state::$id(state) => state.transition(event),
                    )*
                    _ => Err(TransitionError::NoTransition(self, event))
                }
            }
        }
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum TransitionError<State, Event> {
    NoTransition(State, Event),
}

/*
transitions! {
    match TopLevelState {
        State1 => match Event {
            Event1(str) -> State2(str);
            Event2 { field: uint } -> State3 { s: String::new(), u: uint };
        }
        State2(str) => match Event {
            Event2 { field: uint } -> State3 { s: str, u: uint };
        }
        State3 { s, u } => match Event {}
        State4 {
            Var1 => {}
            Var2(str) => {}
            Var3 { field } => {}
        }
    }
}
 */

#[cfg(test)]
mod compile_tests {
    use crate::stenum;
    use super::*;

    mod test1 {
        use super::*;

        stenum! {
            #[derive(Debug, PartialEq)]
            enum Event {
                struct Event1;
                struct Event2;
            }
        }

        stenum! {
            #[derive(Debug, PartialEq)]
            enum State {
                struct State1;
                struct State2;
                struct State3;
            }
        }

        transitions! {
            event : Event;

            match State {
                State1 => match event {
                    Event1 => State2;
                    Event2 => State3;
                }
                State2 => match event {
                    Event2 => State3;
                }
                State3 => match event {}
            }
        }

        #[test]
        fn test_transitions_1() {
            let state1 = State::State1(State1);
            let state2 = state1.transition(Event::Event1(Event1)).unwrap();
            assert_eq!(state2, State::State2(State2));
            let state3 = state2.transition(Event::Event2(Event2)).unwrap();
            assert_eq!(state3, State::State3(State3));
        }
    }

/*
    transitions! {
        @inner
        event : Event;

        match State {
            State1 => match event {
                Event::Event1(str) => State2(str);
                Event::Event2 { u } => State3 { s: String::new(), u };
            }
            State2(str) => match event {
                Event::Event2 { field: uint } => State3 { s: str, u: uint };
            }
            State3 { s, u } => match event {}
        }

        []
    }*/
}