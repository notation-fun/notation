use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_proto::prelude::TabBar;
use std::sync::Arc;

use crate::prelude::{BevyConfig, FrettedPlugin};
use notation_core::prelude::{Duration, Syllable, Units};
use notation_fretted::prelude::{Fretboard, HandShape, Pick};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_pick_blocks::<6>.system())
        .with_system(create_pick_blocks::<4>.system())
    /*
    .with_system(update_pick_transform.system())
     */
}

pub fn calc_transform(config: &BevyConfig, string: u8, position: Units) -> Transform {
    let x = config.grid.unit_size * position.0;
    let y =
        config.theme.fretted.string_space * -1.0 * string as f32 - config.grid.note_height / 2.0;
    Transform::from_xyz(x, y, config.theme.fretted.pick_z)
}

fn create_pick_block(
    commands: &mut Commands,
    config: &BevyConfig,
    entry_entity: Entity,
    duration: Duration,
    position: Units,
    string: u8,
    syllable: Syllable,
) {
    let shape = shapes::Rectangle {
        width: config.grid.unit_size * Units::from(duration).0,
        height: config.grid.note_height,
        origin: shapes::RectangleOrigin::BottomLeft,
    };
    let fill_color = config.theme.syllable.color_of_syllable(syllable);
    let block_entity = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(fill_color, config.theme.outline_color),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(config.grid.note_outline),
            },
            calc_transform(&config, string, position),
        ))
        .id();
    commands.entity(entry_entity).push_children(&[block_entity]);
}

fn create_pick_blocks<const S: usize>(
    mut commands: Commands,
    config: Res<BevyConfig>,
    query: Query<(&Parent, Entity, &Pick, &Duration, &Units), Added<Pick>>,
    layer_query: Query<(&Arc<TabBar>, &Fretboard<S>, &Children)>,
    shape_query: Query<&HandShape<S>>,
) {
    for (parent, entity, pick, duration, units) in query.iter() {
        if let Some((bar, fretboard, shape)) =
            FrettedPlugin::get_fretted_shape(&layer_query, &shape_query, parent.0, units)
        {
            for string in pick.get_strings() {
                if let Some(note) = fretboard.shape_note(&shape, string) {
                    let syllable = bar.calc_syllable(&note);
                    create_pick_block(
                        &mut commands,
                        &config,
                        entity,
                        *duration,
                        *units,
                        string,
                        syllable,
                    );
                }
            }
        }
    }
}

/*
fn update_pick_transform(mut _commands: Commands,
    config: Res<BevyConfig>,
    mut query: Query<(&Pick, &Octave, &Units, &mut Transform),
                    Or<(Changed<Pick>, Changed<Units>)>>,
) {
    for (pick, octave, units, mut transform) in query.iter_mut() {
        *transform = calc_transform(&config, pick, octave, units);
    }
}

 */
