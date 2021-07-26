use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use notation_model::prelude::BarPosition;
use std::sync::Arc;

use super::fretted_grid::FrettedGrid;
use super::fretted_string::{FrettedString, FrettedStringData};
use super::hand_bundles::HandShapeBundle;
use super::pick_bundle::PickBundle;
use super::pick_note::{PickNoteData, PickNoteShape};
use crate::prelude::{BevyConfig, ConfigChangedEvent, LyonShapeOp};
use notation_model::prelude::{Fretboard, FrettedEntry, HandShape, TabBar};

pub struct FrettedPlugin;

impl Plugin for FrettedPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_add_fretted_grid::<6>.system());
        app.add_system(on_add_fretted_grid::<4>.system());
        app.add_system(on_config_changed.system());
        app.add_system_set(crate::fretted::hand_systems::new_system_set());
        app.add_system_set(crate::fretted::pick_systems::new_system_set());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<ConfigChangedEvent>,
    config: Res<BevyConfig>,
    string_query: Query<(Entity, &FrettedStringData)>,
    pick_note_query: Query<(Entity, &PickNoteData)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in string_query.iter() {
            FrettedString::update(&mut commands, &config, entity, data);
        }
        for (entity, data) in pick_note_query.iter() {
            PickNoteShape::update(&mut commands, &config, entity, data);
        }
    }
}

fn on_add_fretted_grid<const S: usize>(
    mut commands: Commands,
    config: Res<BevyConfig>,
    query: Query<(&Parent, Entity, &FrettedGrid<S>), Added<FrettedGrid<S>>>,
    parent_query: Query<&Arc<TabBar>>,
) {
    for (parent, entity, fretted_grid) in query.iter() {
        if let Ok(tab_bar) = parent_query.get(parent.0) {
            fretted_grid.add_strings(&mut commands, &config, entity, tab_bar);
        }
    }
}

impl FrettedPlugin {
    pub fn insert_fretted_entry_extra<const S: usize>(
        commands: &mut EntityCommands,
        entry: &FrettedEntry<S>,
    ) {
        match entry {
            FrettedEntry::Fretboard(_) => (),
            FrettedEntry::Shape(shape, _) => {
                commands.insert_bundle(HandShapeBundle::<S>::from(*shape));
            }
            FrettedEntry::Pick(pick, _duration) => {
                commands.insert_bundle(PickBundle::from(*pick));
            }
            FrettedEntry::Strum(_, _) => (),
        }
    }
    pub fn get_fretted_shape<const S: usize>(
        layer_query: &Query<(&Arc<TabBar>, &Fretboard<S>, &Children)>,
        shape_query: &Query<&HandShape<S>>,
        layer_entity: Entity,
        _position: &BarPosition,
    ) -> Option<(Arc<TabBar>, Fretboard<S>, HandShape<S>)> {
        if let Ok((bar, fretboard, children)) = layer_query.get(layer_entity) {
            let mut matched_shape = None;
            for &child in children.iter() {
                if let Ok(shape) = shape_query.get(child) {
                    //TODO: check shape duration aganst position
                    matched_shape = Some(shape);
                    break;
                }
            }
            if let Some(shape) = matched_shape {
                return Some((bar.clone(), *fretboard, *shape));
            }
        }
        None
    }
}
