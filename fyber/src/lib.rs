/*
    Appellation: fyber <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # fyber
//!
//! fyber is a harmonic orchstrator designed to efficiently facilitate communcations between 
//! ephemeral and persistent computational spaces.
//! 
//! - Every `fyber` speaks to the smallest computable surface
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "fyber"]

#[cfg(feature = "alloc")]
extern crate alloc;


pub mod prelude {}
