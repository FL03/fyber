/*
    Appellation: fyber-actors <library>
    Contrib: @FL03
*/
//! Core modules for the `fyber` library
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use self::actor::*;

pub mod actor;
pub mod orch;

#[allow(unused_imports)]
pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod components;
    
    pub(crate) mod prelude {
        pub use super::components::*;
    }
}

pub mod prelude {
    pub use crate::orch::prelude::*;
}
