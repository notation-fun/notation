use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy_mod_picking::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use notation_viewer::bevy_4x_camera::{CameraRigBundle, FourXCameraPlugin};

struct Note {
    pub pitch: i32,
    pub octave: i32,
    pub duration: i32,
    pub string: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Note {
    pub fn new(pitch: i32, octave: i32, duration: i32, string: i32) -> Self {
        Note {
            pitch,
            octave,
            duration,
            string,
            x: 0,
            y: pitch,
            z: 0,
        }
    }
    pub fn get_color(&self) -> Color {
        match self.pitch % 7 {
            1 => Color::rgb(0.8, 0.0, 0.0),
            2 => Color::rgb(0.8, 0.0, 0.0),
            3 => Color::rgb(0.6, 0.8, 0.8),
            4 => Color::rgb(0.8, 0.0, 0.0),
            5 => Color::rgb(0.8, 0.8, 0.0),
            6 => Color::rgb(0.8, 0.0, 0.0),
            7 => Color::rgb(0.8, 0.0, 0.0),
            0 => Color::rgb(0.8, 0.0, 0.0),
            _ => Color::rgb(0.5, 0.5, 0.5),
        }
    }
    pub fn spawn(&self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn_bundle(PbrBundle {
            //mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            mesh: meshes.add(Mesh::from(shape::Box::new(self.duration as f32 * 4.0 - 0.1, 1.0, 1.0))),
            material: materials.add(self.get_color().into()),
            transform: Transform::from_xyz(self.x as f32, self.y as f32, self.z as f32 + (self.string - 3) as f32 * 2.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert_bundle(PickableBundle::default())
        .insert(BoundVol::default());
    }
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FourXCameraPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //let scene_handle = asset_server.load("guitar/scene.gltf#Scene0");
    //commands.spawn_scene(scene_handle);
    // plane
    /*
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.0, -100.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
     */
    // light
    type PointLightBundle = LightBundle; //Name is changed in bevy latest
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 5.0),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(15.0, 10.0, 10.0),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(30.0, 10.0, 10.0),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(30.0, 80.0, 10.0),
        ..Default::default()
    });
    commands.spawn_bundle(CameraRigBundle::default())
    .with_children(|cb| {
        cb.spawn_bundle(PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection {
                fov: 0.1,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-20.0, 20.0, 0.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        }).insert_bundle(PickingCameraBundle::default());
    });
    let mut notes = vec![
        Note::new(1, 0, 1, 5),
        Note::new(5, 0, 1, 3),
        Note::new(8, 0, 1, 2),
        Note::new(5, 0, 1, 3),
        Note::new(10, 0, 1, 1),
        Note::new(5, 0, 1, 3),
        Note::new(8, 0, 1, 2),
        Note::new(5, 0, 1, 3),
    ];
    let mut x: i32 = 0;
    for note in notes.iter_mut() {
        note.x += x + note.duration * 2;
        x += note.duration * 4;
    }
    let first = notes.first_mut();
    first.unwrap().duration = 8;
    let first = notes.first_mut();
    first.unwrap().x += 16 - 2;
    for note in &notes {
        note.spawn(&mut commands, &mut meshes, &mut materials);
    }
}
