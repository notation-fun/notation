pub mod midi_control;

use bevy::prelude::*;
use notation_midi::prelude::{MidiSettings, MidiState};

use crate::prelude::{TabPlugin, NotationSettings, PlayControlEvent};
use crate::play::play_button::PlayButton;
use midi_control::MidiControl;

#[cfg(feature = "with_egui")]
pub mod egui_midi_control_panel;

impl TabPlugin {
    pub fn on_play_button_clicked(
        settings: &mut NotationSettings,
        _midi_settings: &MidiSettings,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
        button: &PlayButton,
    ) {
        match button.action {
            crate::play::play_button::PlayButtonAction::PlayPause => {
                MidiControl::play_or_pause(midi_state, play_control_evts)
            }
            crate::play::play_button::PlayButtonAction::Stop => {
                MidiControl::stop(midi_state, play_control_evts)
            }
            crate::play::play_button::PlayButtonAction::LoopMode => {
                settings.should_loop = !settings.should_loop;
                MidiControl::sync_should_loop(
                    settings,
                    midi_state,
                    play_control_evts,
                )
            }
            crate::play::play_button::PlayButtonAction::SetBegin => {
                MidiControl::set_begin_bar_ordinal(midi_state, play_control_evts)
            }
            crate::play::play_button::PlayButtonAction::SetEnd => {
                MidiControl::set_end_bar_ordinal(midi_state, play_control_evts)
            }
            crate::play::play_button::PlayButtonAction::Clear => {
                MidiControl::clear_begin_end(midi_state, play_control_evts)
            }
        }
    }
}