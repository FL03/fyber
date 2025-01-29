/*
    Appellation: fyber-core <library>
    Contrib: @FL03
*/
//! Core modules for the `fyber` library
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use self::types::prelude::*;

pub mod error;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod devices;

    pub(crate) mod prelude {
        pub use super::devices::*;
    }
}

pub mod prelude {
    pub use crate::types::prelude::*;
}
