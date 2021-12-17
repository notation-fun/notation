use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_egui::egui::{self, Ui, Slider, CollapsingHeader};
use bevy_egui::EguiContext;
use notation_bevy_utils::prelude::{
    BevyUtil, DockPanel, DockSide, LayoutAnchor, LayoutConstraint, LayoutSize, View, ViewBundle,
};
use float_eq::float_ne;
use notation_midi::prelude::{MidiState, PlayControlEvent, MidiSettings, JumpToBarEvent};
use notation_model::prelude::{Tab};

use crate::settings::layout_settings::LayoutMode;
use crate::ui::layout::NotationLayout;
use crate::viewer::control::Control;

use crate::prelude::{
    NotationAppState, NotationAssets, NotationSettings, NotationTheme, TabPathes,
    WindowResizedEvent, GuitarView,
};

#[derive(Clone, Debug)]
pub struct ControlView {
    pub tab: Arc<Tab>,
}
impl Display for ControlView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ControlView>({})", self.tab.bars.len())
    }
}
impl ControlView {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}

impl<'a> DockPanel<NotationLayout<'a>> for ControlView {
    fn dock_side(&self, _engine: &NotationLayout<'a>, _size: LayoutSize) -> DockSide {
        DockSide::Right
    }
}

impl<'a> View<NotationLayout<'a>> for ControlView {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::CENTER
    }
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        if Self::HUD_MODE || engine.state.hide_control {
            LayoutSize::ZERO
        } else {
            LayoutSize::new(Self::MIN_WIDTH, constraint.max.height)
        }
    }
}

impl ControlView {
    pub const HUD_MODE: bool = true;
    pub const MIN_WIDTH: f32 = 300.0;
    pub const MAX_WIDTH: f32 = 512.0;
    pub fn spawn(
        commands: &mut Commands,
        _materials: &mut ResMut<Assets<ColorMaterial>>,
        _assets: &NotationAssets,
        _theme: &NotationTheme,
        _settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let viewer_bundle = ViewBundle::from(ControlView::new(tab.clone()));
        let viewer_entity = BevyUtil::spawn_child_bundle(commands, entity, viewer_bundle);
        viewer_entity
    }
    pub fn calc_width(
        window_width: f32,
    ) -> f32 {
        let width = window_width * 0.25;
        if width < Self::MIN_WIDTH {
            Self::MIN_WIDTH
        } else if width > Self::MAX_WIDTH {
            Self::MAX_WIDTH
        } else {
            width
        }
    }
    pub fn overrides_ui(
        ui: &mut Ui,
        settings: &mut NotationSettings,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
        guitar_view_query: &mut Query<&mut Transform, With<Arc<GuitarView>>>,
        midi_state: &MidiState,
        jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    ) {
        CollapsingHeader::new("Override Sizes")
        .default_open(true)
        .show(ui, |ui| {
            let mut override_beat_size = settings.override_beat_size.is_some();
            ui.checkbox(&mut override_beat_size, "Override Beat Size");
            if override_beat_size {
                let mut beat_size = settings.override_beat_size.unwrap_or(80.0);
                let last_beat_size = beat_size;
                ui.add(Slider::new(&mut beat_size, 16.0..=512.0).text("Beat Size"));
                if settings.override_beat_size.is_none() || float_ne!(beat_size, last_beat_size, abs <= 1.0) {
                    settings.override_beat_size = Some(beat_size);
                    window_resized_evts.send(WindowResizedEvent());
                }
            } else if settings.override_beat_size.is_some() {
                settings.override_beat_size = None;
                window_resized_evts.send(WindowResizedEvent());
            }
            let mut override_chord_size = settings.override_chord_size.is_some();
            ui.checkbox(&mut override_chord_size, "Override Chord Size");
            if override_chord_size {
                let mut chord_size = settings.override_chord_size.unwrap_or(128.0);
                let last_chord_size = chord_size;
                ui.add(Slider::new(&mut chord_size, 48.0..=256.0).text("Chord Size"));
                if settings.override_chord_size.is_none() || float_ne!(chord_size, last_chord_size, abs <= 1.0) {
                    settings.override_chord_size = Some(chord_size);
                    window_resized_evts.send(WindowResizedEvent());
                }
            } else if settings.override_chord_size.is_some() {
                settings.override_chord_size = None;
                window_resized_evts.send(WindowResizedEvent());
            }
            let mut override_guitar_width = settings.override_guitar_width.is_some();
            ui.checkbox(&mut override_guitar_width, "Override Guitar Width");
            if override_guitar_width {
                let mut guitar_width = settings.override_guitar_width.unwrap_or(256.0);
                let last_guitar_width = guitar_width;
                ui.add(Slider::new(&mut guitar_width, 72.0..=1024.0).text("Guitar Width"));
                if settings.override_guitar_width.is_none() || float_ne!(guitar_width, last_guitar_width, abs <= 1.0) {
                    settings.override_guitar_width = Some(guitar_width);
                    window_resized_evts.send(WindowResizedEvent());
                }
            } else if settings.override_guitar_width.is_some() {
                settings.override_guitar_width = None;
                window_resized_evts.send(WindowResizedEvent());
            }
            let mut override_focus_offset_y = settings.layout.override_focus_offset_y.is_some();
            ui.checkbox(&mut override_focus_offset_y, "Override Focus Offset Y");
            if override_focus_offset_y {
                let mut offset_y = settings.layout.override_focus_offset_y.unwrap_or(0.0);
                let last_offset_y = offset_y;
                ui.add(Slider::new(&mut offset_y, -512.0..=512.0).text("Focus Offset Y"));
                if settings.layout.override_focus_offset_y.is_none() || float_ne!(offset_y, last_offset_y, abs <= 1.0) {
                    settings.layout.override_focus_offset_y = Some(offset_y);
                    Control::jump_to_center_bar(midi_state, jump_to_bar_evts);
                }
            } else if settings.layout.override_focus_offset_y.is_some() {
                settings.layout.override_focus_offset_y = None;
                Control::jump_to_center_bar(midi_state, jump_to_bar_evts);
            }
            let mut override_guitar_y = settings.override_guitar_y.is_some();
            ui.checkbox(&mut override_guitar_y, "Override Guitar Y");
            if override_guitar_y {
                let mut guitar_y = settings.override_guitar_y.unwrap_or(0.0);
                let last_guitar_y = guitar_y;
                ui.add(Slider::new(&mut guitar_y, -4096.0..=4096.0).text("Guitar Y"));
                if settings.override_guitar_y.is_none() || float_ne!(guitar_y, last_guitar_y, abs <= 1.0) {
                    settings.override_guitar_y = Some(guitar_y);
                    GuitarView::update_y(guitar_view_query, guitar_y);
                }
            } else if settings.override_guitar_y.is_some() {
                settings.override_guitar_y = None;
                window_resized_evts.send(WindowResizedEvent());
            }
        });
    }
    pub fn midi_settings_ui(
        ui: &mut Ui,
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
                    Control::pause(midi_state, play_control_evts);
                } else {
                    midi_settings.bypass_hub = bypass_hub;
                }
            }
            if !midi_settings.bypass_hub {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.checkbox(&mut midi_settings.vocal_mute, "Mute");
                    ui.add(Slider::new(&mut midi_settings.vocal_velocity, 0..=128).text("Vocal"));
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut midi_settings.guitar_mute, "Mute");
                    ui.add(Slider::new(&mut midi_settings.guitar_velocity, 0..=128).text("Guitar"));
                });
                if ui.button("Reset Audio").clicked() {
                    let default = MidiSettings::default();
                    midi_settings.vocal_mute = default.vocal_mute;
                    midi_settings.vocal_velocity = default.vocal_velocity;
                    midi_settings.guitar_mute = default.guitar_mute;
                    midi_settings.guitar_velocity = default.guitar_velocity;
                }
            }
        });
    }
    pub fn play_control_ui(
        ui: &mut Ui,
        settings: &mut NotationSettings,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        CollapsingHeader::new("Play Control")
        .default_open(true)
        .show(ui, |ui| {
            let play_title = if midi_state.play_control.play_state.is_playing() {
                "Pause"
            } else {
                "Play"
            };
            ui.horizontal(|ui| {
                if ui.button(play_title).clicked() {
                    Control::play_or_pause(midi_state, play_control_evts);
                }
                if ui.button("Stop").clicked() {
                    if midi_state.play_control.stop() {
                        Control::send_play_state_evt(midi_state, play_control_evts);
                    }
                }
            });
            let play_speed = settings.speed_factor;
            let should_loop = settings.should_loop;
            ui.horizontal(|ui| {
                ui.checkbox(&mut settings.should_loop, "Loop");
                if should_loop != settings.should_loop {
                    Control::sync_should_loop(
                        settings,
                        midi_state,
                        play_control_evts,
                    )
                }
                if ui.button(format!("Begin: {}", midi_state.play_control.begin_bar_ordinal)).clicked() {
                    Control::set_begin_bar_ordinal(midi_state, play_control_evts);
                }
                if ui.button(format!("End: {}", midi_state.play_control.end_bar_ordinal)).clicked() {
                    Control::set_end_bar_ordinal(midi_state, play_control_evts);
                }
                if ui.button("Clear").clicked() {
                    Control::clear_begin_end(midi_state, play_control_evts);
                }
            });
            ui.add(Slider::new(&mut settings.speed_factor, 0.1..=2.0).text("Speed"));
            if float_ne!(play_speed, settings.speed_factor, abs <= 0.01) {
                Control::sync_speed_factor(
                    settings,
                    midi_state,
                    play_control_evts,
                )
            }
        });
    }
    pub fn display_ui(
        ui: &mut Ui,
        state: &mut NotationAppState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        CollapsingHeader::new("Display Options")
        .default_open(true)
        .show(ui, |ui| {
            let show_guitar_syllable = settings.show_guitar_syllable;
            ui.checkbox(&mut settings.show_guitar_syllable, "Show Guitar Syllable");
            if show_guitar_syllable != settings.show_guitar_syllable {
                Control::reload_tab(state, theme);
            }
            let show_melody_syllable = settings.show_melody_syllable;
            ui.checkbox(&mut settings.show_melody_syllable, "Show Melody Syllable");
            if show_melody_syllable != settings.show_melody_syllable {
                Control::reload_tab(state, theme);
            }
            let show_syllable_as_num = settings.show_syllable_as_num;
            ui.checkbox(&mut settings.show_syllable_as_num, "Show Syllable as Numbers");
            if show_syllable_as_num != settings.show_syllable_as_num {
                if show_syllable_as_num {
                    settings.show_melody_syllable = true;
                }
                Control::reload_tab(state, theme);
            }
            let hide_bar_number = settings.hide_bar_number;
            ui.checkbox(&mut settings.hide_bar_number, "Hide Bar Number");
            if hide_bar_number != settings.hide_bar_number {
                Control::reload_tab(state, theme);
            }
            let always_show_fret = settings.always_show_fret;
            ui.checkbox(&mut settings.always_show_fret, "Always Show Fret");
            if always_show_fret != settings.always_show_fret {
                Control::reload_tab(state, theme);
            }
        });
    }
    pub fn layout_ui(
        ui: &mut Ui,
        state: &mut NotationAppState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        CollapsingHeader::new("Layout Options")
        .default_open(true)
        .show(ui, |ui| {
            let last_video_recording_mode = settings.layout.video_recording_mode;
            ui.checkbox(&mut settings.layout.video_recording_mode, "Video Recording Mode");
            if settings.layout.video_recording_mode != last_video_recording_mode {
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
            if !settings.layout.video_recording_mode && settings.layout.mode == LayoutMode::Grid {
                let last_force_centered = settings.layout.grid_force_centered;
                ui.checkbox(&mut settings.layout.grid_force_centered, "Force Centered");
                if settings.layout.grid_force_centered != last_force_centered {
                    Control::reload_tab(state, theme);
                }
            }
        });
    }
    pub fn tab_ui(
        ui: &mut Ui,
        asset_server: &AssetServer,
        state: &mut NotationAppState,
        _settings: &mut NotationSettings,
        theme: &mut NotationTheme,
        tab_pathes: &TabPathes,
    ) {
        if tab_pathes.0.len() > 1 {
            let width = Self::calc_width(state.window_width);
            egui::ComboBox::from_label("")
                .selected_text(state.tab_path.clone())
                .width(width - 24.0)
                .show_ui(ui, |ui| {
                    for path in tab_pathes.0.iter() {
                        if ui.selectable_label(*path == state.tab_path, path).clicked()
                        {
                            theme._bypass_systems = true;
                            state.change_tab(asset_server, path.clone());
                        }
                    }
                });
        }
        if ui.button("Reset Tab").clicked() {
            Control::reload_tab(state, theme);
        }
    }
    pub fn guitar_tab_display_ui(
        ui: &mut Ui,
        state: &mut NotationAppState,
        theme: &mut NotationTheme,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
    ) {
        CollapsingHeader::new("Guitar Tab")
        .default_open(true)
        .show(ui, |ui| {
            let last_string_space = theme.sizes.strings.string_space;
            ui.add(Slider::new(&mut theme.sizes.strings.string_space, 6.0..=32.0).text("String Space"));
            if float_ne!(theme.sizes.strings.string_space, last_string_space, abs <= 0.5) {
                window_resized_evts.send(WindowResizedEvent());
            }
            let last_note_height = theme.sizes.strings.note_height;
            ui.add(Slider::new(&mut theme.sizes.strings.note_height, 6.0..=32.0).text("Note Height"));
            if float_ne!(theme.sizes.strings.note_height, last_note_height, abs <= 0.5) {
                window_resized_evts.send(WindowResizedEvent());
            }
            let mut changed = false;
            let last_word_font_size = theme.texts.strings.fret_font_size;
            ui.add(Slider::new(&mut theme.texts.strings.fret_font_size, 6.0..=64.0).text("Fret Font Size"));
            if float_ne!(theme.texts.strings.fret_font_size, last_word_font_size, abs <= 0.5) {
                changed = true;
            }
            let last_word_text_x = theme.texts.strings.text_x;
            ui.add(Slider::new(&mut theme.texts.strings.text_x, -24.0..=24.0).text("Fret Offset X"));
            if float_ne!(theme.texts.strings.text_x, last_word_text_x, abs <= 0.5) {
                changed = true;
            }
            let last_word_text_y = theme.texts.strings.text_y;
            ui.add(Slider::new(&mut theme.texts.strings.text_y, -24.0..=24.0).text("Fret Offset Y"));
            if float_ne!(theme.texts.strings.text_y, last_word_text_y, abs <= 0.5) {
                changed = true;
            }
            if changed {
                Control::reload_tab( state, theme);
            }
        });
    }
    pub fn lyrics_display_ui(
        ui: &mut Ui,
        state: &mut NotationAppState,
        theme: &mut NotationTheme,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
    ) {
        CollapsingHeader::new("Lyrics")
        .default_open(true)
        .show(ui, |ui| {
            let last_line_height = theme.sizes.lyrics.line_height.idle;
            ui.add(Slider::new(&mut theme.sizes.lyrics.line_height.idle, 2.0..=64.0).text("Line Height (Idle)"));
            if float_ne!(theme.sizes.lyrics.line_height.idle, last_line_height, abs <= 0.5) {
                window_resized_evts.send(WindowResizedEvent());
            }
            let last_line_height = theme.sizes.lyrics.line_height.current;
            ui.add(Slider::new(&mut theme.sizes.lyrics.line_height.current, 2.0..=64.0).text("Line Height (Current)"));
            if float_ne!(theme.sizes.lyrics.line_height.current, last_line_height, abs <= 0.5) {
                window_resized_evts.send(WindowResizedEvent());
            }
            let last_line_height = theme.sizes.lyrics.line_height.played;
            ui.add(Slider::new(&mut theme.sizes.lyrics.line_height.played, 2.0..=64.0).text("Line Height (Played)"));
            if float_ne!(theme.sizes.lyrics.line_height.played, last_line_height, abs <= 0.5) {
                window_resized_evts.send(WindowResizedEvent());
            }
            let last_word_gap = theme.sizes.lyrics.word_gap;
            ui.add(Slider::new(&mut theme.sizes.lyrics.word_gap, 0.0..=8.0).text("Word Gap"));
            if float_ne!(theme.sizes.lyrics.word_gap, last_word_gap, abs <= 0.5) {
                window_resized_evts.send(WindowResizedEvent());
            }
            let mut changed = false;
            let last_word_font_size = theme.texts.lyrics.word_font_size;
            ui.add(Slider::new(&mut theme.texts.lyrics.word_font_size, 6.0..=64.0).text("Word Font Size"));
            if float_ne!(theme.texts.lyrics.word_font_size, last_word_font_size, abs <= 0.5) {
                changed = true;
            }
            let last_word_text_x = theme.texts.lyrics.text_x;
            ui.add(Slider::new(&mut theme.texts.lyrics.text_x, -24.0..=24.0).text("Word Offset X"));
            if float_ne!(theme.texts.lyrics.text_x, last_word_text_x, abs <= 0.5) {
                changed = true;
            }
            let last_word_text_y = theme.texts.lyrics.text_y;
            ui.add(Slider::new(&mut theme.texts.lyrics.text_y, -24.0..=24.0).text("Word Offset Y"));
            if float_ne!(theme.texts.lyrics.text_y, last_word_text_y, abs <= 0.5) {
                changed = true;
            }
            if changed {
                Control::reload_tab( state, theme);
            }
        });
    }
    pub fn melody_display_ui(
        ui: &mut Ui,
        state: &mut NotationAppState,
        theme: &mut NotationTheme,
        window_resized_evts: &mut EventWriter<WindowResizedEvent>,
    ) {
        CollapsingHeader::new("Melody")
        .default_open(true)
        .show(ui, |ui| {
            let last_note_height = theme.sizes.melody.note_height;
            ui.add(Slider::new(&mut theme.sizes.melody.note_height, 1.0..=32.0).text("Note Height"));
            if float_ne!(theme.sizes.melody.note_height, last_note_height, abs <= 0.5) {
                window_resized_evts.send(WindowResizedEvent());
            }
            let last_semitones = theme.sizes.melody.semitones;
            ui.add(Slider::new(&mut theme.sizes.melody.semitones, 12..=60).text("Semitones"));
            if theme.sizes.melody.semitones != last_semitones {
                window_resized_evts.send(WindowResizedEvent());
            }
            let mut changed = false;
            let last_syllable_font_size = theme.texts.melody.syllable_font_size;
            ui.add(Slider::new(&mut theme.texts.melody.syllable_font_size, 6.0..=64.0).text("Syllable Font Size"));
            if float_ne!(theme.texts.melody.syllable_font_size, last_syllable_font_size, abs <= 0.5) {
                changed = true;
            }
            let last_syllable_text_x = theme.texts.melody.text_x;
            ui.add(Slider::new(&mut theme.texts.melody.text_x, -24.0..=24.0).text("Syllable Offset X"));
            if float_ne!(theme.texts.melody.text_x, last_syllable_text_x, abs <= 0.5) {
                changed = true;
            }
            let last_syllable_text_y = theme.texts.melody.text_y;
            ui.add(Slider::new(&mut theme.texts.melody.text_y, -24.0..=24.0).text("Syllable Offset Y"));
            if float_ne!(theme.texts.melody.text_y, last_syllable_text_y, abs <= 0.5) {
                changed = true;
            }
            if changed {
                Control::reload_tab( state, theme);
            }
        });
    }
    pub fn window_size_ui(
        ui: &mut Ui,
        window: &mut Window,
    ) {
        CollapsingHeader::new(format!("Window: {} x {}", window.width() as i32, window.height() as i32))
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
                if ui.button("1920 x 1080").clicked() {
                    Control::set_window_size(window, 1920, 1080);
                }
                if ui.button("1080 x 1920").clicked() {
                    Control::set_window_size(window, 1080, 1920);
                }
            });
        });
    }
    pub fn window_sizes_ui(
        ui: &mut Ui,
        windows: &mut Windows,
    ) {
        if let Some(window) = windows.get_primary_mut() {
            Self::window_size_ui(ui, window);
            ui.separator();
        }
    }
    pub fn control_ui(
        egui_ctx: Res<EguiContext>,
        mut windows: ResMut<Windows>,
        asset_server: Res<AssetServer>,
        mut state: ResMut<NotationAppState>,
        mut settings: ResMut<NotationSettings>,
        mut theme: ResMut<NotationTheme>,
        tab_pathes: Res<TabPathes>,
        mut midi_settings: ResMut<MidiSettings>,
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
        mut window_resized_evts: EventWriter<WindowResizedEvent>,
        mut guitar_view_query: Query<&mut Transform, With<Arc<GuitarView>>>,
        mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
    ) {
        if state.hide_control {
            return;
        }
        let width = Self::calc_width(state.window_width);
        egui::SidePanel::right("control")
            .min_width(width)
            .max_width(width)
            .show(egui_ctx.ctx(), |ui| {
                ui.vertical(|ui| {
                    /*
                    if ui.button("Hide Control\n(Press Tab to Show)").clicked() {
                        state.hide_control = true;
                        window_resized_evts.send(WindowResizedEvent());
                    }
                    ui.separator();
                     */
                    Self::tab_ui(ui, &asset_server, &mut state, &mut settings, &mut theme, &tab_pathes);
                    ui.separator();
                    Self::play_control_ui(ui, &mut settings, &mut midi_state, &mut play_control_evts);
                    ui.separator();
                    egui::ScrollArea::auto_sized().show(ui, |ui| {
                        ui.vertical(|ui| {
                            Self::midi_settings_ui(ui, &mut midi_settings, &mut midi_state, &mut play_control_evts);
                            Self::display_ui(ui, &mut state, &mut settings, &mut theme);
                            ui.separator();
                            Self::layout_ui(ui, &mut state, &mut settings, &mut theme);
                            Self::overrides_ui(ui, &mut settings, &mut window_resized_evts, &mut guitar_view_query, &midi_state, &mut jump_to_bar_evts);
                            ui.separator();
                            #[cfg(not(target_arch = "wasm32"))]
                            Self::window_sizes_ui(ui, &mut windows);
                            ui.label("Override Theme");
                            Self::guitar_tab_display_ui(ui, &mut state, &mut theme, &mut window_resized_evts);
                            Self::lyrics_display_ui(ui, &mut state, &mut theme, &mut window_resized_evts);
                            Self::melody_display_ui(ui, &mut state, &mut theme, &mut window_resized_evts);
                            ui.separator();
                            if ui.button("Reset Theme").clicked() {
                                *theme = NotationTheme::default();
                                Control::reload_tab(&mut state, &mut theme);
                            }
                        });
                    });
                });
            });
    }
}
