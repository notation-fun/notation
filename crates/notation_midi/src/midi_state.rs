
use std::convert::TryInto;
use std::sync::Arc;
use midi_msg::GMSoundSet;

use helgoboss_midi::{Channel, U7};
use notation_model::prelude::*;

use crate::prelude::MidiSettings;

pub const DEFAULT_PROGRAM: u8 = 0;
pub const DEFAULT_VELOCITY: u8 = 64;

#[derive(Debug)]
pub struct MidiChannel {
    pub track: Option<(String, TrackKind)>,
    pub channel: Channel,
    pub program: U7,
    pub velocity: U7,
}
impl MidiChannel {
    pub fn new(channel: u8) -> Self {
        Self {
            track: None,
            channel: Channel::new(channel),
            program: U7::new(DEFAULT_PROGRAM),
            velocity: U7::new(DEFAULT_VELOCITY),
        }
    }
    pub fn set_params(&mut self, track_id: String, track_kind: TrackKind, params: (u8, u8)) {
        self.track = Some((track_id, track_kind));
        self.program = U7::new(params.0);
        self.velocity = U7::new(params.1);
    }
}

pub struct MidiState {
    pub channels: [MidiChannel; 16],
}

impl Default for MidiState {
    fn default() -> Self {
        Self {
            channels: Self::new_channels(),
        }
    }
}

impl MidiState {
    fn new_channels() -> [MidiChannel; 16] {
        let channels: Vec<MidiChannel> = (0u8..16u8)
            .map(MidiChannel::new)
            .collect::<Vec<MidiChannel>>();
        channels.try_into().unwrap()
    }
    fn reset_channels(&mut self) {
        for channel in self.channels.iter_mut() {
            channel.track = None;
            channel.program = U7::new(DEFAULT_PROGRAM);
            channel.program = U7::new(DEFAULT_VELOCITY);
        }
    }
}
impl MidiState {
    pub fn get_channel(&self, track_id: &String, track_kind: &TrackKind) -> Option<&MidiChannel> {
        for channel in self.channels.iter() {
            if let Some(track) = &channel.track {
                if track.0 == *track_id && track.1 == *track_kind {
                    return Some(channel);
                }
            }
        }
        None
    }
    pub fn switch_tab(&mut self, settings: &MidiSettings, tab: Arc<Tab>) {
        self.reset_channels();
        let mut index: usize = 0;
        for track in tab.tracks.iter() {
            if index >= self.channels.len() {
                return;
            }
            if let Some(params) = settings.get_track_channel_params(&track.kind) {
                if let Some(channel) = self.channels.get_mut(index) {
                    channel.set_params(track.id.clone(), track.kind.clone(), params);
                    index += 1;
                }
            }
        }
    }
}
