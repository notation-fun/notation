// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use std::sync::Arc;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;

use bevy_inspector_egui::{bevy_egui, egui};

use notation_bevy::prelude::{
    AddLineEvent, AddTabEvent, ConfigPlugin, NotationDevPlugins, NotationPlugins,
};
use notation_model::prelude::{
    Bar, BarLayer, CoreEntry, Duration, GuitarEntry, GuitarHandShape, GuitarString, GuitarTuning,
    GuitarUtil, Key, Line, Pick, ProtoEntry, Roman, Scale, Section, SectionKind, Signature, Slice,
    Solfege, Tab, TabMeta, Tempo, Track, TrackKind,
};

#[cfg(target_arch = "wasm32")]
pub mod bevy_web_fullscreen;

pub struct CameraPanning(bool);

fn make_note_line() -> Line {
    (
        String::from("notes"),
        vec![
            (Solfege::LA_3, Duration::_1_8),
            (Solfege::DO_5, Duration::_1_8),
            (Solfege::MI_5, Duration::_1_8),
            (Solfege::LA_5, Duration::_1_8),
            (Solfege::MI_5, Duration::_1_8),
            (Solfege::DO_5, Duration::_1_8),
            (Solfege::LA_3, Duration::_1_8),
            (Solfege::DO_5, Duration::_1_8),
            (Solfege::MI_5, Duration::_1_8),
            (Solfege::LA_5, Duration::_1_8),
            (Solfege::MI_5, Duration::_1_8),
            (Solfege::DO_5, Duration::_1_8),
            (Solfege::DO_4, Duration::_1_8),
            (Solfege::DO_5, Duration::_1_8),
            (Solfege::MI_5, Duration::_1_8),
            (Solfege::DO_6, Duration::_1_8),
            (Solfege::MI_5, Duration::_1_8),
            (Solfege::DO_5, Duration::_1_8),
            (Solfege::DO_4, Duration::_1_8),
            (Solfege::DO_5, Duration::_1_8),
            (Solfege::MI_5, Duration::_1_8),
            (Solfege::DO_6, Duration::_1_8),
            (Solfege::MI_5, Duration::_1_8),
            (Solfege::DO_5, Duration::_1_8),
        ]
        .into_iter()
        .map(CoreEntry::from)
        .map(ProtoEntry::from)
        .collect::<Vec<ProtoEntry>>(),
    )
        .into()
}

fn make_chord_line() -> Line {
    (
        String::from("chords"),
        vec![
            (Roman::VI_MINOR, Duration::_1),
            (Roman::I_MAJOR, Duration::_1),
        ]
        .into_iter()
        .map(CoreEntry::from)
        .map(ProtoEntry::from)
        .collect::<Vec<ProtoEntry>>(),
    )
        .into()
}

fn make_shape_line() -> Line {
    let shape_e = GuitarHandShape::from([Some(0), Some(0), Some(0), Some(2), Some(2), Some(0)]);
    let shape_g = GuitarHandShape::from([Some(3), Some(0), Some(0), Some(0), Some(2), Some(3)]);
    let entries: Vec<ProtoEntry> = vec![
        GuitarEntry::from((shape_e, Duration::_1)),
        GuitarEntry::from((shape_g, Duration::_1)),
    ]
    .into_iter()
    .map(ProtoEntry::from)
    .collect();
    (String::from("shape"), entries).into()
}

fn make_pick_line() -> Line {
    let entries: Vec<ProtoEntry> = vec![
        GuitarEntry::from((Pick::from(GuitarString::_6), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_3), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_2), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_1), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_2), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_3), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_6), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_3), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_2), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_1), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_2), Duration::T_1_8)),
        GuitarEntry::from((Pick::from(GuitarString::_3), Duration::T_1_8)),
    ]
    .into_iter()
    .map(ProtoEntry::from)
    .collect();
    (String::from("pick"), entries).into()
}

fn make_tab() -> Arc<Tab> {
    let chords = Arc::new(make_chord_line());
    let chords_1 = Slice::new_arc(&chords, 0, 1);
    let chords_2 = Slice::new_arc(&chords, 1, 1);
    let shapes = Arc::new(make_shape_line());
    let shapes_1 = Slice::new_arc(&shapes, 0, 1);
    let shapes_2 = Slice::new_arc(&shapes, 1, 1);
    let picks = Arc::new(make_pick_line());
    let picks_1 = Slice::new_arc(&picks, 0, 12);
    let guitar = Arc::new(Track::new(TrackKind::Guitar, "guitar".into(), vec![
        Arc::new(ProtoEntry::from(GuitarEntry::Fretboard(
            GuitarUtil::new_acoustic_guitar_fretboard(GuitarTuning::Standard),
        ))),
    ]));

    let chord_1 = Arc::new(BarLayer::from(vec![chords_1]));
    let chord_2 = Arc::new(BarLayer::from(vec![chords_2]));
    let pick_1 = Arc::new(BarLayer::from((&guitar, vec![shapes_1, picks_1.clone()])));
    let pick_2 = Arc::new(BarLayer::from((&guitar, vec![shapes_2, picks_1.clone()])));
    let bar_1 = Arc::new(Bar::from(vec![chord_1, pick_1]));
    let bar_2 = Arc::new(Bar::from(vec![chord_2, pick_2]));
    let verse = Arc::new(Section::from((SectionKind::Verse, vec![
        bar_1.clone(),
        bar_1.clone(),
        bar_2.clone(),
        bar_2.clone(),
    ])));
    let meta = Arc::new(TabMeta {
        key: Key::G,
        scale: Scale::Major,
        signature: Signature::_4_4,
        tempo: Tempo::Bpm(60),
    });
    let lines = vec![chords, shapes, picks];
    let tracks = vec![guitar];
    let sections = vec![verse];
    let form = vec![0, 0];
    Tab::new(meta, lines, tracks, sections, form)
}

fn main() {
    let mut app = App::build();
    ConfigPlugin::insert_window_descriptor(&mut app, String::from("Notation Viewer"));
    app.insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugins(NotationPlugins)
        .add_startup_system(setup.system());

    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    app.add_startup_system(add_lines.system())
        .insert_resource(CameraPanning(false))
        .add_system(update_camera.system())
        .add_plugins(NotationDevPlugins)
        .add_system(setup_ui.system());

    app.add_startup_system(add_tabs.system());

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_web_fullscreen::FullViewportPlugin);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_lines(mut evts: EventWriter<AddLineEvent>) {
    evts.send(AddLineEvent(Arc::new(make_note_line())));
}

fn add_tabs(mut evts: EventWriter<AddTabEvent>) {
    evts.send(AddTabEvent(make_tab()));
}

fn update_camera(
    _keyboard_input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut camera_panning: ResMut<CameraPanning>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut get_cam: Query<(&mut Transform, &mut OrthographicProjection)>,
) {
    if keyboard_input.just_released(KeyCode::Space) {
        *camera_panning = match camera_panning.0 {
            true => CameraPanning(false),
            false => CameraPanning(true),
        }
    }

    if camera_panning.0 {
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

fn setup_ui(
    mut commands: Commands,
    egui_context: ResMut<bevy_egui::EguiContext>,
    mut camera_panning: ResMut<CameraPanning>,
    tab_query: Query<Entity, With<Arc<Tab>>>,
    line_query: Query<Entity, With<Arc<Line>>>,
    tab_evts: EventWriter<AddTabEvent>,
    line_evts: EventWriter<AddLineEvent>,
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
        if ui.button("Add Tabs").clicked() {
            add_tabs(tab_evts);
        }
        ui.separator();
        if ui.button("Clear Lines").clicked() {
            for line in line_query.iter() {
                commands.entity(line).despawn_recursive();
            }
        }
        if ui.button("Add Lines").clicked() {
            add_lines(line_evts);
        }
    });
}
