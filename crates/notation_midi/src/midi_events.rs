use std::sync::Arc;

use helgoboss_midi::StructuredShortMessage;
use notation_model::play::play_control::TickResult;
use notation_model::prelude::*;

use crate::prelude::{MidiChannel, MidiUtil};

#[derive(Debug)]
pub struct SwitchTabEvent {
    pub tab: Arc<Tab>,
}
impl SwitchTabEvent {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}

#[derive(Debug)]
pub struct AddToneEvent {
    pub track_id: String,
    pub track_kind: TrackKind,
    pub tone: Tone,
    pub position: BarPosition,
    pub units: Units,
}

#[derive(Debug)]
pub enum PlayControlEvt {
    OnTick {
        position: Position,
        tick_result: TickResult,
    },
    OnPlayState(PlayState),
    OnPlaySpeed(f32),
}
impl PlayControlEvt {
    pub fn on_tick(position: Position, tick_result: TickResult) -> Self {
        Self::OnTick {
            position,
            tick_result,
        }
    }
    pub fn on_play_state(play_state: PlayState) -> Self {
        Self::OnPlayState(play_state)
    }
    pub fn on_play_speed(play_speed: f32) -> Self {
        Self::OnPlaySpeed(play_speed)
    }
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

impl AddToneEvent {
    pub fn new(
        track_id: String,
        track_kind: TrackKind,
        tone: Tone,
        position: BarPosition,
        units: Units,
    ) -> Self {
        Self {
            track_id,
            track_kind,
            tone,
            position,
            units,
        }
    }
    pub fn to_midi_msgs(
        &self,
        channel: &MidiChannel,
    ) -> Vec<(BarPosition, StructuredShortMessage)> {
        let mut play_msgs: Vec<(BarPosition, StructuredShortMessage)> = self
            .tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_on_msg(x, channel.channel, channel.velocity))
            .map(|x| (self.position, x))
            .collect();
        let stop_position = BarPosition::new(
            self.position.bar_units,
            self.position.bar_ordinal,
            self.position.in_bar_pos + self.units,
        );
        let mut stop_msgs: Vec<(BarPosition, StructuredShortMessage)> = self
            .tone
            .get_notes()
            .iter()
            .flat_map(|x| MidiUtil::note_midi_off_msg(x, channel.channel, channel.velocity))
            .map(|x| (stop_position, x))
            .collect();
        play_msgs.append(&mut stop_msgs);
        play_msgs
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
