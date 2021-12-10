use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy::utils::Uuid;
use bevy_egui::egui::{self, Slider};
use bevy_egui::EguiContext;
use notation_bevy_utils::prelude::{
    BevyUtil, DockPanel, DockSide, LayoutAnchor, LayoutConstraint, LayoutSize, View, ViewBundle,
};
use float_eq::float_ne;
use notation_midi::prelude::{MidiState, PlayControlEvent, MidiSettings};
use notation_model::play::play_control::TickResult;
use notation_model::prelude::{Tab, Units};

use crate::settings::layout_settings::LayoutMode;
use crate::ui::layout::NotationLayout;

use crate::prelude::{
    NotationAppState, NotationAssets, NotationSettings, NotationTheme, TabPathes,
    WindowResizedEvent, GuitarView,
};

use super::app::NotationViewer;

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
            LayoutSize::new(Self::WIDTH, constraint.max.height)
        }
    }
}

impl ControlView {
    pub const HUD_MODE: bool = true;
    pub const WIDTH: f32 = 256.0;
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
    pub fn reload_tab(
        commands: &mut Commands,
        state: &mut NotationAppState,
        viewer_query: &Query<(Entity, &Arc<NotationViewer>), With<Arc<NotationViewer>>>,
    ) {
        for (entity, viewer) in viewer_query.iter() {
            if viewer.uuid == state.viewer_uuid {
                commands.entity(entity).despawn_recursive();
            }
        }
        state.tab = None;
        state.viewer_uuid = Uuid::new_v4();
    }
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
        midi_state
            .play_control
            .should_loop = settings.should_loop;
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
    pub fn stop(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        if midi_state.play_control.stop() {
            midi_state.play_control.position.bar.bar_ordinal = midi_state.play_control.begin_bar_ordinal;
            midi_state.play_control.position.bar.in_bar_pos = Units(0.0);
            Self::send_play_state_evt(midi_state, play_control_evts);
        }
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
        midi_state.play_control.end_bar_ordinal = midi_state.play_control.get_last_car_ordinal();
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
    pub fn toggle_layout_mode(
        commands: &mut Commands,
        state: &mut ResMut<NotationAppState>,
        settings: &mut ResMut<NotationSettings>,
        viewer_query: &Query<(Entity, &Arc<NotationViewer>), With<Arc<NotationViewer>>>,
    ) {
        if settings.layout.mode == LayoutMode::Grid {
            settings.layout.mode = LayoutMode::Line;
        } else {
            settings.layout.mode = LayoutMode::Grid;
        }
        Self::reload_tab(commands, state, viewer_query);
    }

    pub fn control_ui(
        mut commands: Commands,
        egui_ctx: Res<EguiContext>,
        asset_server: Res<AssetServer>,
        mut state: ResMut<NotationAppState>,
        mut settings: ResMut<NotationSettings>,
        viewer_query: Query<(Entity, &Arc<NotationViewer>), With<Arc<NotationViewer>>>,
        tab_pathes: Res<TabPathes>,
        mut midi_settings: ResMut<MidiSettings>,
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
        mut window_resized_evts: EventWriter<WindowResizedEvent>,
        mut guitar_view_query: Query<&mut Transform, With<Arc<GuitarView>>>,
    ) {
        if state.hide_control {
            return;
        }
        egui::SidePanel::right("control")
            .min_width(Self::WIDTH)
            .max_width(state.window_width)
            .show(egui_ctx.ctx(), |ui| {
                ui.vertical(|ui| {
                    if ui.button("Hide Control\n(Press Tab to Show)").clicked() {
                        state.hide_control = true;
                        window_resized_evts.send(WindowResizedEvent());
                    }
                    ui.separator();
                    ui.checkbox(&mut midi_settings.bypass_hub, "Bypass Midi Hub");
                    ui.separator();
                    let play_title = if midi_state.play_control.play_state.is_playing() {
                        "Pause"
                    } else {
                        "Play"
                    };
                    if ui.button(play_title).clicked() {
                        Self::play_or_pause(&mut midi_state, &mut play_control_evts);
                    }
                    if !midi_state.play_control.play_state.is_stopped() {
                        if ui.button("Stop").clicked() {
                            if midi_state.play_control.stop() {
                                Self::send_play_state_evt(&midi_state, &mut play_control_evts);
                            }
                        }
                    }
                    let play_speed = settings.speed_factor;
                    let should_loop = settings.should_loop;
                    ui.checkbox(&mut settings.should_loop, "Loop");
                    if should_loop != settings.should_loop {
                        Self::sync_should_loop(
                            &settings,
                            &mut midi_state,
                            &mut play_control_evts,
                        )
                    }
                    if ui.button(format!("Begin: {}", midi_state.play_control.begin_bar_ordinal)).clicked() {
                        Self::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
                    }
                    if ui.button(format!("End: {}", midi_state.play_control.end_bar_ordinal)).clicked() {
                        Self::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
                    }
                    ui.add(Slider::new(&mut settings.speed_factor, 0.1..=2.0).text("Speed"));
                    if float_ne!(play_speed, settings.speed_factor, abs <= 0.01) {
                        Self::sync_speed_factor(
                            &settings,
                            &mut midi_state,
                            &mut play_control_evts,
                        )
                    }
                    ui.separator();
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
                    let mut override_guitar_y = settings.override_guitar_y.is_some();
                    ui.checkbox(&mut override_guitar_y, "Override Guitar Y");
                    if override_guitar_y {
                        let mut guitar_y = settings.override_guitar_y.unwrap_or(0.0);
                        let last_guitar_y = guitar_y;
                        ui.add(Slider::new(&mut guitar_y, -3000.0..=3000.0).text("Guitar Y"));
                        if settings.override_guitar_y.is_none() || float_ne!(guitar_y, last_guitar_y, abs <= 1.0) {
                            settings.override_guitar_y = Some(guitar_y);
                            GuitarView::update_y(&mut guitar_view_query, guitar_y);
                        }
                    } else if settings.override_guitar_y.is_some() {
                        settings.override_guitar_y = None;
                        window_resized_evts.send(WindowResizedEvent());
                    }

                    let hide_bar_number = settings.hide_bar_number;
                    ui.checkbox(&mut settings.hide_bar_number, "Hide Bar Number");
                    if hide_bar_number != settings.hide_bar_number {
                        Self::reload_tab(&mut commands, &mut state, &viewer_query);
                    }
                    let always_show_fret = settings.always_show_fret;
                    ui.checkbox(&mut settings.always_show_fret, "Always Show Fret");
                    if always_show_fret != settings.always_show_fret {
                        Self::reload_tab(&mut commands, &mut state, &viewer_query);
                    }
                    ui.separator();
                    let mode_text = if settings.layout.mode == LayoutMode::Grid {
                        "Line Mode"
                    } else {
                        "Grid Mode"
                    };
                    if ui.button(mode_text).clicked() {
                        Self::toggle_layout_mode(&mut commands, &mut state, &mut settings, &viewer_query);
                    }
                    if settings.layout.mode == LayoutMode::Grid {
                        ui.checkbox(&mut settings.layout.try_show_last_row_in_grid_mode, "Try Show Last Row");
                    }
                    ui.separator();
                    if ui.button("Reload Tab").clicked() {
                        Self::reload_tab(&mut commands, &mut state, &viewer_query);
                    }
                    if tab_pathes.0.len() > 1 {
                        egui::ComboBox::from_label("")
                            .selected_text(state.tab_path.clone())
                            .show_ui(ui, |ui| {
                                for path in tab_pathes.0.iter() {
                                    if ui.selectable_label(*path == state.tab_path, path).clicked()
                                    {
                                        for (entity, _viewer) in viewer_query.iter() {
                                            commands.entity(entity).despawn_recursive();
                                        }
                                        state.change_tab(&asset_server, path.clone());
                                    }
                                }
                            });
                    }
                });
            });
    }
}
