/*
    Appellation: raw_event <module>
    Created At: 2025.08.31:10:03:44
    Contrib: @FL03
*/

/// The [`RawEvent`] trait is a basis for all events within the framework.
pub trait RawEvent {}

/// The [`Event`] trait is used to define the standard interface for an event within the
/// framework.
pub trait Event: RawEvent {}
