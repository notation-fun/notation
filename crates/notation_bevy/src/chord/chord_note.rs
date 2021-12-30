use std::fmt::Display;

use bevy::prelude::*;

use notation_bevy_utils::prelude::{FillCircle, ShapeOp};
use notation_model::prelude::{Interval, Syllable};

use crate::prelude::{ModelEntryData, NotationTheme};

use super::interval_dot::IntervalDotData;

pub trait ChordNoteExtra: Send + Sync + Clone {
    fn offset(&self, theme: &NotationTheme) -> Vec2;
    fn radius(&self, theme: &NotationTheme) -> f32;
    fn set_diagram_radius(&mut self, diagram_radius: f32);
    #[allow(unused_variables)]
    fn get_color(&self, theme: &NotationTheme, color: Color) -> Color {
        color
    }
    #[allow(unused_variables)]
    fn get_z(&self, theme: &NotationTheme) -> f32 {
        1.0
    }
    fn show_dots(&self) -> bool {
        true
    }
    /* not sure whether want to add this here
    fn show_syllable(&self) -> bool {
        true
    }
     */
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

impl<T: ChordNoteExtra + 'static> ShapeOp<NotationTheme, FillCircle> for ChordNoteData<T> {
    fn get_shape(&self, theme: &NotationTheme) -> FillCircle {
        let color = theme.colors.of_syllable(self.value.calc_syllable());
        let color = self.value.extra.get_color(theme, color);
        let offset = self.value.extra.offset(theme);
        FillCircle {
            radius: self.value.extra.radius(theme),
            color,
            offset: Vec3::new(offset.x, offset.y, self.value.extra.get_z(theme)),
        }
    }
}

impl<T: ChordNoteExtra + 'static> ChordNoteData<T> {
    pub fn update_size(
        &mut self,
        commands: &mut Commands,
        theme: &NotationTheme,
        dot_query: &mut Query<(Entity, &mut IntervalDotData)>,
        entity: Entity,
        children: &Children,
        diagram_radius: f32,
    ) {
        self.value.extra.set_diagram_radius(diagram_radius);
        let note_radius = self.value.extra.radius(theme);
        self.update(commands, theme, entity);
        for child in children.iter() {
            if let Ok((dot_entity, mut dot_data)) = dot_query.get_mut(*child) {
                dot_data.note_radius = note_radius;
                dot_data.update(commands, theme, dot_entity)
            }
        }
    }
    pub fn spawn(&self, commands: &mut Commands, theme: &NotationTheme, entity: Entity) -> Entity {
        let note_entity = self.create(commands, theme, entity);
        self.respawn_dots(commands, theme, None, note_entity);
        note_entity
    }
    pub fn respawn_dots(
        &self,
        commands: &mut Commands,
        theme: &NotationTheme,
        dot_query: Option<&Query<&Children>>,
        note_entity: Entity,
    ) {
        if let Some(dot_query) = dot_query {
            for children in dot_query.get(note_entity) {
                for child in children.iter() {
                    commands.entity(*child).despawn();
                }
            }
        }
        if self.value.extra.show_dots() {
            let quality = self.value.interval.into();
            let dot_count = self.value.interval.dot_count();
            let note_radius = self.value.extra.radius(theme);
            for index in 0..dot_count {
                let dot_data = IntervalDotData::new(quality, dot_count, index, note_radius);
                dot_data.create(commands, theme, note_entity);
            }
        }
    }
}
