use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use notation_bevy_utils::prelude::SingleData;

use crate::notation::assets::NotationAssetsStates;
use crate::prelude::HarmonyGrid;
use crate::settings::notation_settings::NotationSettings;
use crate::theme::notation_theme::NotationTheme;
use notation_model::prelude::BarLane;

pub struct HarmonyPlugin;

impl Plugin for HarmonyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(HarmonyPlugin::on_add_harmony_grid)
        );
    }
}

impl HarmonyPlugin {
    pub fn on_add_harmony_grid(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        settings: Res<NotationSettings>,
        query: Query<(Entity, &SingleData<BarLane>, &HarmonyGrid), Added<HarmonyGrid>>,
    ) {
        if theme._bypass_systems {
            return;
        }
        for (entity, lane, grid) in query.iter() {
            grid.add_lines(&mut commands, &theme, &settings, entity, &lane.0);
        }
    }
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(HarmonyGrid::default());
    }
}
