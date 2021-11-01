use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy::utils::Uuid;
use bevy_egui::egui::{self, Slider};
use bevy_egui::EguiContext;
use bevy_utils::prelude::{
    BevyUtil, DockPanel, DockSide, LayoutAnchor, LayoutConstraint, LayoutSize, View, ViewBundle,
};
use float_eq::float_ne;
use notation_midi::prelude::{MidiState, PlayControlEvent};
use notation_model::play::play_control::TickResult;
use notation_model::prelude::{Tab, Units};

use crate::settings::layout_settings::LayoutMode;
use crate::ui::layout::NotationLayout;

use crate::prelude::{
    NotationAppState, NotationAssets, NotationSettings, NotationTheme, TabPathes,
    WindowResizedEvent,
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
    pub const WIDTH: f32 = 200.0;
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
    pub fn set_begin_bar_ordinal(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        midi_state.play_control.begin_bar_ordinal = midi_state.play_control.position.bar.bar_ordinal;
        Self::send_begin_end_evt(midi_state, play_control_evts);
    }
    pub fn set_end_bar_ordinal(
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
    ) {
        midi_state.play_control.end_bar_ordinal = midi_state.play_control.position.bar.bar_ordinal;
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
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
        mut window_resized_evts: EventWriter<WindowResizedEvent>,
    ) {
        if state.hide_control {
            return;
        }
        egui::SidePanel::right("control")
            .min_width(Self::WIDTH)
            .max_width(Self::WIDTH)
            .show(egui_ctx.ctx(), |ui| {
                ui.vertical(|ui| {
                    if ui.button("Hide Control\n(Press Tab to Show)").clicked() {
                        state.hide_control = true;
                        window_resized_evts.send(WindowResizedEvent());
                    }
                    ui.checkbox(&mut settings.mouse_dragged_panning, "Panning");
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
                    ui.checkbox(&mut midi_state.play_control.should_loop, "Loop");
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
                            &mut &mut midi_state,
                            &mut play_control_evts,
                        )
                    }
                    let always_show_fret = settings.always_show_fret;
                    ui.checkbox(&mut settings.always_show_fret, "Always Show Fret");
                    if always_show_fret != settings.always_show_fret {
                        Self::reload_tab(&mut commands, &mut state, &viewer_query);
                    }
                    let mode_text = if settings.layout.mode == LayoutMode::Grid {
                        "Line Mode"
                    } else {
                        "Grid Mode"
                    };
                    if ui.button(mode_text).clicked() {
                        Self::toggle_layout_mode(&mut commands, &mut state, &mut settings, &viewer_query);
                    }
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
