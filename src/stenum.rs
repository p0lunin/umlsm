#[macro_export]
macro_rules! stenum {
    (
        $( #[ $($meta:tt)+ ] )*
        $v:vis enum $enum_name:ident {
            $($it:tt)*
        }
    ) => {
        $crate::stenum! {
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

        $crate::stenum! {
            @inner
            $( #[ $($meta)+ ] )*
            $ve enum $enum_name {
                $($rest)*
            }
            [$($names,)* $it_name]
        }
        $crate::stenum! { @impl_into $enum_name $it_name }
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

        $crate::stenum! {
            @inner
            $( #[ $($meta)+ ] )*
            $ve enum $enum_name {
                $($rest)*
            }
            [$($names,)* $it_name]
        }
        $crate::stenum! { @impl_into $enum_name $it_name }
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

        $crate::stenum! {
            @inner
            $( #[ $($meta)+ ] )*
            $ve enum $enum_name {
                $($rest)*
            }
            [$($names,)* $it_name]
        }
        $crate::stenum! { @impl_into $enum_name $it_name }
    };

    (
        @inner
        $( #[ $($meta:tt)+ ] )*
        $ve:vis enum $enum_name:ident {}
        [$($name:ident),+]
    ) => {
        $( #[ $($meta)+ ] )*
        $ve enum $enum_name {
            $(
                $name($name),
            )+
        }
    };

    (
        @impl_into
        $enum_name:ident
        $struct_name:ident
    ) => {
        impl Into<$enum_name> for $struct_name {
            fn into(self) -> $enum_name {
                $enum_name::$struct_name(self)
            }
        }
    };
}

#[cfg(test)]
mod compile_tests {
    stenum! {
        #[derive(Debug)]
        #[repr(C)]
        #[allow(unused)]
        enum Event {
            #[derive(Clone)]
            pub struct Event1(String);
            struct Event2 { field: u64 }
            pub(crate) struct Event3;
        }
    }
}
