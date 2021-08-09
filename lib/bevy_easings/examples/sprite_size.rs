use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup.system())
        .run();

    Ok(())
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let initial_size = 10.;
    let final_size = 100.;

    let spacing = 1.25;
    let screen_x = 570.;
    let screen_y = 150.;
    let mut x = -screen_x;
    let mut y = screen_y;

    for ease_function in &[
        bevy_easings::EaseFunction::QuadraticIn,
        bevy_easings::EaseFunction::QuadraticOut,
        bevy_easings::EaseFunction::QuadraticInOut,
        bevy_easings::EaseFunction::CubicIn,
        bevy_easings::EaseFunction::CubicOut,
        bevy_easings::EaseFunction::CubicInOut,
        bevy_easings::EaseFunction::QuarticIn,
        bevy_easings::EaseFunction::QuarticOut,
        bevy_easings::EaseFunction::QuarticInOut,
        bevy_easings::EaseFunction::QuinticIn,
        bevy_easings::EaseFunction::QuinticOut,
        bevy_easings::EaseFunction::QuinticInOut,
        bevy_easings::EaseFunction::SineIn,
        bevy_easings::EaseFunction::SineOut,
        bevy_easings::EaseFunction::SineInOut,
        bevy_easings::EaseFunction::CircularIn,
        bevy_easings::EaseFunction::CircularOut,
        bevy_easings::EaseFunction::CircularInOut,
        bevy_easings::EaseFunction::ExponentialIn,
        bevy_easings::EaseFunction::ExponentialOut,
        bevy_easings::EaseFunction::ExponentialInOut,
        bevy_easings::EaseFunction::ElasticIn,
        bevy_easings::EaseFunction::ElasticOut,
        bevy_easings::EaseFunction::ElasticInOut,
        bevy_easings::EaseFunction::BackIn,
        bevy_easings::EaseFunction::BackOut,
        bevy_easings::EaseFunction::BackInOut,
        bevy_easings::EaseFunction::BounceIn,
        bevy_easings::EaseFunction::BounceOut,
        bevy_easings::EaseFunction::BounceInOut,
    ] {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::RED.into()),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..Default::default()
            })
            .insert(
                Sprite {
                    size: Vec2::new(initial_size, initial_size),
                    ..Default::default()
                }
                .ease_to(
                    Sprite {
                        size: Vec2::new(final_size, final_size),
                        ..Default::default()
                    },
                    *ease_function,
                    bevy_easings::EasingType::PingPong {
                        duration: std::time::Duration::from_secs(1),
                        pause: Some(std::time::Duration::from_millis(500)),
                    },
                ),
            );
        y -= final_size * spacing;
        if y < -screen_y {
            x += final_size * spacing;
            y = screen_y;
        }
    }
}
