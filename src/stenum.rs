#[macro_export]
macro_rules! stenum {
    (
        $( #[ $($meta:tt)+ ] )*
        $v:vis enum $enum_name:ident {
            $($it:tt)*
        }
    ) => {
        stenum! {
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

        stenum! {
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

        stenum! {
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

        stenum! {
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
        $( #[ $($meta)+ ] )*
        $ve enum $enum_name {
            $(
                $name($name),
            )+
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
