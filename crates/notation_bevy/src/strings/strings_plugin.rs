use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use notation_model::prelude::{BarLane, BarPosition, LaneKind};
use std::sync::Arc;

use super::pick_bundle::PickBundle;
use super::pick_note::{PickNoteData, PickNoteShape};
use super::single_string::{SingleString, SingleStringData};
use super::strings_grid::StringsGrid;
use crate::prelude::{BarPlugin, LyonShapeOp, NotationTheme, ShapesPlugin, WindowResizedEvent};
use notation_model::prelude::{Fretboard, FrettedEntry, HandShape, TabBar};

pub struct StringsPlugin;

impl Plugin for StringsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_add_fretted_grid::<6>.system());
        app.add_system(on_add_fretted_grid::<4>.system());
        app.add_system(on_config_changed.system());
        app.add_system_set(super::pick_systems::new_system_set());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    string_query: Query<(Entity, &SingleStringData)>,
    pick_note_query: Query<(Entity, &PickNoteData)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in string_query.iter() {
            SingleString::update(&mut commands, &theme, entity, data);
        }
        for (entity, data) in pick_note_query.iter() {
            PickNoteShape::update(&mut commands, &theme, entity, data);
        }
    }
}

fn on_add_fretted_grid<const S: usize>(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &Arc<TabBar>, &StringsGrid<S>), Added<StringsGrid<S>>>,
) {
    for (entity, tab_bar, strings_grid) in query.iter() {
        strings_grid.add_strings(&mut commands, &theme, entity, tab_bar);
    }
}

impl StringsPlugin {
    pub fn insert_entry_extra<const S: usize>(
        commands: &mut EntityCommands,
        entry: &FrettedEntry<S>,
    ) {
        match entry {
            FrettedEntry::Pick(pick, _duration) => {
                commands.insert_bundle(PickBundle::from(*pick));
            }
            _ => (),
        }
    }
    pub fn get_fretted_shape<const S: usize>(
        entry_entity: Entity,
        position: &BarPosition,
        lane_queries: (&Query<&Parent>, &Query<&Children>, &Query<&Arc<BarLane>>),
        shape_queries: (
            &Query<(&Arc<TabBar>, &Arc<BarLane>, &Fretboard<S>, &Children)>,
            &Query<&HandShape<S>>,
        ),
    ) -> Option<(Arc<TabBar>, Fretboard<S>, HandShape<S>)> {
        if let Some((shapes_lane_entity, _shapes_lane)) =
            BarPlugin::get_lane(entry_entity, 2, LaneKind::Shapes, lane_queries)
        {
            ShapesPlugin::get_fretted_shape::<S>(shapes_lane_entity, position, shape_queries)
        } else {
            None
        }
    }
}
