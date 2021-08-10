use helgoboss_midi::{StructuredShortMessage};
use notation_proto::prelude::*;

use crate::prelude::MidiUtil;

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

impl PlayToneEvent {
    pub fn new(track_id: String, track_kind: TrackKind, tone: Tone) -> Self {
        Self {
            track_id,
            track_kind,
            tone,
        }
    }
    pub fn to_midi_msgs(&self) -> Vec<StructuredShortMessage> {
        self.tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_on_msg(x))
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
    pub fn to_midi_msgs(&self) -> Vec<StructuredShortMessage> {
        self.tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_off_msg(x))
            .collect()
    }
}
