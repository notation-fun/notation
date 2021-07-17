use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::{Duration, Octave, Semitones, Syllable, Units};

use crate::prelude::BevyConfig;

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_note_block.system())
        .with_system(update_note_transform.system())
}

pub fn calc_transform(
    config: &Res<BevyConfig>,
    syllable: &Syllable,
    octave: &Octave,
    position: &Units,
) -> Transform {
    let semitones = Semitones::from(*syllable) + Semitones::from(*octave);
    let x = config.grid.bar_size * position.0;
    let y = config.grid.semitone_size * semitones.0 as f32;
    Transform::from_xyz(x, y, 0.0)
}

fn create_note_block(
    mut commands: Commands,
    config: Res<BevyConfig>,
    query: Query<(Entity, &Syllable, &Octave, &Duration, &Units), Added<Syllable>>,
) {
    for (entity, syllable, octave, duration, units) in query.iter() {
        let shape = shapes::Rectangle {
            width: config.grid.bar_size * Units::from(*duration).0,
            height: config.grid.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        };
        let fill_color = config
            .theme
            .syllable
            .color_of_syllable_octave(*syllable, *octave);
        commands
            .entity(entity)
            .insert_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(fill_color, config.theme.core.outline_color),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default()
                        .with_line_width(config.grid.note_outline),
                },
                calc_transform(&config, syllable, octave, units),
            ));
    }
}

fn update_note_transform(
    mut _commands: Commands,
    config: Res<BevyConfig>,
    mut query: Query<
        (&Syllable, &Octave, &Units, &mut Transform),
        Or<(Changed<Syllable>, Changed<Units>)>,
    >,
) {
    for (syllable, octave, units, mut transform) in query.iter_mut() {
        *transform = calc_transform(&config, syllable, octave, units);
    }
}
