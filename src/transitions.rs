#[macro_export]
macro_rules! transitions {
    // ------------OUTER------------
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

    // ------------INNER------------

    // ------------IMPL TRANSITION EMPTY STATE------------
    (
        @inner
        event : $event:ident;
        match $state:ident {
            $state_v:ident => match event {
                $($t:tt)*
            }
            $($tokens1:tt)*
        }
        [$($idents:ident),*]
    ) => {
        impl $crate::Transition<$event> for $state_v {
            type State = $state;
            #[allow(unreachable_patterns)]
            fn transition(self, event: $event) -> Result<$state, $crate::TransitionError<$state, $event>> {
                $crate::transitions! {
                    @event_matching
                    $event,
                    self,
                    match event {
                        $($t)*
                    }
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

    // ------------IMPL TRANSITION TUPLE STATE------------
    (
        @inner
        event : $event:ident;
        match $state:ident {
            $state_v:ident( $($spt:tt)* ) => match event {
                $($t:tt)*
            }
            $($tokens1:tt)*
        }
        [$($idents:ident),*]
    ) => {
        impl $crate::Transition<$event> for $state_v {
            type State = $state;
            #[allow(unreachable_patterns)]
            fn transition(self, event: $event) -> Result<$state, $crate::TransitionError<$state, $event>> {
                let $state_v( $($spt)* ) = self;
                $crate::transitions! {
                    @event_matching
                    $event,
                    $state::$state_v($state_v( $($spt)* )),
                    match event {
                        $($t)*
                    }
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

    // ------------IMPL TRANSITION ENUM------------
    (
        @inner
        event : $event:ident;
        match $state:ident {}
        [$($id:ident),*]
    ) => {
        impl $state {
            pub fn transition(self, event: impl Into<$event>) -> Result<$state, $crate::TransitionError<$state, $event>> {
                let event = event.into();
                $crate::Transition::transition(self, event)
            }
        }
        impl $crate::Transition<$event> for $state {
            type State = $state;
            #[allow(unreachable_patterns)]
            fn transition(self, event: $event) -> Result<$state, $crate::TransitionError<$state, $event>> {
                match self {
                    $(
                        $state::$id(state) => state.transition(event),
                    )*
                    _ => Err($crate::TransitionError::NoTransition(self, event))
                }
            }
        }
    };

    // ------------IMPL MATCHING EMPTY EVENT------------
    (
        @event_matching
        $event:ident,
        $this:expr,
        match $event_id:ident {
            $pe:ident => $out:expr;
            $($t:tt)*
        }
    ) => {
        match $event_id {
            $event::$pe(_) => Ok($out.into()),
            _ => $crate::transitions! {
                @event_matching
                $event,
                $this,
                match $event_id {
                    $($t)*
                }
            }
        }
    };

    // ------------IMPL MATCHING TUPLE EVENT------------
    (
        @event_matching
        $event:ident,
        $this:expr,
        match $event_id:ident {
            $pe:ident( $($fpats:tt)* ) => $out:expr;
            $($t:tt)*
        }
    ) => {
        match $event_id {
            $event::$pe( $pe( $($fpats)* ) ) => Ok($out.into()),
            _ => $crate::transitions! {
                @event_matching
                $event,
                $this,
                match $event_id {
                    $($t)*
                }
            }
        }
    };

    // ------------IMPL MATCHING NO TRANSITION------------
    (
        @event_matching
        $event:ident,
        $this:expr,
        match $event_id:ident {}
    ) => {
        Err($crate::TransitionError::NoTransition($this.into(), $event_id))
    };
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
    mod test1 {
        use crate::{stenum, TransitionError};

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
            let state1: State = State1.into();
            let state2 = state1.transition(Event1).unwrap();
            assert_eq!(state2, State2.into());
            let state3 = state2.transition(Event2).unwrap();
            assert_eq!(state3, State3.into());
            let err = state3.transition(Event1);
            assert_eq!(err, Err(TransitionError::NoTransition(State3.into(), Event1.into())));
        }
    }

    mod test2 {
        use crate::{stenum, TransitionError};

        stenum! {
            #[derive(Debug, PartialEq)]
            enum Event {
                struct Event1(String);
                struct Event2;
            }
        }

        stenum! {
            #[derive(Debug, PartialEq)]
            enum State {
                struct State1;
                struct State2(String);
                struct State3(String);
            }
        }

        transitions! {
            event : Event;

            match State {
                State1 => match event {
                    Event1(s) => State2(s);
                    Event2 => State3(String::new());
                }
                State2(s) => match event {
                    Event2 => State3(s);
                }
                State3 => match event {}
            }
        }

        #[test]
        fn test_transitions_2() {
            let state1: State = State1.into();

            let state2 = state1.transition(Event1("test".into())).unwrap();
            assert_eq!(state2, State2("test".into()).into());

            let state3 = state2.transition(Event2).unwrap();
            assert_eq!(state3, State3("test".into()).into());

            let err = state3.transition(Event2);
            assert_eq!(err, Err(TransitionError::NoTransition(State3("test".into()).into(), Event2.into())));
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