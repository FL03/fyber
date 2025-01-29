/*
    Appellation: fyber <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # fyber
//!
//! fyber is a harmonic orchstrator designed to efficiently facilitate communcations between
//! ephemeral and persistent computational spaces.
//!
//! - Every `fyber` is considered to be some simplicial topological abstraction defining the
//! smallest computable surface in any dimension.
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "fyber"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "agents")]
pub use fyber_agents as agents;
#[doc(inline)]
pub use fyber_core::*;

pub mod prelude {
    #[cfg(feature = "agents")]
    pub use fyber_agents::prelude::*;
    pub use fyber_core::prelude::*;
}
