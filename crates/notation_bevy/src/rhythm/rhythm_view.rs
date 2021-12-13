use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use notation_bevy_utils::prelude::{BevyUtil, DockPanel, DockSide, LayoutConstraint, LayoutSize, View, ViewBundle};
use notation_model::prelude::Tab;

use crate::prelude::{NotationAssets, NotationTheme};
use crate::tab::tab_events::RhythmViewDoLayoutEvent;
use crate::ui::layout::NotationLayout;

use super::rhythm_bar::{RhythmBarData};
use super::rhythm_beat::RhythmBeatData;
use super::rhythm_indicator::RhythmIndicatorData;

pub struct RhythmView{}

impl Display for RhythmView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RhythmView>()")
    }
}

impl<'a> View<NotationLayout<'a>> for RhythmView {
    fn calc_size(&self, _engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let height = constraint.max.height;
        LayoutSize::new(height, height)
    }
}

impl<'a> DockPanel<NotationLayout<'a>> for RhythmView {
    fn dock_side(&self, _engine: &NotationLayout<'a>, _size: LayoutSize) -> DockSide {
        DockSide::Left
    }
}

impl RhythmView {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let view_entity =
            BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(RhythmView{}));
        RhythmBarData::spawn(
            commands,
            assets,
            theme,
            view_entity,
            tab,
        );
        view_entity
    }
    pub fn do_layout(
        mut evts: EventReader<RhythmViewDoLayoutEvent>,
        mut commands: Commands,
        theme: Res<NotationTheme>,
        mut bar_query: Query<(&Parent, Entity, &mut RhythmBarData, &Children), With<RhythmBarData>>,
        mut beat_query: Query<(Entity, &mut RhythmBeatData)>,
        mut indicator_query: Query<(Entity, &mut RhythmIndicatorData)>,
    ) {
        if !theme.loaded { return; }
        for evt in evts.iter() {
            if evt.layout.size.width > 0.0 && evt.layout.size.height > 0.0 {
                for (parent, bar_entity, mut bar_data, bar_children) in bar_query.iter_mut() {
                    if parent.0 == evt.entity {
                        let layout = evt.layout;
                        let height = layout.size.height;
                        let radius = height * theme.sizes.tab_control.rhythm_bar_radius_factor
                            + theme.sizes.tab_control.rhythm_bar_radius_extra;
                        bar_data.update_size(
                            &mut commands,
                            &theme,
                            &mut beat_query,
                            &mut indicator_query,
                            bar_entity,
                            bar_children,
                            radius,
                            Vec2::new(height / 2.0, -layout.size.height / 2.0),
                        )
                    }
                }
            }
        }
    }
}
