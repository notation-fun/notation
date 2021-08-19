use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::{Chord, TabBar};

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

use super::chord_base::{ChordBase, ChordBaseData, ChordBaseValue};
use super::chord_interval::{ChordInterval, ChordIntervalData, ChordIntervalValue};

#[derive(Clone, Debug)]
pub struct ChordValue {
    pub size: f32,
    pub chord: Chord,
}

pub type ChordData = BarData<ChordValue>;

pub struct ChordDiagram<'a> {
    theme: &'a NotationTheme,
    data: ChordData,
}

impl<'a> LyonShape<shapes::Circle> for ChordDiagram<'a> {
    fn get_name(&self) -> String {
        format!("{}: {:?}", self.data.bar_props.bar_ordinal, self.data.value)
    }
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: self.data.value.size,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = self
            .theme
            .colors
            .color_of_syllable(self.data.value.chord.root);
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }

    fn get_transform(&self) -> Transform {
        Transform::from_xyz(0.0, 0.0, 2.0)
    }
}

impl<'a> LyonShapeOp<'a, ChordData, shapes::Circle, ChordDiagram<'a>> for ChordDiagram<'a> {
    fn new_shape(theme: &'a NotationTheme, data: ChordData) -> ChordDiagram<'a> {
        ChordDiagram::<'a> { theme, data }
    }
}

impl<'a> ChordDiagram<'a> {
    pub fn spawn(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        bar: &TabBar,
        chord: Chord,
        size: f32,
    ) -> Entity {
        let chord_value = ChordValue { size, chord };
        let chord_data = ChordData::new(bar, chord_value);
        let diagram_entity = ChordDiagram::create(commands, entity, theme, chord_data);
        let intervals = chord.intervals.get_intervals();
        for (index, interval) in intervals.iter().enumerate() {
            let interval_value = ChordIntervalValue {
                total: intervals.len(),
                index: index,
                size: size / 3.0,
                root: chord.root,
                interval: interval.clone(),
            };
            let interval_data = ChordIntervalData::new(bar, interval_value);
            let interval_entity =
                ChordInterval::create(commands, diagram_entity, theme, interval_data);
            commands
                .entity(diagram_entity)
                .push_children(&[interval_entity]);
        }
        if let Some(base) = chord.base {
            let interval_value = ChordIntervalValue {
                total: intervals.len(),
                index: 0,
                size: size / 4.0,
                root: chord.root,
                interval: base.clone(),
            };
            let base_data = ChordBaseData::new(bar, ChordBaseValue {
                interval: interval_value,
            });
            let base_entity = ChordBase::create(commands, diagram_entity, theme, base_data);
            commands
                .entity(diagram_entity)
                .push_children(&[base_entity]);
        }
        diagram_entity
    }
    pub fn update_size(
        commands: &mut Commands,
        theme: &NotationTheme,
        chord_query: &mut Query<(Entity, &mut ChordData, &Children)>,
        interval_query: &mut Query<(Entity, &mut ChordIntervalData)>,
        base_query: &mut Query<(Entity, &mut ChordBaseData)>,
        chord_entity: Entity,
        size: f32,
    ) {
        if let Ok((_, mut chord_data, chord_children)) = chord_query.get_mut(chord_entity) {
            chord_data.value.size = size;
            ChordDiagram::update(commands, theme, chord_entity, &chord_data);
            for child in chord_children.iter() {
                if let Ok((interval_entity, mut interval_data)) = interval_query.get_mut(*child) {
                    interval_data.value.size = size / 3.0;
                    ChordInterval::update(commands, theme, interval_entity, &interval_data)
                } else if let Ok((base_entity, mut base_data)) = base_query.get_mut(*child) {
                    base_data.value.interval.size = size / 4.0;
                    ChordBase::update(commands, theme, base_entity, &base_data)
                }
            }
        }
    }
}
