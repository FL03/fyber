/*
    Appellation: fyber <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # fyber
//!
//! The `fyber` protocol aims to facilitate communications between individual WebAssembly components, modules, and their hosts.
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::self_named_constructors,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use fyber_core::*;
#[cfg(feature = "events")]
#[doc(inline)]
pub use fyber_events as events;

#[doc(hidden)]
pub mod prelude {
    #[allow(unused_imports)]
    pub use fyber_core::prelude::*;
    #[cfg(feature = "events")]
    pub use fyber_events::prelude::*;
}
