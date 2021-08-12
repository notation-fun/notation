use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use notation_model::prelude::{BarLane, BarPosition, LaneKind};
use std::sync::Arc;

use super::pick_bundle::PickBundle;
use super::pick_note::{PickNoteData, PickNoteShape};
use super::single_string::{SingleString, SingleStringData};
use super::strings_grid::StringsGrid6;
use super::strings_grid::StringsGrid4;
use crate::prelude::{BarPlugin, LyonShapeOp, NotationTheme, ShapesPlugin, WindowResizedEvent};
use notation_model::prelude::{Fretboard6, FrettedEntry6, HandShape6, Fretboard4, FrettedEntry4, HandShape4, TabBar};

pub struct StringsPlugin;

impl Plugin for StringsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_add_fretted_grid6.system());
        app.add_system(on_add_fretted_grid4.system());
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

macro_rules! impl_strings_plugin {
    ($on_add_fretted_grid:ident, $insert_entry_extra:ident,
        $fretted_entry:ident, $strings_grid:ident
    ) => {
        fn $on_add_fretted_grid(
            mut commands: Commands,
            theme: Res<NotationTheme>,
            query: Query<(Entity, &Arc<TabBar>, &$strings_grid), Added<$strings_grid>>,
        ) {
            for (entity, tab_bar, strings_grid) in query.iter() {
                strings_grid.add_strings(&mut commands, &theme, entity, tab_bar);
            }
        }

        impl StringsPlugin {
            pub fn $insert_entry_extra(
                commands: &mut EntityCommands,
                entry: &$fretted_entry,
            ) {
                match entry {
                    $fretted_entry::Pick(pick, _duration) => {
                        commands.insert_bundle(PickBundle::from(*pick));
                    }
                    _ => (),
                }
            }
        }
    }
}

impl_strings_plugin!(on_add_fretted_grid6, insert_entry_extra6, FrettedEntry6, StringsGrid6);
impl_strings_plugin!(on_add_fretted_grid4, insert_entry_extra4, FrettedEntry4, StringsGrid4);
