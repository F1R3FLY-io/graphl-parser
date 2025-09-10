use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Channel {
    pub name: String,
}

impl Channel {
    /// Creates a new channel with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the channel
    ///
    /// # Returns
    ///
    /// A new `Channel` instance.
    pub fn new<T>(name: T) -> Self
    where
        T: Display,
    {
        Channel {
            name: name.to_string(),
        }
    }
}
