use std::{fmt::Display, sync::Arc};

use bevy::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, GridData, GridView, LayoutAnchor, LayoutQuery, LayoutSize, View, ViewBundle, ViewQuery, DockPanel, DockSide, LayoutConstraint, ColorBackground};
use notation_midi::prelude::PlayControlEvent;
use notation_model::prelude::{PlayState, Tab};

use crate::{prelude::{NotationAppState, NotationAssets, NotationSettings, NotationTheme}, ui::layout::NotationLayout};

use super::{play_button::{PlayButton, PlayButtonShape}, play_plugin::PlayPanelDoLayoutEvent};

pub struct PlayPanel {
    pub playing: bool,
    pub should_loop: bool,
}

impl Display for PlayPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PlayPanel>({})", self.should_loop)
    }
}

impl Default for PlayPanel {
    fn default() -> Self {
        Self { playing: false, should_loop: true }
    }
}

impl<'a> View<NotationLayout<'a>> for PlayPanel {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let width = constraint.max.width;
        if width >= engine.theme.sizes.tab_control.button_size_range.0 * 6.0 {
            let mut height = width / 6.0;
            if height > engine.theme.sizes.tab_control.button_size_range.1 {
                height = engine.theme.sizes.tab_control.button_size_range.1;
            }
            LayoutSize::new(width, height)
        } else {
            LayoutSize::new(width, width / 3.0 * 2.0)
        }
    }
}

impl<'a> DockPanel<NotationLayout<'a>> for PlayPanel {
    fn dock_side(&self, _engine: &NotationLayout<'a>, _size: LayoutSize) -> DockSide {
        DockSide::Bottom
    }
}

impl<'a> GridView<NotationLayout<'a>, PlayButton> for PlayPanel {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        let one_row = grid_size.width >= engine.theme.sizes.tab_control.button_size_range.0 * 6.0;
        let button_size = if one_row {
            grid_size.height
        } else {
            grid_size.width / 3.0
        };
        GridData::new_fixed(
            if one_row { 1 } else { 2 },
            if one_row { 6 } else { 3 },
            LayoutSize::new(button_size, button_size),
            LayoutSize::ZERO,
            LayoutAnchor::TOP_LEFT,
            grid_size,
        )
    }
}

impl PlayPanel {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let panel = PlayPanel::default();
        let panel_entity =
            BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(panel));
        ColorBackground::spawn(
            commands,
            panel_entity,
            theme.z.play_panel,
            theme.colors.ui.control_background,
        );
        for i in 0..=5 {
            PlayButton::spawn(commands, assets, theme, settings, panel_entity, tab, (i as usize).into());
        }
        panel_entity
    }
    pub fn do_layout(
        mut evts: EventReader<PlayPanelDoLayoutEvent>,
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<PlayButton>,
    ) {
        if theme._bypass_systems { return; }
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &mut commands,
                &engine,
                &mut layout_query,
                &cell_query,
                evt.entity,
                evt.layout,
            )
        }
    }
    pub fn on_play_control_evt(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        mut evts: EventReader<PlayControlEvent>,
        mut shape_query: Query<(Entity, &mut PlayButtonShape)>,
    ) {
        if theme._bypass_systems { return; }
        for evt in evts.iter() {
            match evt {
                PlayControlEvent::OnTick {position: _, tick_result} => {
                    if tick_result.stopped {
                        let play_state = PlayState::Stopped;
                        PlayButton::on_play_state(&mut commands, &theme, &mut shape_query, &play_state);
                    }
                }
                PlayControlEvent::OnPlayState(play_state) => {
                    PlayButton::on_play_state(&mut commands, &theme, &mut shape_query, play_state);
                }
                PlayControlEvent::OnShouldLoop(should_loop) => {
                    PlayButton::on_should_loop(&mut commands, &theme, &mut shape_query, *should_loop);
                }
                PlayControlEvent::OnBeginEnd(begin_bar_ordinal, end_bar_ordinal) => {
                    PlayButton::on_begin_end(&mut commands, &theme, &mut shape_query, *begin_bar_ordinal, *end_bar_ordinal)
                }
                PlayControlEvent::OnSpeedFactor(_) => {}
            }
        }
    }
}
