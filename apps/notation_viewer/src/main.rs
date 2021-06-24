use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::render::camera::OrthographicProjection;


use bevy_inspector_egui::egui;
use bevy_inspector_egui::bevy_egui;

use notation_proto::prelude::{Unit, Solfege, Duration, Entry, ArcLine};
use notation_bevy::prelude::{AddLineEvent, NotationPlugins, NotationDevPlugins};

fn make_line() -> ArcLine {
    vec![
        (Solfege::DO_4, Unit::Quarter),
        (Solfege::DO_4, Unit::Quarter),
        (Solfege::RE_4, Unit::Quarter),
        (Solfege::MI_4, Unit::Quarter),
        (Solfege::FA_4, Unit::Quarter),
        (Solfege::SO_4, Unit::Whole),
    ].into_iter().map(
        |v| Entry::from((v.0, Duration::from(v.1)))
    ).collect::<Vec<Entry>>()
    .into()
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugins(NotationPlugins)
        .add_plugins(NotationDevPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(add_lines.system())
        .add_system(update_camera.system())
        .add_system(setup_ui.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_lines(mut evts: EventWriter<AddLineEvent>) {
    evts.send(AddLineEvent(make_line()));
}

fn update_camera(
    _keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut get_cam: Query<(&mut Transform, &mut OrthographicProjection)>,
) {
    for event in mouse_motion_events.iter() {
        if mouse_input.pressed(MouseButton::Left) {
            let (mut cam, _) = get_cam.single_mut().unwrap();
            let trans = cam.translation;
            *cam = Transform::from_xyz(trans.x - event.delta.x, trans.y + event.delta.y, trans.z);
        }
    }
}

fn setup_ui(mut commands: Commands,
        egui_context: ResMut<bevy_egui::EguiContext>,
        query: Query<Entity, With<std::sync::Arc<ArcLine>>>,
        mut evts: EventWriter<AddLineEvent>) {
    egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
        if ui.button("Clear Lines").clicked() {
            for line in query.iter() {
                commands.entity(line).despawn_recursive();
            }
        }
        ui.separator();
        if ui.button("Add Line").clicked() {
            evts.send(AddLineEvent(make_line()));
        }
    });
}