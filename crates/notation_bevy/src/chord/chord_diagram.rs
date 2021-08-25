use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::{Chord, ModelEntryProps, PlayingState};

use crate::prelude::{LyonShape, LyonShapeOp, ModelEntryData, NotationTheme};

use super::chord_base::{ChordBase, ChordBaseData};
use super::chord_interval::{ChordInterval, ChordIntervalData};
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

pub struct ChordDiagram<'a> {
    theme: &'a NotationTheme,
    data: ChordDiagramData,
}

impl<'a> LyonShape<shapes::Circle> for ChordDiagram<'a> {
    fn get_name(&self) -> String {
        format!("{}", self.data)
    }
    fn get_shape(&self) -> shapes::Circle {
        let outline = self
            .theme
            .sizes
            .chord
            .diagram_outline
            .of_state(&self.data.value.playing_state);
        let mut radius = self.data.value.radius;
        if self.data.value.playing_state.is_current() {
            radius += outline;
        } else {
            radius -= outline;
        }

        shapes::Circle {
            center: Vec2::ZERO,
            radius,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let fill = self.theme.colors.of_syllable(self.data.value.chord.root);
        let outline = self
            .theme
            .colors
            .chord
            .diagram_outline
            .of_state(&self.data.value.playing_state);
        ShapeColors::outlined(fill, outline)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(
                self.theme
                    .sizes
                    .chord
                    .diagram_outline
                    .of_state(&self.data.value.playing_state),
            ),
        }
    }
    fn get_transform(&self) -> Transform {
        Transform::from_xyz(0.0, 0.0, 2.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, ChordDiagramData, shapes::Circle, ChordDiagram<'a>>
    for ChordDiagram<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: ChordDiagramData) -> ChordDiagram<'a> {
        ChordDiagram::<'a> { theme, data }
    }
}

impl<'a> ChordDiagram<'a> {
    pub fn update_size(
        commands: &mut Commands,
        theme: &NotationTheme,
        interval_query: &mut Query<(Entity, &mut ChordIntervalData, &Children)>,
        base_query: &mut Query<(Entity, &mut ChordBaseData, &Children)>,
        dot_query: &mut Query<(Entity, &mut IntervalDotData)>,
        entity: Entity,
        data: &mut ChordDiagramData,
        children: &Children,
        radius: f32,
    ) {
        data.value.radius = radius;
        ChordDiagram::update(commands, theme, entity, data);
        for child in children.iter() {
            if let Ok((interval_entity, mut interval_data, interval_children)) =
                interval_query.get_mut(*child)
            {
                ChordInterval::update_size(
                    commands,
                    theme,
                    dot_query,
                    interval_entity,
                    &mut interval_data,
                    interval_children,
                    radius,
                );
            } else if let Ok((base_entity, mut base_data, base_chidren)) =
                base_query.get_mut(*child)
            {
                ChordBase::update_size(
                    commands,
                    theme,
                    dot_query,
                    base_entity,
                    &mut base_data,
                    base_chidren,
                    radius,
                );
            }
        }
    }
    pub fn update_playing_state(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        data: &mut ChordDiagramData,
        playing_state: PlayingState,
    ) {
        data.value.playing_state = playing_state;
        ChordDiagram::update(commands, theme, entity, data);
    }
    pub fn spawn(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        entry_props: ModelEntryProps,
        chord: Chord,
        radius: f32,
    ) {
        let chord_value = ChordDiagramValue {
            chord,
            radius,
            playing_state: PlayingState::Idle,
        };
        let chord_data = ChordDiagramData {
            entry_props,
            value: chord_value,
        };
        let diagram_entity = ChordDiagram::create(commands, theme, entity, chord_data);
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
            ChordInterval::spawn(commands, theme, diagram_entity, interval_data);
        }
        if let Some(base) = chord.base {
            let base_data = ChordBaseData::new_data(entry_props, chord.root, base.clone(), radius);
            ChordBase::spawn(commands, theme, diagram_entity, base_data);
        }
    }
}
