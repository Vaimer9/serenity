use std::collections::HashMap;

use crate::json::{from_number, Value};
use crate::model::channel::ChannelType;

#[derive(Debug, Clone, Default)]
pub struct CreateThread(pub HashMap<&'static str, Value>);

impl CreateThread {
    /// The name of the thread.
    ///
    /// **Note**: Must be between 2 and 100 characters long.
    pub fn name<D: ToString>(&mut self, name: D) -> &mut Self {
        self.0.insert("name", Value::from(name.to_string()));

        self
    }

    /// Duration in minutes to automatically archive the thread after recent activity.
    ///
    /// **Note**: Can only be set to 60, 1440, 4320, 10080 currently.
    pub fn auto_archive_duration(&mut self, duration: u16) -> &mut Self {
        self.0.insert("auto_archive_duration", from_number(duration));

        self
    }

    /// The thread type, which can be [`ChannelType::PublicThread`] or [`ChannelType::PrivateThread`].
    ///
    /// **Note**: This defaults to [`ChannelType::PrivateThread`] in order to match the behavior
    /// when thread documentation was first published. This is a bit of a weird default though,
    /// and thus is highly likely to change in the future, so it is recommended to always
    /// explicitly setting it to avoid any breaking change.
    pub fn kind(&mut self, kind: ChannelType) -> &mut Self {
        self.0.insert("type", from_number(kind as u8));

        self
    }
}
