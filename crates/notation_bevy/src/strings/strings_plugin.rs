use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use std::sync::Arc;

use super::pick_bundle::PickBundle;

use super::strings_grid::{StringsGrid4, StringsGrid6};
use crate::prelude::{NotationAssets, NotationAssetsStates, NotationSettings, NotationTheme};
use notation_model::prelude::{BarLane, FrettedEntry4, FrettedEntry6, LaneEntry, TrackKind};

pub struct StringsPlugin;

impl Plugin for StringsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(on_add_fretted_grid6.system())
                .with_system(on_add_fretted_grid4.system())
                .with_system(super::pick_systems::on_entry_playing_changed.system()),
        );
    }
}

impl StringsPlugin {
    pub fn insert_lane_extra(commands: &mut EntityCommands, lane: &BarLane) {
        match lane.track.kind {
            TrackKind::Guitar => Self::insert_lane_extra6(commands, lane),
            _ => (),
        }
    }
}

macro_rules! impl_strings_plugin {
    ($on_add_fretted_grid:ident,
        $insert_lane_extra:ident, $insert_entry_extra:ident, $create_pick_notes:ident,
        $fretted_entry:ident, $strings_grid:ident
    ) => {
        fn $on_add_fretted_grid(
            mut commands: Commands,
            theme: Res<NotationTheme>,
            query: Query<(Entity, &Arc<BarLane>, &$strings_grid), Added<$strings_grid>>,
        ) {
            for (entity, lane, strings_grid) in query.iter() {
                strings_grid.add_strings(&mut commands, &theme, entity, lane);
            }
        }

        impl StringsPlugin {
            pub fn $insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
                commands.insert($strings_grid::default());
            }
            pub fn $insert_entry_extra(
                commands: &mut Commands,
                assets: &NotationAssets,
                theme: &NotationTheme,
                settings: &NotationSettings,
                entity: Entity,
                entry: &LaneEntry,
                fretted_entry: &$fretted_entry,
            ) {
                match fretted_entry {
                    $fretted_entry::Pick(pick, _duration) => {
                        commands
                            .entity(entity)
                            .insert_bundle(PickBundle::from(*pick));
                        super::pick_systems::$create_pick_notes(
                            commands, assets, theme, settings, entity, entry, pick,
                        );
                    }
                    _ => (),
                }
            }
        }
    };
}

impl_strings_plugin!(
    on_add_fretted_grid6,
    insert_lane_extra6,
    insert_entry_extra6,
    create_pick_notes6,
    FrettedEntry6,
    StringsGrid6
);
impl_strings_plugin!(
    on_add_fretted_grid4,
    insert_lane_extra4,
    insert_entry_extra4,
    create_pick_notes4,
    FrettedEntry4,
    StringsGrid4
);
