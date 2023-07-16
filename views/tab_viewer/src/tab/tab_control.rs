use std::fmt::Display;
use std::sync::Arc;

use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::prelude::{
    entity, DockPanel, DockSide, DockView, LayoutConstraint, LayoutQuery, LayoutSize, View,
    ViewBundle, ViewQuery,
};
use notation_model::prelude::Tab;

use crate::chord::chord_color_background::ChordColorBackground;
use crate::play::play_panel::PlayPanel;
use crate::prelude::{
    GuitarView, NotationState, NotationAssets, NotationSettings, NotationTheme,
};
use crate::prelude::NotationLayout;

use super::tab_events::TabControlDoLayoutEvent;

#[derive(Clone, Debug, Component)]
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
    pub fn calc_width(max_width: f32, theme: &NotationTheme) -> f32 {
        let mut width =
            max_width * theme.sizes.tab_control.control_width_factor;
        if width < theme.sizes.tab_control.tab_control_range.0 {
            width = theme.sizes.tab_control.tab_control_range.0;
        } else if width > theme.sizes.tab_control.tab_control_range.1 {
            width = theme.sizes.tab_control.tab_control_range.1;
        }
        width
    }
}
impl<'a> View<NotationLayout<'a>> for TabControl {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        if engine.settings.hide_guitar_view {
            return LayoutSize::new(0.0, constraint.max.height);
        }
        let width = match engine.settings.override_guitar_width {
            Some(width) => width,
            None => {
                Self::calc_width(constraint.max.width, engine.theme)
            }
        };
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
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let control = TabControl::new(tab.clone());
        let control_entity =
            entity::spawn_child_bundle(commands, entity, ViewBundle::from(control));
        if settings.hide_guitar_view {
            return control_entity;
        }
        ChordColorBackground::spawn(
            commands,
            control_entity,
            theme.z.tab_control,
            theme
                .colors
                .of_syllable(tab.meta.scale.calc_root_syllable()),
        );
        GuitarView::spawn(commands, assets, theme, control_entity, tab);
        PlayPanel::spawn(commands, assets, theme, settings, control_entity, tab);
        control_entity
    }
    pub fn do_layout(
        mut evts: EventReader<TabControlDoLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<PlayPanel>,
        content_query: ViewQuery<GuitarView>,
    ) {
        if theme._bypass_systems {
            return;
        }
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
