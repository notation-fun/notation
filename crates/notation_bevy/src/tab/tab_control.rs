use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{BevyUtil, ColorBackground, DockPanel, DockSide, LayoutChangedQuery, LayoutConstraint, LayoutSize, View, ViewBundle};
use notation_model::prelude::Tab;

use crate::{prelude::{NotationAssets, NotationTheme}, rhythm::{rhythm_bar::{RhythmBar, RhythmBarData}, rhythm_beat::RhythmBeatData}, ui::layout::NotationLayout};

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
            engine.theme.sizes.tab_control.control_height * constraint.max.width / engine.theme.sizes.tab_control.control_width
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

impl TabControl {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let control = TabControl::new(tab.clone());
        let control_entity = BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(control));
        ColorBackground::spawn(
            commands,
            control_entity,
            theme.core.mini_map_z,
            theme.colors.chord.background,
        );
        let bar_props = tab.get_bar_of_ordinal(1).map(|x| x.props).unwrap_or_default();
        let chord = tab.get_bar_of_ordinal(1).and_then(|x| x.get_chord(None));
        RhythmBar::spawn(commands, assets, theme, control_entity, bar_props, tab.signature(), chord);
        control_entity
    }
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedQuery<TabControl>,
        mut bar_query: Query<(&Parent, Entity, &mut RhythmBarData, &Children), With<RhythmBarData>>,
        mut beat_query: Query<(Entity, &mut RhythmBeatData)>,
    ) {
        for (entity, _view, layout) in query.iter() {
            if layout.size.width <= 0.0 || layout.size.height <= 0.0 {
                return;
            }
            for (parent, bar_entity, mut bar_data, bar_children) in bar_query.iter_mut() {
                if parent.0 == entity {
                    let ratio = theme.sizes.tab_control.control_width / theme.sizes.tab_control.control_height;
                    let tall_mode = layout.size.width / layout.size.height < ratio;
                    let height = if tall_mode {
                        layout.size.width / ratio
                    } else {
                        layout.size.height
                    };
                    let radius = height * theme.sizes.tab_control.rhythm_bar_radius_factor + theme.sizes.tab_control.rhythm_bar_radius_extra;
                    RhythmBar::update_size(&mut commands, &theme, &mut beat_query, bar_entity, &mut bar_data, bar_children, radius, Vec2::new(height / 2.0, - layout.size.height / 2.0))
                }
            }
        }
    }
}
