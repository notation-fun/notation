use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::{Duration, Octave, Semitones, Syllable, Units};

use crate::prelude::NotationTheme;

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_note_block.system())
        .with_system(update_note_transform.system())
}

pub fn calc_transform(
    theme: &Res<NotationTheme>,
    syllable: &Syllable,
    octave: &Octave,
    position: &Units,
) -> Transform {
    let semitones = Semitones::from(*syllable) + Semitones::from(*octave);
    let x = theme.grid.bar_size * position.0;
    let y = theme.grid.semitone_size * semitones.0 as f32;
    Transform::from_xyz(x, y, 0.0)
}

fn create_note_block(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &Syllable, &Octave, &Duration, &Units), Added<Syllable>>,
) {
    for (entity, syllable, octave, duration, units) in query.iter() {
        let shape = shapes::Rectangle {
            width: theme.grid.bar_size * Units::from(*duration).0,
            height: theme.grid.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        };
        let fill_color = theme.syllable.color_of_syllable_octave(*syllable, *octave);
        commands
            .entity(entity)
            .insert_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(fill_color, theme.core.outline_color),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default()
                        .with_line_width(theme.grid.note_outline),
                },
                calc_transform(&theme, syllable, octave, units),
            ));
    }
}

fn update_note_transform(
    mut _commands: Commands,
    theme: Res<NotationTheme>,
    mut query: Query<
        (&Syllable, &Octave, &Units, &mut Transform),
        Or<(Changed<Syllable>, Changed<Units>)>,
    >,
) {
    for (syllable, octave, units, mut transform) in query.iter_mut() {
        *transform = calc_transform(&theme, syllable, octave, units);
    }
}
