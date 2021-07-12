use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::Signature;
use std::sync::Arc;

use crate::config::bevy_config::BevyConfig;
use crate::config::grid_config::{GridCol, GridRow};
use crate::prelude::{AddEntryEvent, ConfigChangedEvent, GuitarPlugin, LayerBundle, LyonShapeOp};
use notation_model::prelude::{BarLayer, TabBar, TrackKind, Units};

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
    mut evts: EventReader<ConfigChangedEvent>,
    config: Res<BevyConfig>,
    mut query: Query<(&Arc<TabBar>, &GridRow, &GridCol, &mut Transform)>,
    sep_query: Query<(Entity, &BarSeparatorData)>,
    beat_query: Query<(Entity, &BarBeatData)>,
) {
    for _evt in evts.iter() {
        for (bar, row, col, mut transform) in query.iter_mut() {
            *transform = config.grid.calc_bar_transform(bar.bar_units(), &row, &col);
        }
        for (entity, data) in sep_query.iter() {
            BarSeparator::update(&mut commands, &config, entity, data);
        }
        for (entity, data) in beat_query.iter() {
            BarBeat::update(&mut commands, &config, entity, data);
        }
    }
}

fn create_layers(
    mut commands: Commands,
    config: Res<BevyConfig>,
    query: Query<(Entity, &Arc<TabBar>, &GridCol), (Added<Arc<TabBar>>, Without<Arc<BarLayer>>)>,
    mut add_entry_evts: EventWriter<AddEntryEvent>,
) {
    for (bar_entity, bar, grid_col) in query.iter() {
        for layer in &bar.bar.layers {
            let layer_undle = LayerBundle::new(&bar, layer.clone());
            let mut layer_commands = commands.spawn_bundle(layer_undle);
            BarPlugin::insert_layer_extra(&mut layer_commands, bar.clone(), layer.clone());
            let layer_entity = layer_commands.id();
            commands.entity(bar_entity).push_children(&[layer_entity]);
            for slice in &layer.slices {
                let mut position = Units(0.0);
                for index in slice.index..slice.index + slice.count {
                    if let Some(entry) = slice.line.entries.get(index) {
                        let duration = entry.as_ref().duration();
                        add_entry_evts.send(AddEntryEvent(layer_entity, entry.clone(), position));
                        position = position + Units::from(duration);
                    }
                }
            }
        }
        let top = 15.0; //TODO: calc from layers
        let bottom = -120.0; //TODO: calc from layers
        if grid_col.0 == 0 {
            BarSeparator::create(
                &mut commands,
                bar_entity,
                &config,
                BarSeparatorData::new(&bar, top, bottom, true),
            );
        }
        BarSeparator::create(
            &mut commands,
            bar_entity,
            &config,
            BarSeparatorData::new(&bar, top, bottom, false),
        );
        let signature = bar.signature();
        for beat in 0..signature.beats_per_bar {
            BarBeatData::may_new(&config, &bar, &signature, top, bottom, beat)
                .map(|data| BarBeat::create(&mut commands, bar_entity, &config, data));
        }
    }
}

impl BarPlugin {
    pub fn insert_layer_extra(
        commands: &mut EntityCommands,
        bar: Arc<TabBar>,
        layer: Arc<BarLayer>,
    ) {
        if let Some(track) = layer.track.clone() {
            match track.kind {
                TrackKind::Guitar => {
                    GuitarPlugin::insert_guitar_layer_extra(commands, bar, layer, track)
                }
                _ => (),
            }
        }
    }
    pub fn add_beat_block(
        commands: &mut Commands,
        config: &BevyConfig,
        tab_bar: &Arc<TabBar>,
        bar_entity: Entity,
        top: f32,
        bottom: f32,
        signature: &Signature,
        beat: u8,
    ) -> () {
        if let Some(color) = config.theme.core.get_beat_color(signature, beat) {
            let beat_units = Units::from(signature.beat_unit);
            let shape = shapes::Rectangle {
                width: config.grid.unit_size * beat_units.0,
                height: (top - bottom),
                origin: shapes::RectangleOrigin::BottomLeft,
            };
            let x = config.grid.unit_size * beat_units.0 * beat as f32;
            let bar_ordinal = tab_bar.bar_ordinal;
            let name = format!("{}:{}", bar_ordinal, beat);
            let beat_entity = commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    ShapeColors::new(color),
                    DrawMode::Fill(FillOptions::default()),
                    Transform::from_xyz(x, bottom, config.theme.core.beat_z),
                ))
                .insert(Name::from(name.as_str()))
                .id();
            commands.entity(bar_entity).push_children(&[beat_entity]);
        }
    }
}
