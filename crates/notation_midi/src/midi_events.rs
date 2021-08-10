use std::sync::Arc;

use helgoboss_midi::{Channel, StructuredShortMessage, U7};
use notation_model::prelude::*;

use crate::prelude::{MidiChannel, MidiUtil};

#[derive(Debug)]
pub struct SwitchTabEvent {
    pub tab: Arc<Tab>,
}

#[derive(Debug)]
pub struct PlayToneEvent {
    pub track_id: String,
    pub track_kind: TrackKind,
    pub tone: Tone,
}

#[derive(Debug)]
pub struct StopToneEvent {
    pub track_id: String,
    pub track_kind: TrackKind,
    pub tone: Tone,
}

impl SwitchTabEvent {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}

impl PlayToneEvent {
    pub fn new(track_id: String, track_kind: TrackKind, tone: Tone) -> Self {
        Self {
            track_id,
            track_kind,
            tone,
        }
    }
    pub fn to_midi_msgs(&self, channel: &MidiChannel) -> Vec<StructuredShortMessage> {
        self.tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_on_msg(x, channel.channel, channel.velocity))
            .collect()
    }
}

impl StopToneEvent {
    pub fn new(track_id: String, track_kind: TrackKind, tone: Tone) -> Self {
        Self {
            track_id,
            track_kind,
            tone,
        }
    }
    pub fn to_midi_msgs(&self, channel: &MidiChannel) -> Vec<StructuredShortMessage> {
        self.tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_off_msg(x, channel.channel, channel.velocity))
            .collect()
    }
}
