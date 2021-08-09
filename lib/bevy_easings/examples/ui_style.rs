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
    commands.spawn_bundle(UiCameraBundle::default());

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
            .spawn_bundle(ImageBundle {
                material: materials.add(Color::RED.into()),
                ..Default::default()
            })
            .insert(
                Style {
                    size: Size {
                        width: Val::Percent(3.),
                        height: Val::Percent(3.),
                    },

                    margin: Rect {
                        bottom: Val::Percent(0.),
                        left: Val::Px(3.),
                        ..Default::default()
                    },
                    ..Default::default()
                }
                .ease_to(
                    Style {
                        margin: Rect {
                            bottom: Val::Percent(50.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    *ease_function,
                    bevy_easings::EasingType::PingPong {
                        duration: std::time::Duration::from_secs(1),
                        pause: Some(std::time::Duration::from_millis(500)),
                    },
                ),
            );
    }
}
