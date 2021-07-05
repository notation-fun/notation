use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::{AddEntryEvent, GuitarPlugin, LayerBundle};
use notation_proto::prelude::{BarLayer, TabBar, TrackKind, Units};

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create_layers.system());
    }
}

fn create_layers(
    mut commands: Commands,
    query: Query<(Entity, &Arc<TabBar>), (Added<Arc<TabBar>>, Without<Arc<BarLayer>>)>,
    mut add_entry_evts: EventWriter<AddEntryEvent>,
) {
    for (bar_entity, bar) in query.iter() {
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
    }
}

impl BarPlugin {
    pub fn insert_layer_extra(
        commands: &mut bevy::ecs::system::EntityCommands,
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
}
