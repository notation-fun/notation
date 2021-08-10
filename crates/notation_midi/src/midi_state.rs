
use std::convert::TryInto;
use std::sync::Arc;

use helgoboss_midi::{Channel, U7};
use notation_proto::prelude::*;

#[derive(Debug)]
pub struct MidiChannel {
    pub channel: Channel,
    pub track: Option<(String, TrackKind)>,
    pub volume: U7,
}
impl MidiChannel {
    pub fn new(channel: u8) -> Self {
        Self {
            channel: Channel::new(channel),
            track: None,
            volume: U7::new(127),
        }
    }
}

pub struct MidiState {
    pub channels: Arc<[MidiChannel; 16]>,
}

impl Default for MidiState {
    fn default() -> Self {
        Self {
            channels: Self::new_channels(),
        }
    }
}

impl MidiState {
    fn new_channels() -> Arc<[MidiChannel; 16]> {
        let channels: Vec<MidiChannel> = (0u8..16u8)
            .map(MidiChannel::new)
            .collect::<Vec<MidiChannel>>();
        Arc::new(channels.try_into().unwrap())
    }
}
