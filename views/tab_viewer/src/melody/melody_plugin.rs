use edger_bevy::bevy::ecs::system::EntityCommands;
use edger_bevy::bevy_prelude::*;
use edger_bevy::prelude::SingleData;

use crate::notation::assets::NotationAssetsStates;
use crate::prelude::MelodyGrid;
use crate::settings::notation_settings::NotationSettings;
use crate::theme::notation_theme::NotationTheme;
use notation_model::prelude::BarLane;

pub struct MelodyPlugin;

impl Plugin for MelodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, MelodyPlugin::on_add_melody_grid
            .run_if(in_state(NotationAssetsStates::Loaded))
        );

    }
}

impl MelodyPlugin {
    pub fn on_add_melody_grid(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        settings: Res<NotationSettings>,
        query: Query<(Entity, &SingleData<BarLane>, &MelodyGrid), Added<MelodyGrid>>,
    ) {
        if theme._bypass_systems {
            return;
        }
        for (entity, lane, grid) in query.iter() {
            grid.add_lines(&mut commands, &theme, &settings, entity, &lane.0);
        }
    }
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(MelodyGrid::default());
    }
}
