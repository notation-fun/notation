use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_proto::prelude::{Unit, Solfege, ArcLine};

fn make_line() -> ArcLine {
    vec![
        (Solfege::DO_4, Unit::Quarter),
        (Solfege::RE_4, Unit::Quarter),
        (Solfege::MI_4, Unit::Quarter),
        (Solfege::FA_4, Unit::Quarter),
        (Solfege::SO_4, Unit::Whole),
    ].into()
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::TEAL, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(10.0),
        },
        Transform::default(),
    ));
}
