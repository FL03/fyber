/*
    Appellation: fluo-events <library>
    Created At: 2025.08.31:10:23:55
    Contrib: @FL03
*/
//! the [`events`](self) module works to establish a solid foundation for an event-based
//! architecture within the framework, providing essential structures and traits to define,
//! manage, and interact with events in a consistent manner.
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

#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! {
    "Either the `alloc` or `std` feature must be enabled."
}

#[doc(inline)]
pub use self::{
    error::{Error, Result},
    event::*,
    traits::*,
};

pub mod error;
pub mod event;

mod traits {
    //! traits for defining events and their behaviors
    #[doc(inline)]
    pub use self::prelude::*;

    mod raw_event;

    mod prelude {
        #[doc(inline)]
        pub use super::raw_event::*;
    }
}

mod utils {}

#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::event::EventBase;
    pub use crate::traits::*;
}
