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

pub use self::{agent::*, components::ComponentModel, traits::prelude::*};

pub mod agent;
pub mod components;
pub mod orch;

#[allow(unused_imports)]
pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod actors;

    pub(crate) mod prelude {
        pub use super::actors::*;
    }
}

pub mod prelude {
    
    pub use crate::orch::prelude::*;
    pub use crate::traits::prelude::*;
}
