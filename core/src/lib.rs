/*
    Appellation: fluo-core <library>
    Created At: 2025.08.31:09:36:34
    Contrib: @FL03
*/
//! the core of the fluo protocol
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::self_named_constructors,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::error::{Error, Result};

pub mod error;

#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
}

#[doc(hidden)]
pub mod prelude {}
