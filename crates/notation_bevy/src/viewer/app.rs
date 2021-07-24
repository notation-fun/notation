use std::sync::Arc;

use bevy::input::mouse::MouseMotion;
use bevy::render::camera::OrthographicProjection;

use bevy::prelude::*;
use notation_model::prelude::*;
use crate::prelude::*;

#[cfg(feature = "inspector")]
use notation_bevy::bevy_inspector_egui::{bevy_egui, egui};

use super::top_panel;

pub struct CameraPanning(bool);

pub struct AppState {
    pub tab_path: String,
    pub tab_asset: Handle<TabAsset>,
    pub tab: Option<Arc<Tab>>,
    pub parse_error: Option<ParseError>,
    pub camera_panning: bool,
}

impl FromWorld for AppState {
    fn from_world(world: &mut World) -> Self {
        let server = world.get_resource::<AssetServer>().unwrap();
        let tab_path = "songs/jay/long_juan_feng.ron";
        let tab_asset = server.load(tab_path);
        Self {
            tab_path: tab_path.to_string(),
            tab_asset,
            tab: None,
            parse_error: None,
            camera_panning: false,
        }
    }
}

pub fn main() {
    let mut app = new_notation_app("Notation Viewer");
    app.add_startup_system(setup.system());

    app.init_resource::<AppState>();

    app.add_system(update_camera.system());

    #[cfg(feature = "inspector")]
    app.add_system(setup_ui.system());

    app.add_system(load_tab.system());

    app.insert_resource(top_panel::TopPanelState::default());
    app.add_system(top_panel::top_panel_ui.system());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn load_tab(
    mut state: ResMut<AppState>,
    assets: ResMut<Assets<TabAsset>>,
    mut evts: EventWriter<AddTabEvent>,
) {
    if state.tab.is_none() && state.parse_error.is_none() {
        if let Some(asset) = assets.get(&state.tab_asset) {
            match Tab::try_parse_arc(asset.tab.clone()) {
                Ok(tab) => {
                    state.tab = Some(tab.clone());
                    evts.send(AddTabEvent(tab));
                }
                Err(err) => {
                    println!("Parse Tab Failed: {:?}", err);
                    state.parse_error = Some(err);
                }
            }
        }
    }
}

fn update_camera(
    _keyboard_input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut state: ResMut<AppState>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut get_cam: Query<(&mut Transform, &mut OrthographicProjection)>,
) {
    if keyboard_input.just_released(KeyCode::Space) {
        state.camera_panning = !state.camera_panning;
    }

    if state.camera_panning {
        for event in mouse_motion_events.iter() {
            if mouse_input.pressed(MouseButton::Left) {
                let (mut cam, _) = get_cam.single_mut().unwrap();
                let trans = cam.translation;
                *cam =
                    Transform::from_xyz(trans.x - event.delta.x, trans.y + event.delta.y, trans.z);
            }
        }
    }
}

#[cfg(feature = "inspector")]
fn setup_ui(
    mut commands: Commands,
    mut state: ResMut<AppState>,
    egui_context: ResMut<bevy_egui::EguiContext>,
    mut camera_panning: ResMut<CameraPanning>,
    tab_query: Query<Entity, With<Arc<Tab>>>,
    _line_query: Query<Entity, With<Arc<Line>>>,
    _tab_evts: EventWriter<AddTabEvent>,
    _line_evts: EventWriter<AddLineEvent>,
) {
    egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
        if ui
            .button(format!("[space] Camera Panning: {:?}", camera_panning.0))
            .clicked()
        {
            *camera_panning = match camera_panning.0 {
                true => CameraPanning(false),
                false => CameraPanning(true),
            }
        }
        ui.separator();
        if ui.button("Clear Tabs").clicked() {
            for tab in tab_query.iter() {
                commands.entity(tab).despawn_recursive();
            }
        }
        if ui.button("Load Tab").clicked() {
            state.tab = None;
            state.parse_error = None;
        }
    });
}

