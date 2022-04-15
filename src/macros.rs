#[macro_export]
macro_rules! events {
    (
        $( #[ $($meta:tt)+ ] )*
        $v:vis enum $enum_name:ident {
            $($it:tt)*
        }
    ) => {
        events! {
            @inner
            $( #[ $($meta)+ ] )*
            $v enum $enum_name {
                $($it)*
            }
            []
        }
    };

    (
        @inner
        $( #[ $($meta:tt)+ ] )*
        $ve:vis enum $enum_name:ident {
            $( #[ $($meta2:tt)+ ] )*
            $vs:vis struct $it_name:ident;
            $($rest:tt)*
        }
        [$($names:ident),*]
    ) => {
        $( #[ $($meta)+ ] )*
        $( #[ $($meta2)+ ] )*
        $vs struct $it_name;

        events! {
            @inner
            $( #[ $($meta)+ ] )*
            $ve enum $enum_name {
                $($rest)*
            }
            [$($names,)* $it_name]
        }
    };

    (
        @inner
        $( #[ $($meta:tt)+ ] )*
        $ve:vis enum $enum_name:ident {
            $( #[ $($meta2:tt)+ ] )*
            $vs:vis struct $it_name:ident ( $($it_ty:ty),*);
            $($rest:tt)*
        }
        [$($names:ident),*]
    ) => {
        $( #[ $($meta)+ ] )*
        $( #[ $($meta2)+ ] )*
        $vs struct $it_name ($($it_ty),*);

        events! {
            @inner
            $( #[ $($meta)+ ] )*
            $ve enum $enum_name {
                $($rest)*
            }
            [$($names,)* $it_name]
        }
    };

    (
        @inner
        $( #[ $($meta:tt)+ ] )*
        $ve:vis enum $enum_name:ident {
            $( #[ $($meta2:tt)+ ] )*
            $vs:vis struct $it_name:ident { $($it_fname:ident : $it_fty:ty),* }
            $($rest:tt)*
        }
        [$($names:ident),*]
    ) => {
        $( #[ $($meta)+ ] )*
        $( #[ $($meta2)+ ] )*
        $vs struct $it_name { $($it_fname : $it_fty),* }

        events! {
            @inner
            $( #[ $($meta)+ ] )*
            $ve enum $enum_name {
                $($rest)*
            }
            [$($names,)* $it_name]
        }
    };

    (
        @inner
        $( #[ $($meta:tt)+ ] )*
        $ve:vis enum $enum_name:ident {}
        [$($name:ident),+]
    ) => {
        $ve enum $enum_name {
            $(
                $name($name),
            )+
        }
    };
}

#[macro_export]
macro_rules! states {
    (
        $v:vis trait $dyn_state:ident
            $(:
                $(
                    $trait1:ident $( :: $trait2:ident )*
                        $( < $($gen:ty),* > )?
                ),+
            )?
        ;
        $( #[ $($meta:tt)+ ] )*
        {
            $it:item
            $($rest:item)*
        }
    ) => {
        // Trait that is used as `dyn DynState` in the umlsm::Sm<...> generic.
        $v trait $dyn_state: std::any::Any + $( $($trait1 $( :: $trait2 )* $( < $($gen)* > )? +)+ )? {
            fn tid(&self) -> std::any::TypeId;
        }

        impl<T: 'static + $( $( $trait1 $( :: $trait2 )* $( < $($gen)* > )? +)+ )?> $dyn_state for T {
            // This method needed to recognize TypeId of a `T` type.
            fn tid(&self) -> std::any::TypeId {
                std::any::TypeId::of::<T>()
            }
        }

        impl<T: std::any::Any + $( $( $trait1 $( :: $trait2 )* $( < $($gen)* > )? +)+ )?> $crate::state::Cast<T> for dyn $dyn_state {
            fn upcast(from: Box<T>) -> Box<Self> {
                from
            }

            fn upcast_ref(from: &T) -> &Self {
                from
            }

            fn concrete_tid(&self) -> std::any::TypeId {
                self.tid()
            }
        }

        $crate::events! {
            $( #[ $($meta)+ ] )*
            {
                $it
                $($rest)*
            }
        }
    };
}

#[macro_export]
macro_rules! switch {
    (
        $trait1:ident $( :: $trait2:ident )* $( < $($gen:ty),* > )?
        + $event:ty = $output:expr
    ) => {
        $crate::transition::Switch::<
            $trait1 $( :: $trait2 )* $( < $($gen),* > )?,
            $event,
            _,
        >::new($output)
    };
}

#[cfg(test)]
mod compile_tests {
    use super::*;

    events! {
        #[derive(Debug)]
        #[repr(C)]
        enum Event {
            #[derive(Clone)]
            struct Event1(String);
            struct Event2 { field: u64 }
            struct Event3;
        }
    }
/*
    states! {
        pub trait DynState1;
        {
            struct State1;
        }
    }

    states! {
        pub trait DynState2: std::fmt::Debug;
        {
            #[derive(Debug)]
            struct State2;
        }
    }

    states! {
        pub trait DynState3: std::fmt::Debug, std::fmt::Display;
        {
            struct State3;
        }
    }
*/
}
