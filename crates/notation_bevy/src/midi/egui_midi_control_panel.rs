use bevy::prelude::*;
use notation_midi::prelude::{MidiSettings, MidiState};
use crate::bevy_egui::egui::{CollapsingHeader, Slider, Ui};
use float_eq::float_ne;

use crate::prelude::{NotationSettings, PlayControlEvent, Control, EguiControlPanel, NotationState, NotationTheme, Octave};
use super::midi_control::MidiControl;

impl EguiControlPanel {
    pub fn play_control_ui(
        ui: &mut Ui,
        state: &mut NotationState,
        theme: &mut NotationTheme,
        settings: &mut NotationSettings,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        CollapsingHeader::new("Play Control")
            .default_open(true)
            .show(ui, |ui| {
                let add_ready_section = settings.add_ready_section;
                ui.checkbox(&mut settings.add_ready_section, "Add Empty Bar");
                if add_ready_section != settings.add_ready_section {
                    state.bars_range = None;
                    Control::reload_tab(state, theme);
                }
                let play_title = if midi_state.play_control.play_state.is_playing() {
                    "Pause"
                } else {
                    "Play"
                };
                ui.horizontal(|ui| {
                    if ui.button(play_title).clicked() {
                        MidiControl::play_or_pause(midi_state, play_control_evts);
                    }
                    if ui.button("Stop").clicked() {
                        if midi_state.play_control.stop() {
                            MidiControl::send_play_state_evt(midi_state, play_control_evts);
                        }
                    }
                    let should_loop = settings.should_loop;
                    ui.checkbox(&mut settings.should_loop, "Loop");
                    if should_loop != settings.should_loop {
                        MidiControl::sync_should_loop(settings, midi_state, play_control_evts)
                    }
                });
                let begin_bar_number = state.calc_bar_number(settings.add_ready_section, midi_state.play_control.begin_bar_ordinal);
                let end_bar_number = state.calc_bar_number(settings.add_ready_section, midi_state.play_control.end_bar_ordinal);
                ui.horizontal(|ui| {
                    if ui
                        .button(format!(
                            "Begin: {}",
                            begin_bar_number
                        ))
                        .clicked()
                    {
                        MidiControl::set_begin_bar_ordinal(midi_state, play_control_evts);
                    }
                    if ui
                        .button(format!("End: {}", end_bar_number))
                        .clicked()
                    {
                        MidiControl::set_end_bar_ordinal(midi_state, play_control_evts);
                    }
                    if ui.button("Clear").clicked() {
                        MidiControl::clear_begin_end(midi_state, play_control_evts);
                    }
                });
                if let Some((begin, end)) = state.bars_range {
                    let (begin, end) = if settings.add_ready_section {
                        (begin, end)
                    } else {
                        (begin + 1, end + 1)
                    };
                    if ui.button(format!("Clear Visible Bars: {} - {}", begin, end)).clicked() {
                        state.bars_range = None;
                        Control::reload_tab(state, theme);
                    }
                } else if midi_state.play_control.has_selection(settings.add_ready_section) {
                    let bars_range = (midi_state.play_control.begin_bar_ordinal, midi_state.play_control.end_bar_ordinal);
                    if ui.button(format!("Set Visible Bars: {} - {}", begin_bar_number, end_bar_number)).clicked() {
                        state.bars_range = Some(bars_range);
                        Control::reload_tab(state, theme);
                    }
                }
                ui.separator();
                let mut speed_factor = settings.speed_factor;
                ui.add(Slider::new(&mut speed_factor, 0.1..=2.0).text("Speed"));
                ui.horizontal(|ui| {
                    if ui.button("1/4").clicked() {
                        speed_factor = 0.25;
                    }
                    if ui.button("2/4").clicked() {
                        speed_factor = 0.5;
                    }
                    if ui.button("3/4").clicked() {
                        speed_factor = 0.75;
                    }
                    if ui.button("4/4").clicked() {
                        speed_factor = 1.0;
                    }
                });
                if float_ne!(speed_factor, settings.speed_factor, abs <= 0.01) {
                    MidiControl::set_speed_factor(settings, midi_state, play_control_evts, speed_factor)
                }
            });
    }
    pub fn midi_settings_ui(
        ui: &mut Ui,
        state: &mut NotationState,
        theme: &mut NotationTheme,
        midi_settings: &mut MidiSettings,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        CollapsingHeader::new("Midi & Audio")
            .default_open(true)
            .show(ui, |ui| {
                let mut bypass_hub = midi_settings.bypass_hub;
                ui.checkbox(&mut bypass_hub, "Bypass Midi Hub");
                if midi_settings.bypass_hub != bypass_hub {
                    if midi_state.play_control.play_state.is_playing() {
                        MidiControl::pause(midi_state, play_control_evts);
                    } else {
                        midi_settings.bypass_hub = bypass_hub;
                    }
                }
                if !midi_settings.bypass_hub {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!("Click Octave: {}", midi_settings.click_octave));
                        ui.separator();
                        if midi_settings.click_octave > Octave::P1 && ui.button("lower").clicked() {
                            midi_settings.click_octave = midi_settings.click_octave.get_lower();
                            Control::reload_tab(state, theme);
                        }
                        if midi_settings.click_octave < Octave::P7 && ui.button("higher").clicked()
                        {
                            midi_settings.click_octave = midi_settings.click_octave.get_higher();
                            Control::reload_tab(state, theme);
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut midi_settings.click_mute, "Mute");
                        ui.add(
                            Slider::new(&mut midi_settings.click_velocity, 0..=127).text("Click"),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut midi_settings.vocal_mute, "Mute");
                        ui.add(
                            Slider::new(&mut midi_settings.vocal_velocity, 0..=127).text("Vocal"),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut midi_settings.guitar_mute, "Mute");
                        ui.add(
                            Slider::new(&mut midi_settings.guitar_velocity, 0..=127).text("Guitar"),
                        );
                    });
                    if ui.button("Reset Audio").clicked() {
                        let default = MidiSettings::default();
                        midi_settings.click_mute = default.click_mute;
                        midi_settings.click_velocity = default.click_velocity;
                        midi_settings.vocal_mute = default.vocal_mute;
                        midi_settings.vocal_velocity = default.vocal_velocity;
                        midi_settings.guitar_mute = default.guitar_mute;
                        midi_settings.guitar_velocity = default.guitar_velocity;
                    }
                }
            });
    }
}
