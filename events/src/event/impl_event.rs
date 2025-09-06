/*
    Appellation: impl_event <module>
    Created At: 2025.08.31:10:11:54
    Contrib: @FL03
*/
use super::EventBase;
use alloc::string::String;

impl<T> EventBase<T> {
    /// constructs a new `EventBase` instance.
    ///
    /// # Arguments
    ///
    /// * `data` - The data associated with the event.
    /// * `timestamp` - The timestamp of the event.
    /// * `source` - An optional source identifier for the event.
    ///
    /// # Returns
    ///
    /// A new instance of `EventBase`.
    pub const fn new(data: T) -> Self {
        Self {
            data,
            message: None,
            source: None,
            timestamp: 0,
        }
    }
    /// returns an immutable reference to the event data
    pub const fn data(&self) -> &T {
        &self.data
    }
    /// returns a mutable reference to the event data
    pub const fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
    /// returns the timestamp of the event
    pub const fn timestamp(&self) -> u64 {
        self.timestamp
    }
    /// returns a reference to the source of the event, if available
    pub fn source(&self) -> Option<&str> {
        self.source.as_ref().map(String::as_str)
    }
}
