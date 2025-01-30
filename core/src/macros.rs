/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]

#[allow(unused_macros)]
macro_rules! impl_fmt {
    ($name:ty: $($t:ident($($rest:tt)*)),*) => {
        $(impl_fmt!($name: $t($($rest)*));)*
    };
    (@impl $name:ty: $t:ident($($rest:tt)*)) => {
        impl core::fmt::$t for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $($rest)*)
            }
        }
    };
}


macro_rules! builder {
    ($(#[derive($($d:ident),+)])?$name:ident::<$inner:ty> {$($k:ident: $v:ty),* $(,)?}) => {
        builder!(@loop builder: $name, derive: [$($($d),+)?], inner: $inner {$($k: $v),*});
    };
    ($(#[derive($($d:ident),+)])? $name:ident($inner:ty) {$($k:ident: $v:ty),* $(,)?}) => {
        builder!(@loop builder: $name, derive: [$($($d),+)?], inner: $inner {$($k: $v),*});
    };
    (@loop builder: $name:ident, derive: [$($d:ident),* $(,)?], inner: $inner:ty {$($k:ident: $v:ty),* $(,)?}) => {

        #[derive(Default, $($d),*)]
        pub struct $name {
            inner: $inner,
        }

        builder!(@impl builder: $name, inner: $inner {$($k: $v),*});
    };
    (@impl builder: $name:ident, inner: $inner:ty {$($k:ident: $v:ty),* $(,)?}) => {
        impl $name {
            pub fn new() -> Self {
                Self {
                    inner: Default::default()
                }
            }

            pub fn from_inner(inner: $inner) -> Self {
                Self { inner }
            }

            pub fn build(self) -> $inner {
                self.inner
            }

            $(
                pub fn $k(mut self, $k: $v) -> Self {
                    self.inner.$k = $k;
                    self
                }
            )*
        }
    };
}

macro_rules! getter {
    ($($call:ident$(.$field:ident)?<$out:ty>),* $(,)?) => {
        $($crate::getter!(@impl $call$(.$field)?<$out>);)*
    };
    ($via:ident::<[$($call:ident$(.$field:ident)?<$out:ty>),* $(,)?]>) => {
        $($crate::getter!(@impl $via::$call$(.$field)?<$out>);)*
    };
    ($($call:ident$(.$field:ident)?),* $(,)? => $out:ty) => {
        $($crate::getter!(@impl $call$(.$field)?<$out>);)*
    };
    ($via:ident::<[$($call:ident$(.$field:ident)?),* $(,)?]> => $out:ty) => {
        $crate::getter!($via::<[$($call$(.$field)?<$out>),*]>);
    };

    (@impl $call:ident<$out:ty>) => {
        $crate::getter!(@impl $call.$call<$out>);
    };
    (@impl $via:ident::$call:ident<$out:ty>) => {
        $crate::getter!(@impl $via::$call.$call<$out>);
    };
    (@impl $call:ident.$field:ident<$out:ty>) => {
        pub const fn $call(&self) -> &$out {
            &self.$field
        }
        paste::paste! {
            pub fn [< $call _mut>](&mut self) -> &mut $out {
                &mut self.$field
            }
        }
    };
    (@impl $via:ident::$call:ident.$field:ident<$out:ty>) => {
        /// Returns an immutable reference to
        pub const fn $call(&self) -> &$out {
            &self.$via.$field
        }

        paste::paste! {
            pub fn [< $call _mut>](&mut self) -> &mut $out {
                &mut self.$via.$field
            }
        }
    };
}

macro_rules! impl_getters {
    (@impl $call:ident<$out:ty>) => {
        $crate::getter!(@impl $call.$call<$out>);
    };
    (@impl $via:ident::$call:ident<$out:ty>) => {
        $crate::getter!(@impl $via::$call.$call<$out>);
    };
    (@impl $call:ident.$field:ident<$out:ty>) => {
        pub const fn $call(&self) -> &$out {
            &self.$field
        }
        paste::paste! {
            pub fn [< $call _mut>](&mut self) -> &mut $out {
                &mut self.$field
            }
        }
    };
    (@impl $via:ident::$call:ident.$field:ident<$out:ty>) => {
        /// Returns an immutable reference to
        pub const fn $call(&self) -> &$out {
            &self.$via.$field
        }

        paste::paste! {
            pub fn [< $call _mut>](&mut self) -> &mut $out {
                &mut self.$via.$field
            }
        }
    };
}
