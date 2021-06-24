use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_core::prelude::{Syllable, Semitones, Octave, Duration, Units};

use crate::{grid::{self, grid_config::GridConfig}, prelude::Theme};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
    .with_system(create_note_block.system())
    .with_system(update_note_transform.system())
}

pub fn calc_transform(
        grid_config: &Res<GridConfig>,
        syllable: &Syllable, position: &Units) -> Transform {
    let semitones = Semitones::from(*syllable);
    let x = grid_config.unit_size * position.0;
    let y = grid_config.semitone_size * semitones.0 as f32;
    Transform::from_xyz(x, y, 0.0)
}

fn create_note_block(mut commands: Commands,
        grid_config: Res<GridConfig>,
        theme: Res<Theme>,
        query: Query<(Entity, &Syllable, &Octave, &Duration, &Units), Added<Syllable>>,
) {
    for (entity, syllable, octave, duration, units) in query.iter() {
        let shape = shapes::Rectangle {
            width: grid_config.unit_size * Units::from(*duration).0,
            height: grid_config.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        };
        let fill_color = theme.syllable.from_syllable_octave(*syllable, *octave);
        commands.entity(entity).insert_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(fill_color, theme.outline_color),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(grid_config.note_outline),
            },
            calc_transform(&grid_config, syllable, units),
        ));
    }
}

fn update_note_transform(mut _commands: Commands,
        grid_config: Res<GridConfig>,
        mut query: Query<(&Syllable, &Units, &mut Transform),
                        Or<(Changed<Syllable>, Changed<Units>)>>,
) {
    for (syllable, units, mut transform) in query.iter_mut() {
        println!("update_note_transform: {:?}, {:?}, {:?}", syllable, units, transform);
        *transform = calc_transform(&grid_config, syllable, units);
    }
}
