/*
    Appellation: actors <module>
    Contrib: @FL03
*/
//! The `actors` module focuses on defining concrete abstractions for actors in the `fyber` 
//! library.
//! 
//! Assuming every actor to be a node allows the generalization of input / output procedures
//! and provides the ability to define a common interface for all actors.
//! 
//! 


pub struct ActorBase {
    pub id: String,
    pub name: String,
    pub position: String,
}