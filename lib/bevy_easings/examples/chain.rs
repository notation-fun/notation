use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup.system())
        .add_system(add_easing.system())
        .run();

    Ok(())
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::RED.into()),
            sprite: Sprite {
                size: Vec2::new(100., 100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Timer::from_seconds(1., false));
}

fn add_easing(mut commands: Commands, mut query: Query<(&mut Timer, Entity)>, time: Res<Time>) {
    for (mut timer, entity) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            let transform0 = Transform::default();
            let transform1 = Transform::from_translation(Vec3::new(500., 0., 0.));
            let transform2 = Transform::from_translation(Vec3::new(500., 300., 0.));
            let transform3 = Transform::from_translation(Vec3::new(-500., 300., 0.));
            let transform4 = Transform::from_translation(Vec3::new(-500., -300., 0.));
            let transform5 = Transform::from_translation(Vec3::new(500., -300., 0.));
            let transform6 = Transform::from_translation(Vec3::new(500., 0., 0.));
            let transform7 = Transform::default();

            let duration = std::time::Duration::from_millis(500);
            commands.entity(entity).insert(Timer::from_seconds(
                7. * duration.as_secs_f32() as f32 + 1.,
                false,
            ));
            commands.entity(entity).insert(
                transform0
                    .ease_to(
                        transform1,
                        bevy_easings::EaseFunction::QuadraticInOut,
                        bevy_easings::EasingType::Once { duration },
                    )
                    .ease_to(
                        transform2,
                        bevy_easings::EaseFunction::QuadraticInOut,
                        bevy_easings::EasingType::Once { duration },
                    )
                    .ease_to(
                        transform3,
                        bevy_easings::EaseFunction::QuadraticInOut,
                        bevy_easings::EasingType::Once { duration },
                    )
                    .ease_to(
                        transform4,
                        bevy_easings::EaseFunction::QuadraticInOut,
                        bevy_easings::EasingType::Once { duration },
                    )
                    .ease_to(
                        transform5,
                        bevy_easings::EaseFunction::QuadraticInOut,
                        bevy_easings::EasingType::Once { duration },
                    )
                    .ease_to(
                        transform6,
                        bevy_easings::EaseFunction::QuadraticInOut,
                        bevy_easings::EasingType::Once { duration },
                    )
                    .ease_to(
                        transform7,
                        bevy_easings::EaseFunction::QuadraticInOut,
                        bevy_easings::EasingType::Once { duration },
                    ),
            );
        }
    }
}
