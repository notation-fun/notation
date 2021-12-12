use std::fmt::Display;

use bevy::prelude::*;

use notation_bevy_utils::prelude::{OutlineCircle, ShapeOp};
use notation_model::prelude::{Chord, ModelEntryProps, PlayingState};

use crate::prelude::{ModelEntryData, NotationTheme};

use super::chord_base::{ChordBaseData};
use super::chord_interval::{ChordIntervalData};
use super::interval_dot::IntervalDotData;

#[derive(Clone, Debug)]
pub struct ChordDiagramValue {
    pub radius: f32,
    pub chord: Chord,
    pub playing_state: PlayingState,
}
impl Display for ChordDiagramValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ChordDiagramValue>({})", self.chord)
    }
}
pub type ChordDiagramData = ModelEntryData<ChordDiagramValue>;

impl ShapeOp<NotationTheme, OutlineCircle> for ChordDiagramData {
    fn get_shape(&self, theme: &NotationTheme) -> OutlineCircle {
        let outline_width = theme
            .sizes
            .chord
            .diagram_outline
            .of_state(&self.value.playing_state);
        let mut radius = self.value.radius;
        if self.value.playing_state.is_current() {
            radius += outline_width;
        } else {
            radius -= outline_width;
        }
        let color = theme.colors.of_syllable(self.value.chord.root);
        let outline_color = theme
            .colors
            .chord
            .diagram_outline
            .of_state(&self.value.playing_state);
        OutlineCircle {
            radius,
            color,
            outline_width,
            outline_color,
            offset: Vec3::new(0.0, 0.0, theme.z.mini_bar),
        }
    }
}

impl ChordDiagramData {
    pub fn update_size(
        &mut self,
        commands: &mut Commands,
        theme: &NotationTheme,
        interval_query: &mut Query<(Entity, &mut ChordIntervalData, &Children)>,
        base_query: &mut Query<(Entity, &mut ChordBaseData, &Children)>,
        dot_query: &mut Query<(Entity, &mut IntervalDotData)>,
        entity: Entity,
        children: &Children,
        radius: f32,
    ) {
        self.value.radius = radius;
        self.update(commands, theme, entity);
        for child in children.iter() {
            if let Ok((interval_entity, mut interval_data, interval_children)) =
                interval_query.get_mut(*child)
            {
                interval_data.update_size(
                    commands,
                    theme,
                    dot_query,
                    interval_entity,
                    interval_children,
                    radius,
                );
            } else if let Ok((base_entity, mut base_data, base_chidren)) =
                base_query.get_mut(*child)
            {
                base_data.update_size(
                    commands,
                    theme,
                    dot_query,
                    base_entity,
                    base_chidren,
                    radius,
                );
            }
        }
    }
    pub fn update_playing_state(
        &mut self,
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        playing_state: PlayingState,
    ) {
        self.value.playing_state = playing_state;
        self.update(commands, theme, entity);
    }
    pub fn spawn(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        entry_props: ModelEntryProps,
        chord: Chord,
        radius: f32,
    ) -> Entity {
        let chord_value = ChordDiagramValue {
            chord,
            radius,
            playing_state: PlayingState::Idle,
        };
        let chord_data = ChordDiagramData {
            entry_props,
            value: chord_value,
        };
        let diagram_entity = chord_data.create(commands, theme, entity);
        let intervals = chord.intervals.get_intervals();
        for (index, interval) in intervals.iter().enumerate() {
            let interval_data = ChordIntervalData::new_data(
                entry_props,
                chord.root,
                interval.clone(),
                intervals.len(),
                index,
                radius,
            );
            interval_data.spawn(commands, theme, diagram_entity);
        }
        if let Some(base) = chord.base {
            let base_data = ChordBaseData::new_data(entry_props, chord.root, base.clone(), radius);
            base_data.spawn(commands, theme, diagram_entity);
        }
        diagram_entity
    }
}
