use bevy::prelude::*;

#[derive(Debug)]
pub struct WindowResizedEvent();

pub fn add_notation_app_events(app: &mut AppBuilder) {
    app.add_event::<WindowResizedEvent>();
}
