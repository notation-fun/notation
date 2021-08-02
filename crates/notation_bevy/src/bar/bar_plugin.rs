use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::{
    AddEntryEvent, GridCol, GridRow, GuitarPlugin, LayerBundle, LyonShapeOp, LyricsPlugin,
    MelodyPlugin, NotationTheme, WindowResizedEvent,
};
use notation_model::prelude::{BarLayer, BarPosition, TabBar, TrackKind, Units};

use super::bar_beat::{BarBeat, BarBeatData};
use super::bar_separator::{BarSeparator, BarSeparatorData};

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create_layers.system());
        app.add_system(on_config_changed.system());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    mut query: Query<(&Arc<TabBar>, &GridRow, &GridCol, &mut Transform)>,
    sep_query: Query<(Entity, &BarSeparatorData)>,
    beat_query: Query<(Entity, &BarBeatData)>,
) {
    for _evt in evts.iter() {
        for (_bar, row, col, mut transform) in query.iter_mut() {
            *transform = theme.grid.calc_bar_transform(row, col);
        }
        for (entity, data) in sep_query.iter() {
            BarSeparator::update(&mut commands, &theme, entity, data);
        }
        for (entity, data) in beat_query.iter() {
            BarBeat::update(&mut commands, &theme, entity, data);
        }
    }
}

fn create_layers(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &Arc<TabBar>, &GridCol), (Added<Arc<TabBar>>, Without<Arc<BarLayer>>)>,
    mut add_entry_evts: EventWriter<AddEntryEvent>,
) {
    for (bar_entity, bar, grid_col) in query.iter() {
        for layer in &bar.bar.layers {
            if layer.rounds.is_some() {
                if layer
                    .rounds
                    .clone()
                    .unwrap()
                    .iter()
                    .find(|&x| *x == bar.section_round)
                    .is_none()
                {
                    continue;
                }
            }
            let layer_bundle = LayerBundle::new(bar.clone(), layer.clone());
            let mut layer_commands = commands.spawn_bundle(layer_bundle);
            BarPlugin::insert_layer_extra(&mut layer_commands, bar.clone(), layer.clone());
            let layer_entity = layer_commands.id();
            commands.entity(bar_entity).push_children(&[layer_entity]);
            for slice in &layer.slices {
                let mut pos = BarPosition::new(bar.bar_ordinal, Units(0.0));
                for entry in slice.entries.iter() {
                    let duration = entry.as_ref().duration();
                    add_entry_evts.send(AddEntryEvent(layer_entity, entry.clone(), pos));
                    pos.in_bar_pos = pos.in_bar_pos + Units::from(duration);
                }
            }
        }
        if grid_col.0 == 0 {
            BarSeparator::create(
                &mut commands,
                bar_entity,
                &theme,
                BarSeparatorData::new(bar, true),
            );
        }
        BarSeparator::create(
            &mut commands,
            bar_entity,
            &theme,
            BarSeparatorData::new(bar, false),
        );
        let signature = bar.signature();
        for beat in 0..signature.beats_per_bar {
            BarBeatData::may_new(&theme, bar, &signature, beat)
                .map(|data| BarBeat::create(&mut commands, bar_entity, &theme, data));
        }
    }
}

impl BarPlugin {
    pub fn insert_layer_extra(
        commands: &mut EntityCommands,
        _bar: Arc<TabBar>,
        layer: Arc<BarLayer>,
    ) {
        if let Some(track) = layer.track.clone() {
            commands.insert(track.clone());
            match track.kind {
                TrackKind::Guitar => GuitarPlugin::insert_guitar_layer_extra(commands, track),
                TrackKind::Vocal => MelodyPlugin::insert_melody_layer_extra(commands, track),
                TrackKind::Lyrics => LyricsPlugin::insert_lyrics_layer_extra(commands, track),
                _ => (),
            }
        }
    }
}
