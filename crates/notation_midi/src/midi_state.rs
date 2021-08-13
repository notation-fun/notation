use std::cmp::Ordering;
use std::convert::TryInto;
use std::sync::Arc;

use helgoboss_midi::{controller_numbers, Channel, StructuredShortMessage, U7};
use notation_model::play::play_control::TickResult;
use notation_model::prelude::*;

use crate::midi_hub::MidiHub;
use crate::prelude::{AddToneEvent, MidiSettings, MidiUtil};

pub const DEFAULT_PROGRAM: u8 = 0;
pub const DEFAULT_VELOCITY: u8 = 64;

#[derive(Debug)]
pub struct MidiChannel {
    pub track: Option<(String, TrackKind)>,
    pub channel: Channel,
    pub program: U7,
    pub velocity: U7,
    pub messages: Vec<(BarPosition, StructuredShortMessage)>,
    need_sort: bool,
    next_index: usize,
}
impl MidiChannel {
    pub fn new(channel: u8) -> Self {
        Self {
            track: None,
            channel: Channel::new(channel),
            program: U7::new(DEFAULT_PROGRAM),
            velocity: U7::new(DEFAULT_VELOCITY),
            messages: Vec::new(),
            need_sort: false,
            next_index: 0,
        }
    }
    pub fn reset(&mut self) {
        self.track = None;
        self.program = U7::new(DEFAULT_PROGRAM);
        self.program = U7::new(DEFAULT_VELOCITY);
        self.messages.clear();
        self.need_sort = false;
        self.next_index = 0;
    }
    pub fn add_message(&mut self, msg: (BarPosition, StructuredShortMessage)) {
        self.messages.push(msg);
        self.need_sort = true;
    }
    fn ensure_sorted(&mut self) -> bool {
        if self.need_sort {
            dmsort::sort_by(&mut self.messages, |a, b| {
                let units_a = Units::from(a.0).0;
                let units_b = Units::from(b.0).0;
                if units_a == units_b {
                    Ordering::Equal
                } else if units_a < units_b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            self.need_sort = false;
            true
        } else {
            false
        }
    }
    fn calc_next_index(&mut self, position: &BarPosition) {
        for (index, value) in self.messages.iter().enumerate() {
            if Units::from(value.0) >= Units::from(*position) {
                self.next_index = index;
                return;
            }
        }
        self.next_index = self.messages.len();
    }
    pub fn send_passed_msgs(
        &mut self,
        settings: &MidiSettings,
        hub: &mut MidiHub,
        old_position: &Position,
        play_control: &PlayControl,
        end_passed: bool,
    ) -> usize {
        if self.messages.len() == 0 {
            return 0;
        }
        if end_passed {
            self.init_channel(settings, hub);
            self.calc_next_index(&play_control.begin_bar_position());
        } else if self.ensure_sorted() {
            self.calc_next_index(&old_position.bar);
        }
        let mut count = 0;
        loop {
            if let Some(&next) = self.messages.get(self.next_index) {
                if play_control.is_in_range(&next.0) && play_control.position.is_passed(&next.0) {
                    self.next_index += 1;
                    count += 1;
                    hub.send(settings, next.1);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        count
    }
    fn init_channel(&mut self, settings: &MidiSettings, hub: &mut MidiHub) {
        let msg = StructuredShortMessage::ProgramChange {
            channel: self.channel,
            program_number: self.program,
        };
        hub.send(settings, msg);
        let msg = StructuredShortMessage::ControlChange {
            channel: self.channel,
            controller_number: controller_numbers::ALL_SOUND_OFF,
            control_value: U7::new(0),
        };
        hub.send(settings, msg);
    }
    pub fn setup(
        &mut self,
        settings: &MidiSettings,
        hub: &mut MidiHub,
        params: (u8, u8),
        track: &Track,
    ) {
        self.track = Some((track.id.clone(), track.kind.clone()));
        self.program = U7::new(params.0);
        self.velocity = U7::new(params.1);
        self.init_channel(settings, hub);
    }
}

pub struct MidiState {
    pub channels: [MidiChannel; 16],
    pub play_control: PlayControl,
}

impl Default for MidiState {
    fn default() -> Self {
        Self {
            channels: Self::new_channels(),
            play_control: PlayControl::default(),
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
            channel.reset();
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
    pub fn get_channel_mut(
        &mut self,
        track_id: &String,
        track_kind: &TrackKind,
    ) -> Option<&mut MidiChannel> {
        for channel in self.channels.iter_mut() {
            if let Some(track) = &channel.track {
                if track.0 == *track_id && track.1 == *track_kind {
                    return Some(channel);
                }
            }
        }
        None
    }
    pub fn switch_tab(&mut self, settings: &MidiSettings, hub: &mut MidiHub, tab: Arc<Tab>) {
        self.reset_channels();
        let mut index: usize = 0;
        for track in tab.tracks.iter() {
            if index >= self.channels.len() {
                return;
            }
            if let Some(params) = settings.get_track_channel_params(&track.kind) {
                if let Some(channel) = self.channels.get_mut(index) {
                    channel.setup(settings, hub, params, track);
                    index += 1;
                }
            }
        }
        for bar in tab.bars.iter() {
            for lane in bar.lanes.iter() {
                if let Some(channel) = self.get_channel_mut(&lane.track.id, &lane.track.kind) {
                    for entry in lane.entries.iter() {
                        if let Some(msgs) = MidiUtil::get_midi_msgs(channel, bar, &entry) {
                            for msg in msgs {
                                channel.add_message(msg);
                            }
                        }
                    }
                }
            }
        }
        self.play_control = PlayControl::new(&tab);
    }
    pub fn add_tone(&mut self, evt: &AddToneEvent) {
        if let Some(channel) = self.get_channel_mut(&evt.track_id, &evt.track_kind) {
            for msg in evt.to_midi_msgs(channel) {
                channel.add_message(msg);
            }
        }
    }
    pub fn tick(
        &mut self,
        settings: &MidiSettings,
        hub: &mut MidiHub,
        delta_seconds: f32,
    ) -> TickResult {
        let old_position = self.play_control.position;
        let tick_result = self.play_control.tick(delta_seconds);
        if tick_result.changed {
            for channel in self.channels.iter_mut() {
                channel.send_passed_msgs(
                    settings,
                    hub,
                    &old_position,
                    &self.play_control,
                    tick_result.end_passed,
                );
            }
        }
        tick_result
    }
    pub fn init_channels(&mut self, settings: &MidiSettings, hub: &mut MidiHub) {
        for channel in self.channels.iter_mut() {
            if channel.track.is_some() {
                channel.init_channel(settings, hub);
                channel.calc_next_index(&self.play_control.position.bar);
            }
        }
    }
}
