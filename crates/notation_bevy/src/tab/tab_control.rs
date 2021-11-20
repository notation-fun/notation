use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, ColorBackground, DockPanel, DockSide, DockView, LayoutConstraint, LayoutQuery, LayoutSize, View, ViewBundle, ViewQuery};
use notation_model::prelude::Tab;

use crate::play::play_panel::PlayPanel;
use crate::prelude::{NotationAppState, NotationAssets, NotationSettings, NotationTheme};
use crate::rhythm::rhythm_view::RhythmView;
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
    pub fn calc_height(engine: &NotationLayout, constraint: LayoutConstraint) -> f32 {
        if constraint.max.width < engine.theme.sizes.tab_control.control_width {
            engine.theme.sizes.tab_control.control_height * constraint.max.width
                / engine.theme.sizes.tab_control.control_width
        } else {
            engine.theme.sizes.tab_control.control_height
        }
    }
}
impl<'a> View<NotationLayout<'a>> for TabControl {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        if constraint.max.width <= engine.theme.sizes.tab_control.dock_top_width {
            let height = Self::calc_height(engine, constraint);
            LayoutSize::new(constraint.max.width, height)
        } else {
            let width = engine.theme.sizes.tab_control.control_width;
            LayoutSize::new(width, constraint.max.height)
        }
    }
    fn log_set_layout(&self) -> bool {
        true
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for TabControl {
    fn dock_side(&self, engine: &NotationLayout<'a>, size: LayoutSize) -> DockSide {
        if size.width <= engine.theme.sizes.tab_control.dock_top_width {
            DockSide::Top
        } else {
            DockSide::Left
        }
    }
}
impl<'a> DockView<NotationLayout<'a>, RhythmView, PlayPanel> for TabControl {}

impl TabControl {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let control = TabControl::new(tab.clone());
        let control_entity =
            BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(control));
        ColorBackground::spawn(
            commands,
            control_entity,
            theme.core.mini_map_z,
            theme.colors.ui.control_background,
        );
        RhythmView::spawn(
            commands,
            assets,
            theme,
            control_entity,
            tab,
        );
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
        panel_query: ViewQuery<RhythmView>,
        content_query: ViewQuery<PlayPanel>,
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
