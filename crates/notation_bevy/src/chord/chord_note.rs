use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::{Interval, Syllable};

use crate::prelude::{LyonShape, LyonShapeOp, ModelEntryData, NotationTheme};

use super::interval_dot::{IntervalDot, IntervalDotData};

pub trait ChordNoteExtra: Send + Sync + Clone {
    fn offset(&self, theme: &NotationTheme) -> Vec2;
    fn radius(&self, theme: &NotationTheme) -> f32;
    fn set_diagram_radius(&mut self, diagram_radius: f32);
}

#[derive(Clone, Debug)]
pub struct ChordNoteValue<T: ChordNoteExtra + 'static> {
    pub root: Syllable,
    pub interval: Interval,
    pub extra: T,
}
impl<T: ChordNoteExtra + 'static> Display for ChordNoteValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<ChordNoteValue<{}>>({} {})",
            std::any::type_name::<T>(),
            self.root,
            self.interval
        )
    }
}
impl<T: ChordNoteExtra + 'static> ChordNoteValue<T> {
    pub fn new(root: Syllable, interval: Interval, extra: T) -> Self {
        Self {
            root,
            interval,
            extra,
        }
    }
    pub fn calc_syllable(&self) -> Syllable {
        Syllable::from((self.root, self.interval))
    }
}

pub type ChordNoteData<T> = ModelEntryData<ChordNoteValue<T>>;

pub struct ChordNote<'a, T: ChordNoteExtra + 'static> {
    pub theme: &'a NotationTheme,
    pub data: ChordNoteData<T>,
}

impl<'a, T: ChordNoteExtra + 'static> LyonShape<shapes::Circle> for ChordNote<'a, T> {
    fn get_name(&self) -> String {
        format!("{}", self.data)
    }
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: self.data.value.extra.radius(self.theme),
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = self
            .theme
            .colors
            .of_syllable(self.data.value.calc_syllable());
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        let offset = self.data.value.extra.offset(self.theme);
        Transform::from_xyz(offset.x, offset.y, 1.0)
    }
}

impl<'a, T: ChordNoteExtra + 'static>
    LyonShapeOp<'a, NotationTheme, ChordNoteData<T>, shapes::Circle, ChordNote<'a, T>>
    for ChordNote<'a, T>
{
    fn new_shape(theme: &'a NotationTheme, data: ChordNoteData<T>) -> ChordNote<'a, T> {
        ChordNote::<'a, T> { theme, data }
    }
}

impl<'a, T: ChordNoteExtra + 'static> ChordNote<'a, T> {
    pub fn update_size(
        commands: &mut Commands,
        theme: &NotationTheme,
        dot_query: &mut Query<(Entity, &mut IntervalDotData)>,
        entity: Entity,
        data: &mut ChordNoteData<T>,
        children: &Children,
        diagram_radius: f32,
    ) {
        data.value.extra.set_diagram_radius(diagram_radius);
        let note_radius = data.value.extra.radius(theme);
        ChordNote::<T>::update(commands, theme, entity, data);
        for child in children.iter() {
            if let Ok((dot_entity, mut dot_data)) = dot_query.get_mut(*child) {
                dot_data.note_radius = note_radius;
                IntervalDot::update(commands, theme, dot_entity, &dot_data)
            }
        }
    }
    pub fn spawn(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        data: ChordNoteData<T>,
    ) -> Entity {
        let quality = data.value.interval.into();
        let dot_count = data.value.interval.dot_count();
        let note_radius = data.value.extra.radius(theme);
        let note_entity = ChordNote::create(commands, theme, entity, data);
        for index in 0..dot_count {
            let dot_data = IntervalDotData::new(quality, dot_count, index, note_radius);
            IntervalDot::create(commands, theme, note_entity, dot_data);
        }
        note_entity
    }
}
