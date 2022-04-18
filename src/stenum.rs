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
            impl_for: [$enum_name,]
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
        impl_for: [$($impl_for:ident,)+]
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
            impl_for: [$($impl_for,)+]
        }
        $crate::stenum! { @impl_into [$($impl_for,)+] $it_name }
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
        impl_for: [$($impl_for:ident,)+]
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
            impl_for: [$($impl_for,)+]
        }
        $crate::stenum! { @impl_into [$($impl_for,)+] $it_name }
    };

    // ------------IMPL REGION------------
    (
        @inner
        $( #[ $($meta:tt)+ ] )*
        $ve:vis enum $enum_name:ident {
            $( #[ $($meta2:tt)+ ] )*
            $vs:vis region $it_name:ident { $($rt:tt)* }
            $($rest:tt)*
        }
        [$($names:ident),*]
        impl_for: [$($impl_for:ident,)+]
    ) => {
        $crate::stenum! {
            @inner
            $( #[ $($meta2)+ ] )*
            $vs enum $it_name {
                $($rt)*
            }
            []
            impl_for: [$it_name, $($impl_for,)+]
        }
        $crate::stenum! {
            @inner
            $( #[ $($meta)+ ] )*
            $ve enum $enum_name {
                $($rest)*
            }
            [$($names,)* $it_name]
            impl_for: [$($impl_for,)+]
        }
        $crate::stenum! { @impl_into [$($impl_for,)+] $it_name }
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
        impl_for: [$($impl_for:ident,)+]
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
            impl_for: [$($impl_for,)+]
        }
        $crate::stenum! { @impl_into [$($impl_for,)+] $it_name }
    };

    (
        @inner
        $( #[ $($meta:tt)+ ] )*
        $ve:vis enum $enum_name:ident {}
        [$($name:ident),+]
        impl_for: [$($impl_for:ident,)+]
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
        [$enum_name:ident, $($idents:ident,)*]
        $struct_name:ident
    ) => {
        impl Into<$enum_name> for $struct_name {
            fn into(self) -> $enum_name {
                $enum_name::$struct_name(self)
            }
        }
        $crate::stenum! {
            @impl_into_expr
            [$($idents,)*]
            $enum_name
            $struct_name
            (|x| $enum_name::$struct_name(x))
        }
    };
    (
        @impl_into_expr
        [$enum_name:ident, $($idents:ident,)*]
        $prev_name:ident
        $struct_name:ident
        $into:expr
    ) => {
        impl Into<$enum_name> for $struct_name {
            fn into(self) -> $enum_name {
                $enum_name::$prev_name(($into)(self))
            }
        }
        $crate::stenum! {
            @impl_into_expr
            [$($idents,)*]
            $enum_name
            $struct_name
            |x| $enum_name::$struct_name(($into)(x))
        }
    };
    (
        @impl_into_expr
        []
        $prev_name:ident
        $struct_name:ident
        $into:expr
    ) => {};
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
}
