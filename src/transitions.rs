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
            top_state : $state;
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
        top_state : $top_state:ident;
        match $state:ident {
            $state_v:ident => match event {
                $($t:tt)*
            }
            $($tokens1:tt)*
        }
        [$($idents:ident),*]
    ) => {
        impl $crate::Transition<$event> for $state_v {
            type State = $top_state;
            #[allow(unreachable_patterns)]
            fn transition(self, event: $event) -> Result<$top_state, $crate::TransitionError<$top_state, $event>> {
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
            top_state : $top_state;
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
        top_state : $top_state:ident;
        match $state:ident {
            $state_v:ident( $($spt:tt)* ) => match event {
                $($t:tt)*
            }
            $($tokens1:tt)*
        }
        [$($idents:ident),*]
    ) => {
        impl $crate::Transition<$event> for $state_v {
            type State = $top_state;
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
            top_state : $top_state;
            match $state {
                $($tokens1)*
            }
            [$($idents,)* $state_v]
        }
    };

    // ------------IMPL TRANSITION STRUCT STATE------------
    (
        @inner
        event : $event:ident;
        top_state : $top_state:ident;
        match $state:ident {
            $state_v:ident { $($spt:tt)* } => match event {
                $($t:tt)*
            }
            $($tokens1:tt)*
        }
        [$($idents:ident),*]
    ) => {
        impl $crate::Transition<$event> for $state_v {
            type State = $top_state;
            #[allow(unreachable_patterns)]
            fn transition(self, event: $event) -> Result<$state, $crate::TransitionError<$state, $event>> {
                let $state_v { $($spt)* } = self;
                $crate::transitions! {
                    @event_matching
                    $event,
                    $state::$state_v($state_v { $($spt)* }),
                    match event {
                        $($t)*
                    }
                }
            }
        }

        $crate::transitions! {
            @inner
            event : $event;
            top_state : $top_state;
            match $state {
                $($tokens1)*
            }
            [$($idents,)* $state_v]
        }
    };

    // ------------IMPL TRANSITION REGION STATE------------
    (
        @inner
        event : $event:ident;
        top_state : $top_state:ident;
        match $state:ident {
            region $state_v:ident => {
                $($t:tt)*
            }
            $($tokens1:tt)*
        }
        [$($idents:ident),*]
    ) => {
        $crate::transitions! {
            @inner
            event : $event;
            top_state : $top_state;
            match $state_v {
                $($t)*
            }
            []
        }

        $crate::transitions! {
            @inner
            event : $event;
            top_state : $top_state;
            match $state {
                $($tokens1)*
            }
            [$($idents,)* $state_v]
        }
    };

    // ------------IMPL TRANSITION FOR ALL STATES------------
    (
        @inner
        event : $event:ident;
        top_state : $top_state:ident;
        match $state:ident {
            _ => match event {
                $($t:tt)*
            }
        }
        [$($id:ident),*]
    ) => {
        impl $state {
            pub fn transition(self, event: impl Into<$event>) -> Result<$top_state, $crate::TransitionError<$top_state, $event>> {
                let event = event.into();
                $crate::Transition::transition(self, event)
            }
        }
        impl $crate::Transition<$event> for $state {
            type State = $top_state;
            #[allow(unreachable_patterns)]
            fn transition(self, event: $event) -> Result<$top_state, $crate::TransitionError<$top_state, $event>> {
                match self {
                    $(
                        $state::$id(state) => state.transition(event),
                    )*
                    _ => $crate::transitions! {
                        @event_matching
                        $event,
                        self,
                        match event {
                            $($t)*
                        }
                    }
                }
            }
        }
    };

    // ------------IMPL TRANSITION ENUM------------
    (
        @inner
        event : $event:ident;
        top_state : $top_state:ident;
        match $state:ident {}
        [$($id:ident),*]
    ) => {
        impl $state {
            pub fn transition(self, event: impl Into<$event>) -> Result<$top_state, $crate::TransitionError<$top_state, $event>> {
                let event = event.into();
                $crate::Transition::transition(self, event)
            }
        }
        impl $crate::Transition<$event> for $state {
            type State = $top_state;
            #[allow(unreachable_patterns)]
            fn transition(self, event: $event) -> Result<$top_state, $crate::TransitionError<$top_state, $event>> {
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

    // ------------IMPL MATCHING STRUCT EVENT------------
    (
        @event_matching
        $event:ident,
        $this:expr,
        match $event_id:ident {
            $pe:ident { $($fpats:tt)* } => $out:expr;
            $($t:tt)*
        }
    ) => {
        match $event_id {
            $event::$pe( $pe { $($fpats)* } ) => Ok($out.into()),
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

    mod test3 {
        use crate::{stenum, TransitionError};

        stenum! {
            #[derive(Debug, PartialEq)]
            enum Event {
                struct Event1 { str: String }
                struct Event2;
            }
        }

        stenum! {
            #[derive(Debug, PartialEq)]
            enum State {
                struct State1;
                struct State2 { str: String }
                struct State3 { str: String }
            }
        }

        transitions! {
            event : Event;

            match State {
                State1 => match event {
                    Event1 { str } => State2 { str };
                    Event2 => State3 { str: String::new() };
                }
                State2 { str } => match event {
                    Event2 => State3 { str };
                }
                State3 => match event {}
            }
        }

        #[test]
        fn test_transitions_3() {
            let state1: State = State1.into();

            let state2 = state1.transition(Event1 { str: "test".into() }).unwrap();
            assert_eq!(state2, State2 { str: "test".into() }.into());

            let state3 = state2.transition(Event2).unwrap();
            assert_eq!(state3, State3 { str: "test".into() }.into());

            let err = state3.transition(Event2);
            assert_eq!(err, Err(TransitionError::NoTransition(State3 { str: "test".into() }.into(), Event2.into())));
        }
    }

    mod test4 {
        use crate::{stenum};

        stenum! {
            #[derive(Debug, PartialEq, Clone)]
            enum Event {
                struct Event1 { num: u64 }
            }
        }

        stenum! {
            #[derive(Debug, PartialEq, Clone)]
            enum State {
                struct State1;
                struct State2 { num: u64 }
            }
        }

        transitions! {
            event : Event;

            match State {
                State1 => match event {
                    Event1 { num: 0 } => State2 { num: 1 };
                    Event1 { num: 1 } => State2 { num: 2 };
                    Event1 { num: _ } => State1;
                }
                State2 { num } => match event {}
            }
        }

        #[test]
        fn test_transitions_4() {
            let state1: State = State1.into();

            assert_eq!(
                state1.clone().transition(Event1 { num: 0 }).unwrap(),
                State2 { num: 1 }.into()
            );
            assert_eq!(
                state1.clone().transition(Event1 { num: 1 }).unwrap(),
                State2 { num: 2 }.into()
            );
            assert_eq!(
                state1.clone().transition(Event1 { num: 2 }).unwrap(),
                State1.into()
            );
        }
    }

    mod test5_composite_sm {
        use crate::{stenum};

        stenum! {
            #[derive(Debug, PartialEq, Clone)]
            enum Event {
                struct Event1;
                struct Event2;
                struct Event3;
            }
        }

        stenum! {
            #[derive(Debug, PartialEq, Clone)]
            enum State {
                struct State1;

                #[derive(Debug, PartialEq, Clone)]
                region InnerState {
                    struct InnerState1;
                    struct InnerState2;
                }
            }
        }

        transitions! {
            event : Event;

            match State {
                State1 => match event {
                    Event1 => InnerState1;
                }
                region InnerState => {
                    InnerState1 => match event {
                        Event2 => InnerState2;
                    }
                    _ => match event {
                        Event3 => State1;
                    }
                }
            }
        }

        #[test]
        fn test_transitions_5() {
            let state1: State = State1.into();

            let state2 = state1.transition(Event1).unwrap();
            assert_eq!(state2, InnerState1.into());

            let state3 = state2.transition(Event2).unwrap();
            assert_eq!(state3, InnerState2.into());

            let state4 = state3.transition(Event3).unwrap();
            assert_eq!(state4, State1.into());
        }
    }
}