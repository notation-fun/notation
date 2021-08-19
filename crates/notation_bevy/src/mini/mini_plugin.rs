use bevy::prelude::*;

use notation_model::prelude::{PlayingState, Tab};

use crate::chord::chord_base::ChordBaseData;
use crate::chord::chord_diagram::{ChordData, ChordDiagram};
use crate::chord::chord_interval::ChordIntervalData;
use crate::prelude::{
    BarPlaying, BevyUtil, LyonShapeOp, NotationAppState, NotationSettings, NotationTheme,
    SingleBundle, WindowResizedEvent,
};

use super::mini_bar::{MiniBarData, MiniBarShape};
use super::mini_map::{MiniMap, MiniMapBack, MiniMapBackData};
use super::mini_section_separator::{
    MiniSectionSeparator, MiniSectionSeparatorData, MiniSectionSeparatorValue,
};

pub struct MiniPlugin;

impl Plugin for MiniPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
        app.add_system(on_bar_playing_changed.system());
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
    mut mini_bar_query: Query<(Entity, &mut MiniBarData, &Children)>,
    mut mini_section_separator_query: Query<(Entity, &mut MiniSectionSeparatorData)>,
    mut chord_query: Query<(Entity, &mut ChordData, &Children)>,
    mut interval_query: Query<(Entity, &mut ChordIntervalData)>,
    mut base_query: Query<(Entity, &mut ChordBaseData)>,
) {
    for _evt in evts.iter() {
        for (minimap, mut transform) in query.iter_mut() {
            let data_value = settings
                .layout
                .calc_mini_bar_value(&app_state, minimap.bars);
            let (new_transform, new_back_data) =
                theme.grid.calc_mini_map_transform(&app_state, &data_value);
            *transform = new_transform;
            if let Ok((back_entity, mut back_data)) = mini_map_back_query.single_mut() {
                *back_data = new_back_data;
                MiniMapBack::update(&mut commands, &theme, back_entity, &back_data);
            }
            for (entity, mut data, bar_children) in mini_bar_query.iter_mut() {
                data.value = data_value.clone();
                MiniBarShape::update(&mut commands, &theme, entity, &data);
                for chord_entity in bar_children.iter() {
                    let chord_size = data_value.size * settings.layout.mini_beats_factor;
                    ChordDiagram::update_size(
                        &mut commands,
                        &theme,
                        &mut chord_query,
                        &mut interval_query,
                        &mut base_query,
                        *chord_entity,
                        chord_size,
                    );
                }
            }
            for (entity, mut data) in mini_section_separator_query.iter_mut() {
                data.value.bar = data_value.clone();
                MiniSectionSeparator::update(&mut commands, &theme, entity, &data);
            }
        }
    }
}

fn on_bar_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut query: Query<(Entity, &BarPlaying, &mut MiniBarData), Changed<BarPlaying>>,
) {
    for (entity, playing, mut data) in query.iter_mut() {
        //println!("{:?} -> {:?} -> {:?}", name, data, playing)
        data.value.playing_state = playing.value;
        MiniBarShape::update(&mut commands, &theme, entity, &data);
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
        let map_entity = BevyUtil::spawn_child_bundle(
            commands,
            tab_entity,
            SingleBundle::from((MiniMap { bars }, transform)),
        );
        MiniMapBack::create(commands, map_entity, theme, back_data);
        for bar in tab.bars.iter() {
            let data = MiniBarData::new(bar, data_value.clone());
            let mini_bar_entity = MiniBarShape::create(commands, map_entity, theme, data);
            if bar.props.bar_index == 0 {
                let section_separator_data = MiniSectionSeparatorData::new(
                    bar,
                    MiniSectionSeparatorValue::new(data_value.clone()),
                );
                MiniSectionSeparator::create(commands, map_entity, theme, section_separator_data);
            }

            if let Some(chord) = bar.get_chord(None) {
                let chord_size = data_value.size * settings.layout.mini_beats_factor;
                ChordDiagram::spawn(commands, theme, mini_bar_entity, bar, chord, chord_size);
                commands
                    .entity(mini_bar_entity)
                    .insert(BarPlaying::new(bar, PlayingState::Idle));
            }
        }
        map_entity
    }
}
