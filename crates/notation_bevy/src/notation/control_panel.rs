use bevy::prelude::*;
use bevy_egui::egui::{self, CollapsingHeader, Slider, Ui};
use bevy_egui::EguiContext;
use float_eq::float_ne;
use notation_model::prelude::{JumpToBarEvent, PlayControlEvent};

use crate::settings::layout_settings::{GridAlignMode, LayoutMode};
use super::control::Control;

use crate::prelude::{
    GuitarView, NotationState, NotationSettings, NotationTheme, TabAsset,
    NotationArgs, WindowResizedEvent,
};

#[cfg(feature = "midi")]
use notation_midi::prelude::{MidiSettings, MidiState};

#[cfg(feature = "midi")]
use crate::midi::midi_control::MidiControl;

#[derive(Clone, Debug)]
pub struct ControlPanel {
}

impl ControlPanel {
    pub const HUD_MODE: bool = true;
    pub const MIN_WIDTH: f32 = 320.0;
    pub const MAX_WIDTH: f32 = 512.0;
    pub fn calc_width(window_width: f32) -> f32 {
        let width = window_width * 0.30;
        if width < Self::MIN_WIDTH {
            Self::MIN_WIDTH
        } else if width > Self::MAX_WIDTH {
            Self::MAX_WIDTH
        } else {
            width
        }
    }
    pub fn is_pos_inside(window_width: f32, pos: Vec2) -> bool {
        window_width / 2.0 - pos.x <= ControlPanel::calc_width(window_width)
    }
    pub fn overrides_ui(
        ui: &mut Ui,
        state: &NotationState,
        settings: &mut NotationSettings,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
        guitar_view_query: &mut Query<&mut Transform, With<GuitarView>>,
        #[cfg(feature = "midi")]
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        CollapsingHeader::new("Override Sizes")
            .default_open(true)
            .show(ui, |ui| {
                let mut override_tab_width = settings.layout.override_tab_width.is_some();
                ui.checkbox(&mut override_tab_width, "Override Tab Width");
                if override_tab_width {
                    let mut tab_width = settings.layout.override_tab_width.unwrap_or(512.0);
                    let last_tab_width = tab_width;
                    ui.add(Slider::new(&mut tab_width, 256.0..=1024.0).text("Tab Width"));
                    if settings.layout.override_tab_width.is_none()
                        || float_ne!(tab_width, last_tab_width, abs <= 1.0)
                    {
                        settings.layout.override_tab_width = Some(tab_width);
                        window_resized_evts.send(WindowResizedEvent::new(&state));
                    }
                } else if settings.layout.override_tab_width.is_some() {
                    settings.layout.override_tab_width = None;
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let mut override_beat_size = settings.override_beat_size.is_some();
                ui.checkbox(&mut override_beat_size, "Override Beat Size");
                if override_beat_size {
                    let mut beat_size = settings.override_beat_size.unwrap_or(80.0);
                    let last_beat_size = beat_size;
                    ui.add(Slider::new(&mut beat_size, 16.0..=512.0).text("Beat Size"));
                    if settings.override_beat_size.is_none()
                        || float_ne!(beat_size, last_beat_size, abs <= 1.0)
                    {
                        settings.override_beat_size = Some(beat_size);
                        window_resized_evts.send(WindowResizedEvent::new(&state));
                    }
                } else if settings.override_beat_size.is_some() {
                    settings.override_beat_size = None;
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let mut override_chord_size = settings.override_chord_size.is_some();
                ui.checkbox(&mut override_chord_size, "Override Chord Size");
                if override_chord_size {
                    let mut chord_size = settings.override_chord_size.unwrap_or(128.0);
                    let last_chord_size = chord_size;
                    ui.add(Slider::new(&mut chord_size, 48.0..=256.0).text("Chord Size"));
                    if settings.override_chord_size.is_none()
                        || float_ne!(chord_size, last_chord_size, abs <= 1.0)
                    {
                        settings.override_chord_size = Some(chord_size);
                        window_resized_evts.send(WindowResizedEvent::new(&state));
                    }
                } else if settings.override_chord_size.is_some() {
                    settings.override_chord_size = None;
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let mut override_guitar_width = settings.override_guitar_width.is_some();
                ui.checkbox(&mut override_guitar_width, "Override Guitar Width");
                if override_guitar_width {
                    let mut guitar_width = settings.override_guitar_width.unwrap_or(256.0);
                    let last_guitar_width = guitar_width;
                    ui.add(Slider::new(&mut guitar_width, 72.0..=1024.0).text("Guitar Width"));
                    if settings.override_guitar_width.is_none()
                        || float_ne!(guitar_width, last_guitar_width, abs <= 1.0)
                    {
                        settings.override_guitar_width = Some(guitar_width);
                        window_resized_evts.send(WindowResizedEvent::new(&state));
                    }
                } else if settings.override_guitar_width.is_some() {
                    settings.override_guitar_width = None;
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let mut override_focus_offset_y = settings.layout.override_focus_offset_y.is_some();
                ui.checkbox(&mut override_focus_offset_y, "Override Focus Offset Y");
                if override_focus_offset_y {
                    let mut offset_y = settings.layout.override_focus_offset_y.unwrap_or(0.0);
                    let last_offset_y = offset_y;
                    ui.add(Slider::new(&mut offset_y, -512.0..=512.0).text("Focus Offset Y"));
                    if settings.layout.override_focus_offset_y.is_none()
                        || float_ne!(offset_y, last_offset_y, abs <= 1.0)
                    {
                        settings.layout.override_focus_offset_y = Some(offset_y);
                        #[cfg(feature = "midi")]
                        MidiControl::jump_to_center_bar(midi_state, jump_to_bar_evts);
                    }
                } else if settings.layout.override_focus_offset_y.is_some() {
                    settings.layout.override_focus_offset_y = None;
                    #[cfg(feature = "midi")]
                    MidiControl::jump_to_center_bar(midi_state, jump_to_bar_evts);
                }
                let mut override_guitar_y = settings.override_guitar_y.is_some();
                ui.checkbox(&mut override_guitar_y, "Override Guitar Y");
                if override_guitar_y {
                    let mut guitar_y = settings.override_guitar_y.unwrap_or(0.0);
                    let last_guitar_y = guitar_y;
                    ui.add(Slider::new(&mut guitar_y, -4096.0..=4096.0).text("Guitar Y"));
                    if settings.override_guitar_y.is_none()
                        || float_ne!(guitar_y, last_guitar_y, abs <= 1.0)
                    {
                        settings.override_guitar_y = Some(guitar_y);
                        GuitarView::update_y(guitar_view_query, guitar_y);
                    }
                } else if settings.override_guitar_y.is_some() {
                    settings.override_guitar_y = None;
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
            });
    }
    pub fn display_ui(
        ui: &mut Ui,
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        CollapsingHeader::new("Display Options")
            .default_open(true)
            .show(ui, |ui| {
                let mut hide_harmony_lane = settings.hide_harmony_lane;
                ui.checkbox(&mut hide_harmony_lane, "Hide Guitar Notes");
                if settings.hide_harmony_lane != hide_harmony_lane {
                    settings.hide_harmony_lane = hide_harmony_lane;
                    Control::reload_tab(state, theme);
                }
                let mut hide_shapes_lane = settings.hide_shapes_lane;
                ui.checkbox(&mut hide_shapes_lane, "Hide Guitar Chords");
                if settings.hide_shapes_lane != hide_shapes_lane {
                    settings.hide_shapes_lane = hide_shapes_lane;
                    Control::reload_tab(state, theme);
                }
                let mut hide_strings_lane = settings.hide_strings_lane;
                ui.checkbox(&mut hide_strings_lane, "Hide Guitar Strings");
                if settings.hide_strings_lane != hide_strings_lane {
                    settings.hide_strings_lane = hide_strings_lane;
                    Control::reload_tab(state, theme);
                }
                let mut hide_lyrics_lane = settings.hide_lyrics_lane;
                ui.checkbox(&mut hide_lyrics_lane, "Hide Lyrics ");
                if settings.hide_lyrics_lane != hide_lyrics_lane {
                    settings.hide_lyrics_lane = hide_lyrics_lane;
                    Control::reload_tab(state, theme);
                }
                let mut hide_melody_lane = settings.hide_melody_lane;
                ui.checkbox(&mut hide_melody_lane, "Hide Melody");
                if settings.hide_melody_lane != hide_melody_lane {
                    settings.hide_melody_lane = hide_melody_lane;
                    Control::reload_tab(state, theme);
                }
                ui.separator();
                let show_note_pitch = settings.show_note_pitch;
                ui.checkbox(
                    &mut settings.show_note_pitch,
                    "Show Note Pitch",
                );
                if show_note_pitch != settings.show_note_pitch {
                    Control::reload_tab(state, theme);
                }
                let show_note_syllable = settings.show_note_syllable;
                ui.checkbox(&mut settings.show_note_syllable, "Show Note Syllable");
                if show_note_syllable != settings.show_note_syllable {
                    Control::reload_tab(state, theme);
                }
                let show_syllable_as_num = settings.show_syllable_as_num;
                ui.checkbox(
                    &mut settings.show_syllable_as_num,
                    "Show Syllable as Numbers",
                );
                if show_syllable_as_num != settings.show_syllable_as_num {
                    Control::reload_tab(state, theme);
                }
                let hide_bar_number = settings.hide_bar_number;
                ui.checkbox(&mut settings.hide_bar_number, "Hide Bar Number");
                if hide_bar_number != settings.hide_bar_number {
                    Control::reload_tab(state, theme);
                }
                let hide_indicators = settings.hide_indicators;
                ui.checkbox(&mut settings.hide_indicators, "Hide Indicators");
                if hide_indicators != settings.hide_indicators {
                    Control::reload_tab(state, theme);
                }
                let always_show_fret = settings.always_show_fret;
                ui.checkbox(&mut settings.always_show_fret, "Always Show Fret");
                if always_show_fret != settings.always_show_fret {
                    Control::reload_tab(state, theme);
                }
                ui.separator();
                let mut hide_guitar_view = settings.hide_guitar_view;
                ui.checkbox(&mut hide_guitar_view, "Hide Guitar View");
                if settings.hide_guitar_view != hide_guitar_view {
                    settings.hide_guitar_view = hide_guitar_view;
                    Control::reload_tab(state, theme);
                }
                let mut hide_chords_view = settings.hide_chords_view;
                ui.checkbox(&mut hide_chords_view, "Hide Chords View");
                if settings.hide_chords_view != hide_chords_view {
                    settings.hide_chords_view = hide_chords_view;
                    Control::reload_tab(state, theme);
                }
                let mut hide_mini_map = settings.hide_mini_map;
                ui.checkbox(&mut hide_mini_map, "Hide Mini Map");
                if settings.hide_mini_map != hide_mini_map {
                    settings.hide_mini_map = hide_mini_map;
                    Control::reload_tab(state, theme);
                }
            });
    }
    pub fn layout_ui(
        ui: &mut Ui,
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        CollapsingHeader::new("Layout Options")
            .default_open(true)
            .show(ui, |ui| {
                let mut video_recording_mode = settings.layout.video_recording_mode;
                ui.checkbox(&mut video_recording_mode, "Video Recording Mode");
                if settings.layout.video_recording_mode != video_recording_mode {
                    settings.layout.video_recording_mode = video_recording_mode;
                    Control::reload_tab(state, theme);
                }
                ui.separator();
                let mode_text = if settings.layout.mode == LayoutMode::Grid {
                    "Switch to Line Mode"
                } else {
                    "Switch to Grid Mode"
                };
                if ui.button(mode_text).clicked() {
                    Control::toggle_layout_mode(state, settings, theme);
                }
                if !settings.layout.video_recording_mode && settings.layout.mode == LayoutMode::Grid
                {
                    ui.label("Switch Align Mode");
                    ui.horizontal(|ui| {
                        if settings.layout.grid_align_mode != GridAlignMode::Top {
                            if ui.button("Top").clicked() {
                                settings.layout.grid_align_mode = GridAlignMode::Top;
                                Control::reload_tab(state, theme);
                            }
                        } else {
                            ui.label("Top");
                        }
                        if settings.layout.grid_align_mode != GridAlignMode::ForceTop {
                            if ui.button("Force Top").clicked() {
                                settings.layout.grid_align_mode = GridAlignMode::ForceTop;
                                Control::reload_tab(state, theme);
                            }
                        } else {
                            ui.label("Force Top");
                        }
                    });
                    ui.horizontal(|ui| {
                        if settings.layout.grid_align_mode != GridAlignMode::Center {
                            if ui.button("Center").clicked() {
                                settings.layout.grid_align_mode = GridAlignMode::Center;
                                Control::reload_tab(state, theme);
                            }
                        } else {
                            ui.label("Center");
                        }
                        if settings.layout.grid_align_mode != GridAlignMode::ForceCenter {
                            if ui.button("Force Center").clicked() {
                                settings.layout.grid_align_mode = GridAlignMode::ForceCenter;
                                Control::reload_tab(state, theme);
                            }
                        } else {
                            ui.label("Force Center");
                        }
                    });
                }
            });
    }
    pub fn tab_ui(
        ui: &mut Ui,
        args: &mut NotationArgs,
        state: &mut NotationState,
        _settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        if theme._bypass_systems {
            if state.tab_error.is_some() {
                ui.label("Load Tab Failed");
                ui.separator();
                ui.label(format!("{:?}", state.tab_error.as_ref().unwrap()));
            } else {
                ui.label("Loading Tab ...");
            }
            ui.separator();
        }
        ui.horizontal(|ui| {
            if ui.button("Reload Tab").clicked() {
                state.bars_range = None;
                Control::reload_tab(state, theme);
            }
            ui.separator();
            if ui.button("Help").clicked() {
                state.show_kb = true;
            }
            egui::warn_if_debug_build(ui);
            #[cfg(not(target_arch = "wasm32"))]
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                ui.separator();
                if ui.button("Open Tab").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Fun Notation", &TabAsset::EXTENSIONS)
                        .pick_file()
                    {
                        let path_str = path.clone().into_os_string().into_string();
                        if let Ok(path_str) = path_str {
                            args.tab.insert(0, path_str.clone());
                            state.change_tab(theme, path_str.clone());
                        } else {
                            println!(
                                "Failed to convert path to string: {:?} -> {:?}",
                                path, path_str
                            );
                        }
                    }
                }
            });
        });
        if args.tab.len() > 1 {
            let width = Self::calc_width(state.window_width);
            egui::ComboBox::from_id_source("tab")
                .selected_text(state.tab_path.clone())
                .width(width - 24.0)
                .show_ui(ui, |ui| {
                    for path in args.tab.iter() {
                        if ui.selectable_label(*path == state.tab_path, path).clicked() {
                            state.change_tab(theme, path.clone());
                        }
                    }
                });
        }
    }
    pub fn guitar_tab_display_ui(
        ui: &mut Ui,
        state: &mut NotationState,
        theme: &mut NotationTheme,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
    ) {
        CollapsingHeader::new("Guitar Tab")
            .default_open(true)
            .show(ui, |ui| {
                let last_string_space = theme.sizes.strings.string_space;
                ui.add(
                    Slider::new(&mut theme.sizes.strings.string_space, 6.0..=32.0)
                        .text("String Space"),
                );
                if float_ne!(
                    theme.sizes.strings.string_space,
                    last_string_space,
                    abs <= 0.5
                ) {
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let last_note_height = theme.sizes.strings.note_height;
                ui.add(
                    Slider::new(&mut theme.sizes.strings.note_height, 6.0..=32.0)
                        .text("Note Height"),
                );
                if float_ne!(
                    theme.sizes.strings.note_height,
                    last_note_height,
                    abs <= 0.5
                ) {
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let mut changed = false;
                let last_word_font_size = theme.texts.strings.fret_font_size;
                ui.add(
                    Slider::new(&mut theme.texts.strings.fret_font_size, 6.0..=64.0)
                        .text("Fret Font Size"),
                );
                if float_ne!(
                    theme.texts.strings.fret_font_size,
                    last_word_font_size,
                    abs <= 0.5
                ) {
                    changed = true;
                }
                let last_word_text_x = theme.texts.strings.text_x;
                ui.add(
                    Slider::new(&mut theme.texts.strings.text_x, -24.0..=24.0)
                        .text("Fret Offset X"),
                );
                if float_ne!(theme.texts.strings.text_x, last_word_text_x, abs <= 0.5) {
                    changed = true;
                }
                let last_word_text_y = theme.texts.strings.text_y;
                ui.add(
                    Slider::new(&mut theme.texts.strings.text_y, -24.0..=24.0)
                        .text("Fret Offset Y"),
                );
                if float_ne!(theme.texts.strings.text_y, last_word_text_y, abs <= 0.5) {
                    changed = true;
                }
                if changed {
                    Control::reload_tab(state, theme);
                }
            });
    }
    pub fn lyrics_display_ui(
        ui: &mut Ui,
        state: &mut NotationState,
        theme: &mut NotationTheme,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
    ) {
        CollapsingHeader::new("Lyrics")
            .default_open(true)
            .show(ui, |ui| {
                let last_line_height = theme.sizes.lyrics.line_height.idle;
                ui.add(
                    Slider::new(&mut theme.sizes.lyrics.line_height.idle, 2.0..=64.0)
                        .text("Line Height (Idle)"),
                );
                if float_ne!(
                    theme.sizes.lyrics.line_height.idle,
                    last_line_height,
                    abs <= 0.5
                ) {
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let last_line_height = theme.sizes.lyrics.line_height.current;
                ui.add(
                    Slider::new(&mut theme.sizes.lyrics.line_height.current, 2.0..=64.0)
                        .text("Line Height (Current)"),
                );
                if float_ne!(
                    theme.sizes.lyrics.line_height.current,
                    last_line_height,
                    abs <= 0.5
                ) {
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let last_line_height = theme.sizes.lyrics.line_height.played;
                ui.add(
                    Slider::new(&mut theme.sizes.lyrics.line_height.played, 2.0..=64.0)
                        .text("Line Height (Played)"),
                );
                if float_ne!(
                    theme.sizes.lyrics.line_height.played,
                    last_line_height,
                    abs <= 0.5
                ) {
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let last_word_gap = theme.sizes.lyrics.word_gap;
                ui.add(Slider::new(&mut theme.sizes.lyrics.word_gap, 0.0..=8.0).text("Word Gap"));
                if float_ne!(theme.sizes.lyrics.word_gap, last_word_gap, abs <= 0.5) {
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let mut changed = false;
                let last_word_font_size = theme.texts.lyrics.word_font_size;
                ui.add(
                    Slider::new(&mut theme.texts.lyrics.word_font_size, 6.0..=64.0)
                        .text("Word Font Size"),
                );
                if float_ne!(
                    theme.texts.lyrics.word_font_size,
                    last_word_font_size,
                    abs <= 0.5
                ) {
                    changed = true;
                }
                let last_word_text_x = theme.texts.lyrics.text_x;
                ui.add(
                    Slider::new(&mut theme.texts.lyrics.text_x, -24.0..=24.0).text("Word Offset X"),
                );
                if float_ne!(theme.texts.lyrics.text_x, last_word_text_x, abs <= 0.5) {
                    changed = true;
                }
                let last_word_text_y = theme.texts.lyrics.text_y;
                ui.add(
                    Slider::new(&mut theme.texts.lyrics.text_y, -24.0..=24.0).text("Word Offset Y"),
                );
                if float_ne!(theme.texts.lyrics.text_y, last_word_text_y, abs <= 0.5) {
                    changed = true;
                }
                if changed {
                    Control::reload_tab(state, theme);
                }
            });
    }
    pub fn melody_display_ui(
        ui: &mut Ui,
        state: &mut NotationState,
        theme: &mut NotationTheme,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
    ) {
        CollapsingHeader::new("Melody")
            .default_open(true)
            .show(ui, |ui| {
                let last_note_height = theme.sizes.melody.note_height;
                ui.add(
                    Slider::new(&mut theme.sizes.melody.note_height, 1.0..=32.0)
                        .text("Note Height"),
                );
                if float_ne!(theme.sizes.melody.note_height, last_note_height, abs <= 0.5) {
                    window_resized_evts.send(WindowResizedEvent::new(&state));
                }
                let mut changed = false;
                let last_syllable_font_size = theme.texts.melody.syllable_font_size;
                ui.add(
                    Slider::new(&mut theme.texts.melody.syllable_font_size, 6.0..=64.0)
                        .text("Syllable Font Size"),
                );
                if float_ne!(
                    theme.texts.melody.syllable_font_size,
                    last_syllable_font_size,
                    abs <= 0.5
                ) {
                    changed = true;
                }
                let last_syllable_text_x = theme.texts.melody.text_x;
                ui.add(
                    Slider::new(&mut theme.texts.melody.text_x, -24.0..=24.0)
                        .text("Syllable Offset X"),
                );
                if float_ne!(theme.texts.melody.text_x, last_syllable_text_x, abs <= 0.5) {
                    changed = true;
                }
                let last_syllable_text_y = theme.texts.melody.text_y;
                ui.add(
                    Slider::new(&mut theme.texts.melody.text_y, -24.0..=24.0)
                        .text("Syllable Offset Y"),
                );
                if float_ne!(theme.texts.melody.text_y, last_syllable_text_y, abs <= 0.5) {
                    changed = true;
                }
                if changed {
                    Control::reload_tab(state, theme);
                }
            });
    }
    pub fn window_size_ui(ui: &mut Ui, window: &mut Window) {
        CollapsingHeader::new(format!(
            "Window: {} x {}",
            window.width() as i32,
            window.height() as i32
        ))
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("1280 x 720").clicked() {
                    Control::set_window_size(window, 1280, 720);
                }
                if ui.button("720 x 1280").clicked() {
                    Control::set_window_size(window, 720, 1280);
                }
            });
            ui.horizontal(|ui| {
                if ui.button("1440 x 810").clicked() {
                    Control::set_window_size(window, 1440, 810);
                }
                if ui.button("810 x 1440").clicked() {
                    Control::set_window_size(window, 810, 1440);
                }
            });
            ui.horizontal(|ui| {
                if ui.button("1920 x 1080").clicked() {
                    Control::set_window_size(window, 1920, 1080);
                }
                if ui.button("1080 x 1920").clicked() {
                    Control::set_window_size(window, 1080, 1920);
                }
            });
        });
    }
    pub fn window_sizes_ui(ui: &mut Ui, windows: &mut Windows) {
        if let Some(window) = windows.get_primary_mut() {
            Self::window_size_ui(ui, window);
            ui.separator();
        }
    }
    pub fn presets_ui(
        ui: &mut Ui,
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
        windows: &mut Windows,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
    ) {
        CollapsingHeader::new(format!(
            "Preset: {}",
            state.preset.clone().unwrap_or("".to_string())
        ))
        .default_open(true)
        .show(ui, |ui| {
            for preset in Control::ALL_PRESETS.iter() {
                if ui.button(*preset).clicked() {
                    Control::set_preset(state, settings, theme, windows, window_resized_evts, *preset);
                }
            }
        });
    }
    pub fn control_ui(
        mut egui_ctx: ResMut<EguiContext>,
        mut windows: ResMut<Windows>,
        mut args: ResMut<NotationArgs>,
        mut state: ResMut<NotationState>,
        mut settings: ResMut<NotationSettings>,
        mut theme: ResMut<NotationTheme>,
        #[cfg(feature = "midi")]
        mut midi_settings: ResMut<MidiSettings>,
        #[cfg(feature = "midi")]
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
        mut window_resized_evts: EventWriter<WindowResizedEvent>,
        mut guitar_view_query: Query<&mut Transform, With<GuitarView>>,
        mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
    ) {
        if !state.show_control {
            return;
        }
        let width = Self::calc_width(state.window_width);
        egui::SidePanel::right("control")
            .min_width(width)
            .max_width(width)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.vertical(|ui| {
                    /*
                    if ui.button("Hide Control\n(Press Tab to Show)").clicked() {
                        state.hide_control = true;
                        window_resized_evts.send(WindowResizedEvent::new(&state));
                    }
                    ui.separator();
                     */
                    Self::tab_ui(ui, &mut args, &mut state, &mut settings, &mut theme);
                    ui.separator();
                    #[cfg(feature = "midi")]
                    {
                        Self::play_control_ui(
                            ui,
                            &mut state,
                            &mut theme,
                            &mut settings,
                            &mut midi_state,
                            &mut play_control_evts,
                        );
                        ui.separator();
                    }
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical(|ui| {
                            #[cfg(feature = "midi")]
                            Self::midi_settings_ui(
                                ui,
                                &mut state,
                                &mut theme,
                                &mut midi_settings,
                                &mut midi_state,
                                &mut play_control_evts,
                            );
                            Self::display_ui(ui, &mut state, &mut settings, &mut theme);
                            ui.separator();
                            Self::layout_ui(ui, &mut state, &mut settings, &mut theme);
                            Self::overrides_ui(
                                ui,
                                &state,
                                &mut settings,
                                &mut window_resized_evts,
                                &mut guitar_view_query,
                                #[cfg(feature = "midi")]
                                &midi_state,
                                &mut jump_to_bar_evts,
                            );
                            ui.separator();
                            #[cfg(not(target_arch = "wasm32"))]
                            Self::window_sizes_ui(ui, &mut windows);
                            ui.label("Override Theme");
                            Self::guitar_tab_display_ui(
                                ui,
                                &mut state,
                                &mut theme,
                                &mut window_resized_evts,
                            );
                            Self::lyrics_display_ui(
                                ui,
                                &mut state,
                                &mut theme,
                                &mut window_resized_evts,
                            );
                            Self::melody_display_ui(
                                ui,
                                &mut state,
                                &mut theme,
                                &mut window_resized_evts,
                            );
                            ui.separator();
                            if ui.button("Reset Theme").clicked() {
                                *theme = NotationTheme::default();
                                Control::reload_tab(&mut state, &mut theme);
                            }
                            ui.separator();
                            Self::presets_ui(ui, &mut state, &mut settings, &mut theme, &mut windows, &mut window_resized_evts);
                        });
                    });
                });
            });
    }
}
