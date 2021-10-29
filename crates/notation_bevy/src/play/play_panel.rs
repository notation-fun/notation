use std::fmt::Display;

use bevy::prelude::*;
use bevy_utils::prelude::{BevyUtil, GridData, GridView, LayoutAnchor, LayoutQuery, LayoutSize, View, ViewBundle, ViewQuery};

use crate::{prelude::{NotationAppState, NotationAssets, NotationSettings, NotationTheme}, ui::layout::NotationLayout};

use super::{play_button::{PlayButton}, play_plugin::PlayPanelDoLayoutEvent};

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
    fn log_set_layout(&self) -> bool {
        true
    }
}

impl<'a> GridView<NotationLayout<'a>, PlayButton> for PlayPanel {
    fn calc_grid_data(&self, _engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        let size = LayoutSize::new(grid_size.width / 3.0, grid_size.height / 2.0);
        GridData::new_fixed(
            2,
            3,
            size,
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
        entity: Entity,
    ) -> Entity {
        let panel = PlayPanel::default();
        let panel_entity =
            BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(panel));
        for i in 0..=5 {
            PlayButton::spawn(commands, assets, theme, panel_entity, (i as usize).into());
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
}
