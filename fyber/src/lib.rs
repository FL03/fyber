/*
    Appellation: fyber <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # fyber
//!
//! fyber focuses on facilitating communications between various wasm instances
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "fyber"]

#[cfg(feature = "alloc")]
extern crate alloc;


pub mod prelude {}
