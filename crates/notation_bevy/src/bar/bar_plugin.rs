use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::{
    AddEntryEvent, BarLayout, BevyUtil, LaneBundle, LaneLayout, LyonShapeOp, NotationAppState,
    NotationSettings, NotationTheme, WindowResizedEvent,
};
use notation_model::prelude::{BarLane, BarPosition, TabBar};

use super::bar_beat::{BarBeat, BarBeatData, BarBeatValue};
use super::bar_separator::{BarSeparator, BarSeparatorData};

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    mut query: Query<(&Arc<TabBar>, &BarLayout, &mut Transform)>,
    sep_query: Query<(Entity, &BarSeparatorData)>,
    beat_query: Query<(Entity, &BarBeatData)>,
) {
    for _evt in evts.iter() {
        for (_bar, layout, mut transform) in query.iter_mut() {
            *transform = theme.grid.calc_bar_transform(layout);
        }
        for (entity, data) in sep_query.iter() {
            BarSeparator::update(&mut commands, &theme, entity, data);
        }
        for (entity, data) in beat_query.iter() {
            BarBeat::update(&mut commands, &theme, entity, data);
        }
    }
}

impl BarPlugin {
    fn create_lane(
        commands: &mut Commands,
        _app_state: &NotationAppState,
        _settings: &NotationSettings,
        _theme: &NotationTheme,
        bar_entity: Entity,
        bar: &Arc<TabBar>,
        _bar_layout: &BarLayout,
        add_entry_evts: &mut EventWriter<AddEntryEvent>,
        lane: &Arc<BarLane>,
        lane_layout: &LaneLayout,
    ) {
        let lane_bundle = LaneBundle::new(lane.clone(), *lane_layout);
        let lane_entity = BevyUtil::spawn_child_bundle(commands, bar_entity, lane_bundle);
        for entry in lane.entries.iter() {
            add_entry_evts.send(AddEntryEvent(
                lane_entity,
                entry.clone(),
                BarPosition::new(
                    bar.bar_units(),
                    bar.props.bar_ordinal,
                    entry.props.in_bar_pos,
                ),
            ));
        }
    }
    pub fn create_lanes(
        commands: &mut Commands,
        app_state: &NotationAppState,
        settings: &NotationSettings,
        theme: &NotationTheme,
        bar_entity: Entity,
        bar: Arc<TabBar>,
        bar_layout: &BarLayout,
        add_entry_evts: &mut EventWriter<AddEntryEvent>,
    ) {
        for lane in bar.lanes.iter() {
            if let Some(lane_layout) = bar_layout.lane_layouts.get(&lane.id()) {
                Self::create_lane(
                    commands,
                    app_state,
                    settings,
                    theme,
                    bar_entity,
                    &bar,
                    bar_layout,
                    add_entry_evts,
                    lane,
                    lane_layout,
                );
            }
        }
        if bar_layout.data.col == 0 {
            BarSeparator::create(
                commands,
                bar_entity,
                &theme,
                BarSeparatorData::new(&bar, bar_layout, true),
            );
        }
        BarSeparator::create(
            commands,
            bar_entity,
            &theme,
            BarSeparatorData::new(&bar, bar_layout, false),
        );
        let signature = bar.signature();
        for beat in 0..signature.bar_beats {
            BarBeatValue::may_new(&theme, &bar, &signature, bar_layout, beat)
                .map(|value| BarBeatData::new(&bar, value))
                .map(|data| BarBeat::create(commands, bar_entity, theme, data));
        }
    }
}
