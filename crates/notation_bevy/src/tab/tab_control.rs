use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, DockPanel, DockSide, DockView, LayoutConstraint, LayoutQuery, LayoutSize, View, ViewBundle, ViewQuery};
use notation_model::prelude::{Tab};

use crate::chord::chord_color_background::ChordColorBackground;
use crate::play::play_panel::PlayPanel;
use crate::prelude::{NotationAppState, NotationAssets, NotationSettings, NotationTheme, GuitarView};
use crate::ui::layout::NotationLayout;

use super::tab_events::TabControlDoLayoutEvent;

pub struct TabControl {
    pub tab: Arc<Tab>,
}
impl Display for TabControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabControl>({})", self.tab.bars.len())
    }
}
impl TabControl {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}
impl<'a> View<NotationLayout<'a>> for TabControl {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let mut width = constraint.max.width * engine.theme.sizes.tab_control.control_width_factor;
        if width > engine.theme.sizes.tab_control.max_control_width {
            width = engine.theme.sizes.tab_control.max_control_width;
        }
        LayoutSize::new(width, constraint.max.height)
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for TabControl {
    fn dock_side(&self, _engine: &NotationLayout<'a>, _size: LayoutSize) -> DockSide {
        DockSide::Left
    }
}
impl<'a> DockView<NotationLayout<'a>, PlayPanel, GuitarView> for TabControl {}

impl TabControl {
    pub fn spawn(
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let control = TabControl::new(tab.clone());
        let control_entity =
            BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(control));
        ChordColorBackground::spawn(
            commands,
            control_entity,
            theme.core.mini_map_z,
            theme.colors.of_syllable(tab.meta.scale.calc_root_syllable()),
        );
        GuitarView::spawn(commands, materials, assets, theme, control_entity, tab);
        PlayPanel::spawn(
            commands,
            assets,
            theme,
            settings,
            control_entity,
            tab,
        );
        control_entity
    }
    pub fn do_layout(
        mut evts: EventReader<TabControlDoLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<PlayPanel>,
        content_query: ViewQuery<GuitarView>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &engine,
                &mut layout_query,
                &panel_query,
                &content_query,
                evt.entity,
                evt.layout,
            );
        }
    }
}
