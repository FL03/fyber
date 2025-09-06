/*
    Appellation: event <module>
    Created At: 2025.08.31:10:05:58
    Contrib: @FL03
*/

mod impl_event;

use alloc::string::String;

/// The [`EventBase`] struct is a generic implementation of an event within the framework.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct EventBase<T> {
    pub(crate) data: T,
    pub(crate) message: Option<String>,
    pub(crate) source: Option<String>,
    pub(crate) timestamp: u64,
}
