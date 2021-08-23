use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::{Chord, PlayingState, ModelEntryProps};

use crate::prelude::{ModelEntryData, LyonShape, LyonShapeOp, NotationTheme};

use super::chord_base::{ChordBase, ChordBaseData, ChordBaseValue};
use super::chord_interval::{ChordInterval, ChordIntervalData, ChordIntervalValue};

#[derive(Clone, Debug)]
pub struct ChordDiagramValue {
    pub radius: f32,
    pub chord: Chord,
    pub playing_state: PlayingState,
}
impl Display for ChordDiagramValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
            .chords
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
            .chords
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
                    .chords
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
        interval_query: &mut Query<(Entity, &mut ChordIntervalData)>,
        base_query: &mut Query<(Entity, &mut ChordBaseData)>,
        entity: Entity,
        data: &mut ChordDiagramData,
        children: &Children,
        radius: f32,
    ) {
        data.value.radius = radius;
        ChordDiagram::update(commands, theme, entity, data);
        for child in children.iter() {
            if let Ok((interval_entity, mut interval_data)) = interval_query.get_mut(*child) {
                interval_data.value.radius = radius / 3.0;
                ChordInterval::update(commands, theme, interval_entity, &interval_data)
            } else if let Ok((base_entity, mut base_data)) = base_query.get_mut(*child) {
                base_data.value.interval.radius = radius / 4.0;
                ChordBase::update(commands, theme, base_entity, &base_data)
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
            let interval_value = ChordIntervalValue {
                total: intervals.len(),
                index: index,
                radius: radius * theme.sizes.chords.diagram_interval_factor,
                root: chord.root,
                interval: interval.clone(),
            };
            let interval_data = ChordIntervalData::from((entry_props, interval_value));
            let interval_entity =
                ChordInterval::create(commands, theme, diagram_entity, interval_data);
            commands
                .entity(diagram_entity)
                .push_children(&[interval_entity]);
        }
        if let Some(base) = chord.base {
            let interval_value = ChordIntervalValue {
                total: intervals.len(),
                index: 0,
                radius: radius * theme.sizes.chords.diagram_base_factor,
                root: chord.root,
                interval: base.clone(),
            };
            let base_data = ChordBaseData::from((entry_props, ChordBaseValue {
                interval: interval_value,
            }));
            let base_entity = ChordBase::create(commands, theme, diagram_entity, base_data);
            commands
                .entity(diagram_entity)
                .push_children(&[base_entity]);
        }
    }
}
