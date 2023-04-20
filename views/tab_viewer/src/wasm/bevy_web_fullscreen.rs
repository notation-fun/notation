//https://github.com/ostwilkens/bevy_web_fullscreen

use bevy::prelude::*;
use bevy::window::Windows;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

type OnResizeSender = Sender<()>;
type OnResizeReceiver = Receiver<()>;

#[derive(Resource)]
pub struct ResizeSender {
    pub sender: Mutex<OnResizeSender>
}

#[derive(Resource)]
pub struct ResizeReceiver {
    pub receiver: Mutex<OnResizeReceiver>
}

pub struct FullViewportPlugin;

impl Plugin for FullViewportPlugin {
    fn build(&self, app: &mut App) {
        let channel = std::sync::mpsc::channel();
        let resize_sender: OnResizeSender = channel.0;
        let resize_receiver: OnResizeReceiver = channel.1;

        app.insert_resource(ResizeSender {
                sender: Mutex::new(resize_sender)
            })
            .insert_resource(ResizeReceiver{
                receiver: Mutex::new(resize_receiver)
            })
            .add_startup_system(setup_viewport_resize_system)
            .add_system(viewport_resize_system);
    }
}

pub fn get_viewport_size() -> (f32, f32) {
    let web_window = web_sys::window().expect("could not get window");
    let document_element = web_window
        .document()
        .expect("could not get document")
        .document_element()
        .expect("could not get document element");

    let width = document_element.client_width();
    let height = document_element.client_height();
    web_log!(
        "bevy_web_fullscreen::get_viewport_size() -> {}, {}",
        width,
        height
    );

    (width as f32, height as f32)
}

fn setup_viewport_resize_system(resize_sender: Res<ResizeSender>) {
    let web_window = web_sys::window().expect("could not get window");
    let local_sender = resize_sender.sender.lock().unwrap().clone();

    local_sender.send(()).unwrap();

    gloo_events::EventListener::new(&web_window, "resize", move |_event| {
        web_log!(
            "bevy_web_fullscreen::setup_viewport_resize_system() {:?}",
            _event
        );
        local_sender.send(()).unwrap();
    })
    .forget();
}

fn viewport_resize_system(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    resize_receiver: Res<ResizeReceiver>,
) {
    if resize_receiver.receiver.lock().unwrap().try_recv().is_ok() {
        if let Ok(mut window) = window_query.get_single_mut() {
            let size = get_viewport_size();
            web_log!(
                "bevy_web_fullscreen::viewport_resize_system() {}, {}",
                size.0,
                size.1
            );
            window.set_resolution(size.0, size.1);
        }
    }
}
