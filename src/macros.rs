#[macro_export]
macro_rules! events {
    (
        $( #[ $($meta:tt)+ ] )*
        {
            $it:item
            $($rest:item)*
        }
    ) => {
        $( #[ $($meta)+ ] )*
        $it

        events! {
            $( #[ $($meta)+ ] )*
            {
                $($rest)*
            }
        }
    };

    ( $( #[ $($meta:tt)+ ] )* {}) => {};
}

#[macro_export]
macro_rules! states {
    (
        $v:vis trait $dyn_state:ident
            $(:
                $(
                    $trait1:ident $( :: $trait2:ident )*
                        $( < $($gen:ty)* > + )?
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

#[cfg(test)]
mod compile_tests {
    use super::*;

    events! {
        #[derive(Debug, PartialEq)]
        #[repr(C)]
        {
            struct Foo;
            struct Bar;
            struct Baz;
        }
    }

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
}
