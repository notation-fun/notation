use std::fmt::Display;
use bevy::prelude::*;

use notation_model::prelude::Tab;

use crate::prelude::{BevyUtil, LyonShapeOp, NotationAppState, NotationSettings, NotationTheme, SingleBundle, WindowResizedEvent};

use super::mini_bar::{MiniBarData, MiniBarShape};
use super::mini_beats::{MiniBeats, MiniBeatsData, MiniBeatsValue};
use super::mini_map::{MiniMap, MiniMapBack, MiniMapBackData};

pub struct MiniPlugin;

impl Plugin for MiniPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    app_state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut query: Query<(&MiniMap, &mut Transform)>,
    mut mini_map_back_query: Query<(Entity, &mut MiniMapBackData)>,
    mut mini_bar_query: Query<(Entity, &mut MiniBarData)>,
) {
    for _evt in evts.iter() {
        for (minimap, mut transform) in query.iter_mut() {
            let data_value = settings.layout.calc_mini_bar_value(&app_state, minimap.bars);
            let (new_transform, new_back_data) = theme.grid.calc_mini_map_transform(&app_state, &data_value);
            *transform = new_transform;
            if let Ok((back_entity, mut back_data)) = mini_map_back_query.single_mut() {
                *back_data = new_back_data;
                MiniMapBack::update(&mut commands, &theme, back_entity, &back_data);
            }
            for (entity, mut data) in mini_bar_query.iter_mut() {
                data.value = data_value.clone();
                MiniBarShape::update(&mut commands, &theme, entity, &data);
            }
        }
    }
}

impl MiniPlugin {
    pub fn spawn_mini_map(
        commands: &mut Commands,
        app_state: &NotationAppState,
        settings: &NotationSettings,
        theme: &NotationTheme,
        tab_entity: Entity,
        tab: &Tab,
    ) -> Entity {
        let bars = tab.bars.len();
        let data_value = settings.layout.calc_mini_bar_value(app_state, bars);
        let (transform, back_data) = theme.grid.calc_mini_map_transform(app_state, &data_value);
        let map_entity = BevyUtil::spawn_child_bundle(commands, tab_entity,
            SingleBundle::from((MiniMap { bars }, transform)));
        MiniMapBack::create(commands, map_entity, theme, back_data);
        for bar in tab.bars.iter() {
            let data = MiniBarData::new(bar, data_value.clone());
            let mini_bar_entity = MiniBarShape::create(commands, map_entity, theme, data);
            let beats_value = MiniBeatsValue {
                size: data_value.size / 3.0,
                offset: data_value.size / 2.0,
                syllable: notation_model::prelude::Syllable::Fa,
            };
            let beats_data = MiniBeatsData::new(bar, beats_value);
            MiniBeats::create(commands, mini_bar_entity, theme, beats_data);
        }
        map_entity
    }
}