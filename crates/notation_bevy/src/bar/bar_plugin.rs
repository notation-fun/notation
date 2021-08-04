use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::{AddEntryEvent, BarLayout, GuitarPlugin, LaneBundle, LyonShapeOp, LyricsPlugin, MelodyPlugin, NotationTheme, WindowResizedEvent};
use notation_model::prelude::{BarLane, BarLayer, BarPosition, LaneKind, TabBar, TrackKind, Units};

use super::bar_beat::{BarBeat, BarBeatData};
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
    pub fn get_lane(
        entity: Entity,
        depth: usize,
        lane_kind: LaneKind,
        lane_queries: (&Query<&Parent>, &Query<&Children>, &Query<&Arc<BarLane>>),
    ) -> Option<(Entity, Arc<BarLane>)> {
        let mut current_entity = entity;
        for i in 0..depth {
            if let Ok(parent) = lane_queries.0.get(current_entity) {
                current_entity = parent.0;
            } else {
                println!("BarPlugin::get_lane({:?}, {}, {}) Parent Not Found: {}", entity, depth, lane_kind, i);
                return None;
            }
        }
        if let Ok(children) = lane_queries.1.get(current_entity) {
            if children.len() == 0 {
                println!("BarPlugin::get_lane({:?}, {}, {}) Children Is Empty: {:?}", entity, depth, lane_kind, current_entity);
            }
            for &child in children.iter() {
                if let Ok(lane) = lane_queries.2.get(child) {
                    if lane.kind == lane_kind {
                        //println!("BarPlugin::get_lane({:?}, {}, {}) Found: {}", entity, depth, lane_kind, lane);
                        return Some((child, lane.clone()))
                    } else {
                        println!("BarPlugin::get_lane({:?}, {}, {}) BarLane Not Matched: {}", entity, depth, lane_kind, lane);
                    }
                } else {
                    println!("BarPlugin::get_lane({:?}, {}, {}) BarLane Not Found: {:?}", entity, depth, lane_kind, child);
                }
            }
        } else {
            println!("BarPlugin::get_lane({:?}, {}, {}) Children Not Found: {:?}", entity, depth, lane_kind, current_entity);
        }
        None
    }
    fn insert_lane_extra(
        commands: &mut EntityCommands,
        _bar: Arc<TabBar>,
        lane: Arc<BarLane>,
    ) {
        commands.insert(lane.slice.track.clone());
        let track = lane.slice.track.clone();
        match lane.kind {
            LaneKind::Lyrics => LyricsPlugin::insert_lyrics_lane_extra(commands, track),
            LaneKind::Melody => MelodyPlugin::insert_melody_lane_extra(commands, track),
            LaneKind::Strings => {
                if track.kind == TrackKind::Guitar {
                    GuitarPlugin::insert_guitar_strings_lane_extra(commands, track)
                }
            }
            LaneKind::Shapes => {
                if track.kind == TrackKind::Guitar {
                    GuitarPlugin::insert_guitar_shapes_lane_extra(commands, track)
                }
            }
            _ => (),
        }
    }
    fn create_lane(
        commands: &mut Commands,
        _theme: &NotationTheme,
        bar_entity: Entity,
        bar: &Arc<TabBar>,
        _bar_layout: &BarLayout,
        add_entry_evts: &mut EventWriter<AddEntryEvent>,
        lane: &Arc<BarLane>,
    ) {
        if lane.slice.rounds.is_some()
            && lane.slice.rounds.clone().unwrap()
                .iter()
                .find(|&x| *x == bar.section_round)
                .is_none() {
            return;
        }
        let layer_bundle = LaneBundle::new(bar.clone(), lane.clone());
        let mut layer_commands = commands.spawn_bundle(layer_bundle);
        BarPlugin::insert_lane_extra(&mut layer_commands, bar.clone(), lane.clone());
        let layer_entity = layer_commands.id();
        commands.entity(bar_entity).push_children(&[layer_entity]);
        let mut pos = BarPosition::new(bar.bar_ordinal, Units(0.0));
        for entry in lane.slice.entries.iter() {
            let duration = entry.as_ref().value.duration();
            add_entry_evts.send(AddEntryEvent(layer_entity, entry.clone(), pos));
            pos.in_bar_pos = pos.in_bar_pos + Units::from(duration);
        }
    }
    pub fn create_lanes(
        commands: &mut Commands,
        theme: &NotationTheme,
        bar_entity: Entity,
        bar: Arc<TabBar>,
        bar_layout: &BarLayout,
        add_entry_evts: &mut EventWriter<AddEntryEvent>,
    ) {
        for lane in &bar.bar.lanes {
            Self::create_lane(commands, theme, bar_entity, &bar, bar_layout, add_entry_evts, lane);
        }
        if bar_layout.col == 0 {
            BarSeparator::create(
                commands,
                bar_entity,
                &theme,
                BarSeparatorData::new(&bar, true),
            );
        }
        BarSeparator::create(
            commands,
            bar_entity,
            &theme,
            BarSeparatorData::new(&bar, false),
        );
        let signature = bar.signature();
        for beat in 0..signature.beats_per_bar {
            BarBeatData::may_new(&theme, &bar, &signature, beat)
                .map(|data| BarBeat::create(commands, bar_entity, &theme, data));
        }
    }
}

