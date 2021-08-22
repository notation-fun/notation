use bevy::prelude::*;

use bevy_utils::prelude::{LayoutConstraint, ViewBundle};
use notation_model::prelude::{PlayingState, Syllable, Tab};

use crate::prelude::{
    BarPlaying, BevyUtil, LyonShapeOp, NotationAppState, NotationSettings, NotationTheme,
};
use crate::ui::layout::NotationLayout;

use super::mini_bar::{MiniBarData, MiniBarShape, MiniBarValue};
use super::mini_map::{MiniMap, MiniMapBack, MiniMapBackData};
use super::mini_section_separator::{
    MiniSectionSeparator, MiniSectionSeparatorData, MiniSectionSeparatorValue,
};

pub struct MiniPlugin;

impl Plugin for MiniPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(MiniMap::on_layout_changed.system());
        app.add_system(on_bar_playing_changed.system());
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
        theme: &NotationTheme,
        state: &NotationAppState,
        settings: &NotationSettings,
        tab_entity: Entity,
        tab: &Tab,
    ) -> Entity {
        let engine = NotationLayout::new(&theme, &state, &settings);
        let bars = tab.bars.len();
        let minimap = MiniMap::new(bars);
        let constraint = LayoutConstraint::from((state.window_width, state.window_height));
        let layout = minimap.calc_mini_bar_layout(&engine, constraint);
        let back_data = MiniMapBackData::default();
        let map_entity =
            BevyUtil::spawn_child_bundle(commands, tab_entity, ViewBundle::from(minimap));
        MiniMapBack::create(commands, theme, map_entity, back_data);
        for bar in tab.bars.iter() {
            let syllable = bar.get_chord(None).map(|x| x.root).unwrap_or(Syllable::Do);
            let data = MiniBarData::new(bar, MiniBarValue::new(layout.clone(), syllable));
            let mini_bar_entity = MiniBarShape::create(commands, theme, map_entity, data);
            if bar.props.bar_index == 0 {
                let section_separator_data = MiniSectionSeparatorData::new(
                    bar,
                    MiniSectionSeparatorValue::new(layout.clone()),
                );
                MiniSectionSeparator::create(commands, theme, map_entity, section_separator_data);
            }
            commands
                .entity(mini_bar_entity)
                .insert(BarPlaying::new(bar, PlayingState::Idle));
        }
        map_entity
    }
}
