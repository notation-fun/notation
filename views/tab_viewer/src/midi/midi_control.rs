use std::sync::Arc;

use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::prelude::GridData;
use notation_model::prelude::{BarPosition, Tab, Units, TabBar};
use notation_midi::prelude::{MidiSettings, MidiState, TickResult, JumpToBarEvent, PlayControlEvent};

use crate::tab::tab_bars::TabBars;
use crate::tab::tab_plugin::{TabPlugin};

use crate::prelude::{NotationSettings};

pub struct MidiControl();

impl MidiControl {
    pub fn sync_speed_factor(
        settings: &NotationSettings,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        midi_state
            .play_control
            .play_speed
            .set_factor(settings.speed_factor);
        play_control_evts.send(PlayControlEvent::on_speed_factor(
            midi_state.play_control.play_speed.factor(),
        ));
    }
    pub fn sync_should_loop(
        settings: &NotationSettings,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        midi_state.play_control.should_loop = settings.should_loop;
        play_control_evts.send(PlayControlEvent::on_should_loop(
            midi_state.play_control.should_loop,
        ));
    }
    pub fn send_play_state_evt(
        midi_state: &MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        play_control_evts.send(PlayControlEvent::on_play_state(
            midi_state.play_control.play_state,
        ));
        let tick_result = TickResult {
            changed: true,
            end_passed: false,
            stopped: midi_state.play_control.play_state.is_stopped(),
            jumped: false,
        };
        play_control_evts.send(PlayControlEvent::on_tick(
            midi_state.play_control.position,
            tick_result,
        ));
    }
    pub fn play(midi_state: &mut MidiState, play_control_evts: &mut EventWriter<PlayControlEvent>) {
        if !midi_state.play_control.play_state.is_playing() {
            if midi_state.play_control.play() {
                Self::send_play_state_evt(midi_state, play_control_evts);
            }
        }
    }
    pub fn pause(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        if midi_state.play_control.play_state.is_playing() {
            if midi_state.play_control.pause() {
                Self::send_play_state_evt(midi_state, play_control_evts);
            }
        }
    }
    pub fn play_or_pause(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        if midi_state.play_control.play_state.is_playing() {
            if midi_state.play_control.pause() {
                Self::send_play_state_evt(midi_state, play_control_evts);
            }
        } else {
            if midi_state.play_control.play() {
                Self::send_play_state_evt(midi_state, play_control_evts);
            }
        }
    }
    pub fn play_or_stop(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        if midi_state.play_control.play_state.is_playing() {
            Self::stop(midi_state, play_control_evts);
        } else {
            if midi_state.play_control.play() {
                Self::send_play_state_evt(midi_state, play_control_evts);
            }
        }
    }
    pub fn stop(midi_state: &mut MidiState, play_control_evts: &mut EventWriter<PlayControlEvent>) {
        if midi_state.play_control.stop() {
            midi_state.play_control.position.bar.bar_ordinal =
                midi_state.play_control.begin_bar_ordinal;
            midi_state.play_control.position.bar.in_bar_pos = Units(0.0);
            Self::send_play_state_evt(midi_state, play_control_evts);
        }
    }
    pub fn seek_forward(
        midi_settings: &MidiSettings,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        if midi_state.play_control.play_state.is_playing() {
            if midi_state.play_control.pause() {
                Self::send_play_state_evt(midi_state, play_control_evts);
            }
        } else if midi_state.seek_forward(midi_settings) {
        }
    }
    pub fn jump_to_bar<F: Fn(&Tab, BarPosition) -> Option<Arc<TabBar>>>(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
        calc_bar: &F,
    ) {
        if let Some(tab) = &midi_state.tab {
            let pos = midi_state.play_control.position.bar;
            if let Some(bar) = calc_bar(&tab, pos) {
                TabPlugin::jump_to_bar(jump_to_bar_evts, bar.props);
            }
        }
    }
    pub fn jump_to_center_bar(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            let center = tab.bars.len() / 2;
            tab.get_bar_of_ordinal(if center == pos.bar_ordinal {
                center + 1
            } else {
                center
            })
        });
    }
    pub fn jump_to_prev_bar(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            if pos.in_bar_pos.0 > 0.0 {
                tab.get_bar_of_ordinal(pos.bar_ordinal)
            } else if pos.bar_ordinal > 0 {
                tab.get_bar_of_ordinal(pos.bar_ordinal - 1)
            } else {
                None
            }
        });
    }
    pub fn jump_to_next_bar(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            if pos.bar_ordinal < tab.bars.len() - 1 {
                tab.get_bar_of_ordinal(pos.bar_ordinal + 1)
            } else {
                None
            }
        });
    }
    pub fn jump_to_section_start(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            if let Some(bar) = tab.get_bar_of_ordinal(pos.bar_ordinal) {
                return tab.get_bar_of_ordinal(bar.props.get_section_first_bar_ordinal());
            }
            None
        });
    }
    pub fn jump_to_section_end(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            if let Some(bar) = tab.get_bar_of_ordinal(pos.bar_ordinal) {
                return tab.get_bar_of_ordinal(
                    bar.props.get_section_first_bar_ordinal() + bar.section.bars.len() - 1,
                );
            }
            None
        });
    }
    pub fn jump_to_prev_section(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            if let Some(bar) = tab.get_bar_of_ordinal(pos.bar_ordinal) {
                let first_bar_ordinal = pos.bar_ordinal - bar.props.bar_index;
                if first_bar_ordinal > 0 {
                    if let Some(bar) = tab.get_bar_of_ordinal(first_bar_ordinal - 1) {
                        return tab.get_bar_of_ordinal(bar.props.get_section_first_bar_ordinal());
                    }
                }
            }
            None
        });
    }
    pub fn jump_to_next_section(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            if let Some(bar) = tab.get_bar_of_ordinal(pos.bar_ordinal) {
                return tab.get_bar_of_ordinal(
                    bar.props.get_section_first_bar_ordinal() + bar.section.bars.len(),
                );
            }
            None
        });
    }
    pub fn get_grid_cols(
        tab_bars_query: &Query<(&TabBars, &GridData), With<TabBars>>,
    ) -> usize {
        for (_tab_bars, grid_data) in tab_bars_query.iter() {
            return grid_data.cols;
        }
        2
    }
    pub fn jump_to_prev_row(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
        tab_bars_query: &Query<(&TabBars, &GridData), With<TabBars>>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            let cols = Self::get_grid_cols(tab_bars_query);
            if pos.bar_ordinal > cols {
                tab.get_bar_of_ordinal(pos.bar_ordinal - cols)
            } else {
                tab.get_bar_of_ordinal(0)
            }
        });
    }
    pub fn jump_to_next_row(
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
        tab_bars_query: &Query<(&TabBars, &GridData), With<TabBars>>,
    ) {
        Self::jump_to_bar(midi_state, jump_to_bar_evts, &|tab, pos| {
            let cols = Self::get_grid_cols(tab_bars_query);
            if pos.bar_ordinal < tab.bars.len() - cols {
                tab.get_bar_of_ordinal(pos.bar_ordinal + cols)
            } else {
                tab.get_bar_of_ordinal(tab.bars.len() - 1)
            }
        });
    }
    pub fn send_begin_end_evt(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        play_control_evts.send(PlayControlEvent::on_begin_end(
            midi_state.play_control.begin_bar_ordinal,
            midi_state.play_control.end_bar_ordinal,
        ));
    }
    pub fn clear_begin_end(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        midi_state.play_control.begin_bar_ordinal = 0;
        midi_state.play_control.end_bar_ordinal = midi_state.play_control.get_last_bar_ordinal();
        Self::send_begin_end_evt(midi_state, play_control_evts);
    }
    pub fn set_begin_bar_ordinal(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        let begin_bar_ordinal = midi_state.play_control.position.bar.bar_ordinal;
        midi_state.play_control.begin_bar_ordinal = begin_bar_ordinal;
        if midi_state.play_control.end_bar_ordinal < begin_bar_ordinal {
            midi_state.play_control.end_bar_ordinal = begin_bar_ordinal;
        }
        Self::send_begin_end_evt(midi_state, play_control_evts);
    }
    pub fn set_end_bar_ordinal(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        let end_bar_ordinal = midi_state.play_control.position.bar.bar_ordinal;
        midi_state.play_control.end_bar_ordinal = end_bar_ordinal;
        if midi_state.play_control.begin_bar_ordinal > end_bar_ordinal {
            midi_state.play_control.begin_bar_ordinal = end_bar_ordinal;
        }
        Self::send_begin_end_evt(midi_state, play_control_evts);
    }
    pub fn set_begin_end_to_section(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        if let Some(tab) = &midi_state.tab {
            if let Some(bar) =
                tab.get_bar_of_ordinal(midi_state.play_control.position.bar.bar_ordinal)
            {
                midi_state.play_control.begin_bar_ordinal =
                    bar.props.get_section_first_bar_ordinal();
                midi_state.play_control.end_bar_ordinal =
                    bar.props.get_section_first_bar_ordinal() + bar.section.bars.len() - 1;
                Self::send_begin_end_evt(midi_state, play_control_evts);
            }
        }
    }
    pub fn set_speed_factor(
        settings: &mut NotationSettings,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
        speed_factor: f32,
    ) {
        settings.speed_factor = speed_factor;
        Self::sync_speed_factor(settings, midi_state, play_control_evts)
    }
}
