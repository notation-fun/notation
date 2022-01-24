use std::cmp::Ordering;
use std::convert::TryInto;
use std::sync::Arc;

use helgoboss_midi::{controller_numbers, Channel, StructuredShortMessage, U7};
use notation_model::play::play_control::TickResult;
use notation_model::prelude::*;

use crate::midi_hub::MidiHub;
use crate::prelude::{MidiMessage, MidiSettings, MidiUtil};

pub const DEFAULT_PROGRAM: u8 = 0;
pub const DEFAULT_VELOCITY: u8 = 64;

#[derive(Debug)]
pub struct MidiChannel {
    pub track: Option<Arc<Track>>,
    pub channel: Channel,
    pub program: U7,
    pub velocity: U7,
    pub messages: Vec<MidiMessage>,
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
        self.velocity = U7::new(DEFAULT_VELOCITY);
        self.messages.clear();
        self.need_sort = false;
        self.next_index = 0;
    }
    pub fn add_message(&mut self, msg: MidiMessage) {
        self.messages.push(msg);
        self.need_sort = true;
    }
    fn ensure_sorted(&mut self) -> bool {
        if self.need_sort {
            dmsort::sort_by(&mut self.messages, |a, b| {
                let units_a = a.effect_units().0;
                let units_b = b.effect_units().0;
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
    pub fn calc_next_index(&mut self, position: &BarPosition) {
        let pos_units = Units::from(*position);
        for (index, value) in self.messages.iter().enumerate() {
            if Units::from(value.effect_position()) >= pos_units {
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
        speed: &PlaySpeed,
        is_seeking: bool,
        old_position: &Position,
        play_control: &PlayControl,
        end_passed: bool,
        jumped: bool,
    ) -> usize {
        if self.messages.len() == 0 {
            return 0;
        }
        if end_passed || jumped {
            self.init_channel(settings, hub, speed);
            if end_passed {
                self.calc_next_index(&play_control.begin_bar_position());
            } else {
                self.calc_next_index(&play_control.position.bar);
            }
        } else if self.ensure_sorted() {
            self.calc_next_index(&old_position.bar);
        }
        let bypass = if is_seeking {
            self.track
                .as_ref()
                .map(|x| x.kind != settings.seeking_track)
                .unwrap_or(true)
        } else {
            false
        };
        let mut velocity = self.velocity.into();
        if !bypass {
            match &self.track {
                Some(track) => {
                    let seeking = is_seeking && track.kind == settings.seeking_track;
                    match track.kind {
                        TrackKind::Vocal => {
                            velocity = if !seeking && settings.vocal_mute {
                                0
                            } else {
                                settings.vocal_velocity
                            };
                        }
                        TrackKind::Guitar => {
                            velocity = if !seeking && settings.guitar_mute {
                                0
                            } else {
                                settings.guitar_velocity
                            };
                        }
                        TrackKind::Piano => {
                            velocity = if !seeking && settings.piano_mute {
                                0
                            } else {
                                settings.piano_velocity
                            };
                        }
                        _ => (),
                    }
                }
                None => {
                    velocity = if settings.click_mute {
                        0
                    } else {
                        settings.click_velocity
                    };
                }
            }
        }
        let mut count = 0;
        loop {
            if let Some(next) = self.messages.get(self.next_index) {
                if play_control.is_bar_in_range(next.bar_ordinal())
                    && play_control
                        .position
                        .is_passed(next.pass_mode, &next.effect_position())
                {
                    self.next_index += 1;
                    count += 1;
                    if !bypass {
                        hub.send(settings, speed, next, velocity);
                    }
                } else {
                    if next.effect_position().bar_ordinal < play_control.begin_bar_ordinal {
                        self.next_index += 1;
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        count
    }
    fn init_channel(&mut self, settings: &MidiSettings, hub: &mut MidiHub, speed: &PlaySpeed) {
        if let Some(first_msg) = self.messages.get(0) {
            let msg = StructuredShortMessage::ProgramChange {
                channel: self.channel,
                program_number: self.program,
            };
            hub.send(
                settings,
                speed,
                &MidiMessage::new(
                    first_msg.pass_mode,
                    first_msg.pos,
                    first_msg.duration,
                    false,
                    msg,
                ),
                self.velocity.into(),
            );
            let msg = StructuredShortMessage::ControlChange {
                channel: self.channel,
                controller_number: controller_numbers::ALL_SOUND_OFF,
                control_value: U7::new(0),
            };
            hub.send(
                settings,
                speed,
                &MidiMessage::new(
                    first_msg.pass_mode,
                    first_msg.pos,
                    first_msg.duration,
                    false,
                    msg,
                ),
                self.velocity.into(),
            );
        }
    }
    pub fn setup(
        &mut self,
        _settings: &MidiSettings,
        _hub: &mut MidiHub,
        params: (u8, u8),
        track: &Arc<Track>,
    ) {
        self.track = Some(track.clone());
        self.program = U7::new(params.0);
        self.velocity = U7::new(params.1);
    }
    pub fn setup_no_track(
        &mut self,
        _settings: &MidiSettings,
        _hub: &mut MidiHub,
        params: (u8, u8),
    ) {
        self.track = None;
        self.program = U7::new(params.0);
        self.velocity = U7::new(params.1);
    }
}

pub struct MidiState {
    pub tab: Option<Arc<Tab>>,
    pub channels: [MidiChannel; 16],
    pub play_control: PlayControl,
    pub seek_position: Option<BarPosition>,
}

impl Default for MidiState {
    fn default() -> Self {
        Self {
            tab: None,
            channels: Self::new_channels(),
            play_control: PlayControl::default(),
            seek_position: None,
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
                if track.id == *track_id && track.kind == *track_kind {
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
                if track.id == *track_id && track.kind == *track_kind {
                    return Some(channel);
                }
            }
        }
        None
    }
    fn create_click_channel(
        &mut self,
        settings: &MidiSettings,
        hub: &mut MidiHub,
        tab: &Tab,
        index: &mut usize,
    ) {
        let params = settings.get_click_channel_params();
        if let Some(channel) = self.channels.get_mut(*index) {
            channel.setup_no_track(settings, hub, params);
            println!(
                "switch_tab(), setup click channel: [{}] -> {}, {}",
                index, params.0, params.1
            );
            *index += 1;
            let scale_root = tab.meta.scale.calc_root_syllable();
            let signature = tab.signature();
            let bar_units = tab.bar_units();
            let beat_duration = Units::from(signature.beat_unit);
            for bar in tab.bars.iter() {
                for beat in 0..signature.bar_beats {
                    let in_bar_pos = Units(beat as f32 * Units::from(signature.beat_unit).0);
                    let root = bar
                        .get_chord(Some(in_bar_pos))
                        .map(|x| x.root)
                        .unwrap_or(scale_root);
                    let note = tab.meta.scale.calc_click_note(
                        &tab.meta.key,
                        &settings.click_octave,
                        &root,
                    );
                    let pos = BarPosition::new(bar_units, bar.props.bar_ordinal, in_bar_pos);
                    if let Some(midi_msg) =
                        MidiUtil::note_midi_on_msg(&note, channel.channel, channel.velocity)
                    {
                        channel.add_message(MidiMessage::new(
                            EntryPassMode::Delayed,
                            pos,
                            beat_duration,
                            false,
                            midi_msg,
                        ));
                    }
                    if let Some(midi_msg) =
                        MidiUtil::note_midi_off_msg(&note, channel.channel, channel.velocity)
                    {
                        channel.add_message(MidiMessage::new(
                            EntryPassMode::Delayed,
                            pos,
                            beat_duration,
                            true,
                            midi_msg,
                        ));
                    }
                }
            }
        }
    }
    pub fn switch_tab(&mut self, settings: &MidiSettings, hub: &mut MidiHub, tab: Arc<Tab>) {
        self.tab = Some(tab.clone());
        self.reset_channels();
        let mut index: usize = 0;
        self.create_click_channel(settings, hub, &tab, &mut index);
        for track in tab.tracks.iter() {
            if index >= self.channels.len() {
                return;
            }
            if let Some(params) = settings.get_track_channel_params(&track.kind) {
                if let Some(channel) = self.channels.get_mut(index) {
                    channel.setup(settings, hub, params, track);
                    println!(
                        "switch_tab(), setup channel: [{}] -> {}, {} - {}",
                        index, params.0, params.1, track
                    );
                    index += 1;
                }
            }
        }
        for bar in tab.bars.iter() {
            for ((_k, _i), lane) in bar.lanes.iter() {
                if let Some(channel) = self.get_channel_mut(&lane.track.id, &lane.track.kind) {
                    for entry in lane.entries.iter() {
                        if let Some(msgs) = MidiUtil::get_midi_msgs(channel, bar, &entry) {
                            for msg in msgs {
                                channel.add_message(MidiMessage::of_entry(entry, msg.0, msg.1));
                            }
                        }
                    }
                }
            }
        }
        self.play_control = PlayControl::new(&tab);
        self.init_channels(settings, hub);
    }
    pub fn jump_to_bar(
        &mut self,
        settings: &MidiSettings,
        hub: &mut MidiHub,
        bar_props: TabBarProps,
    ) {
        self.play_control
            .position
            .set_in_bar(bar_props.bar_ordinal, Units(0.0));
        if self.play_control.is_bar_in_range(bar_props.bar_ordinal) {
            for channel in self.channels.iter_mut() {
                channel.calc_next_index(&self.play_control.position.bar);
            }
        } else if self.play_control.play_state.is_playing() {
            self.play_control.pause();
            self.init_channels(settings, hub);
        }
    }
    pub fn tick(
        &mut self,
        settings: &MidiSettings,
        hub: &mut MidiHub,
        jumped: bool,
        delta_seconds: f32,
    ) -> TickResult {
        let is_seeking = self.seek_position.is_some();
        let old_position = self.play_control.position;
        if self.seek_position.is_some() {
            let pos = self.play_control.position.bar;
            if Units::from(pos) >= Units::from(self.seek_position.unwrap()) {
                if !self.seek_passed(settings) {
                    self.seek_position = None;
                }
            }
        }
        let tick_result = match self.seek_position {
            Some(pos) => self.play_control._tick_to_position(jumped, pos.into()),
            None => self.play_control.tick(jumped, delta_seconds),
        };
        if tick_result.changed {
            for channel in self.channels.iter_mut() {
                if is_seeking && settings.seeking_init_channel {
                    channel.init_channel(settings, hub, &self.play_control.play_speed);
                }
                channel.send_passed_msgs(
                    settings,
                    hub,
                    &self.play_control.play_speed,
                    is_seeking,
                    &old_position,
                    &self.play_control,
                    tick_result.end_passed,
                    tick_result.jumped,
                );
            }
        }
        self.seek_position = None;
        if is_seeking {
            self.play_control.pause();
        }
        tick_result
    }
    pub fn init_channels(&mut self, settings: &MidiSettings, hub: &mut MidiHub) {
        for channel in self.channels.iter_mut() {
            if channel.messages.len() > 0 {
                channel.init_channel(settings, hub, &self.play_control.play_speed);
                channel.calc_next_index(&self.play_control.position.bar);
            }
        }
    }
    fn setup_seek(&mut self, seek_position: BarPosition) {
        println!(
            "MidiState::setup_seek() {} -> {}",
            self.play_control.position.bar, seek_position
        );
        self.seek_position = Some(seek_position);
        self.play_control.play();
    }
    pub fn seek_forward(&mut self, settings: &MidiSettings) -> bool {
        if self.tab.is_some() {
            let pos = self.play_control.position.bar;
            if let Some(bar) = self.tab.as_ref().unwrap().get_bar(pos) {
                if let Some(props) = bar.get_next_entry(pos.in_bar_pos, &|x| {
                    if x.track_kind() != settings.seeking_track {
                        None
                    } else if x.prev_is_tie() {
                        None
                    } else {
                        Some(x.props.clone())
                    }
                }) {
                    self.setup_seek(
                        pos.with_in_bar_pos(props.in_bar_pos - Units::HALF_MIN_ACCURACY),
                    );
                    return true;
                } else {
                    self.setup_seek(BarPosition::new(
                        pos.bar_units,
                        pos.bar_ordinal,
                        pos.bar_units - Units::HALF_MIN_ACCURACY,
                    ));
                    return true;
                }
            }
        }
        false
    }
    /*
     * The reason for this logic is to show the guitar view's string animation properly, can't seek to
     *  next bar directly, that will make the guitar view changed all in a sudden, so did a seek to
     *  almost end of the bar, then the next time pass the first note on next bar.
     * A bit hacky, but works fine.
     */
    fn seek_passed(&mut self, settings: &MidiSettings) -> bool {
        if self.tab.is_some() {
            let mut pos = self.play_control.position.bar;
            if pos.in_bar_pos >= pos.bar_units - Units::MIN_ACCURACY {
                pos = BarPosition::new(pos.bar_units, pos.bar_ordinal + 1, Units::MIN_ACCURACY);
                if let Some(bar) = self.tab.as_ref().unwrap().get_bar(pos) {
                    if let Some(props) = bar.get_next_entry(pos.in_bar_pos, &|x| {
                        if x.track_kind() != settings.seeking_track {
                            None
                        } else if x.prev_is_tie() {
                            None
                        } else {
                            Some(x.props.clone())
                        }
                    }) {
                        self.setup_seek(pos.with_in_bar_pos(props.in_bar_pos));
                        return true;
                    }
                }
            }
        }
        false
    }
}
