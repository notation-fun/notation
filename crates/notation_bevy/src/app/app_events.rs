use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct WindowResizedEvent();

#[derive(Clone, Debug)]
pub struct MouseClickedEvent {
    pub cursor_position: Vec2,
}

#[derive(Clone, Debug)]
pub struct MouseDraggedEvent {
    pub delta: Vec2,
}

pub fn add_notation_app_events(app: &mut AppBuilder) {
    app.add_event::<WindowResizedEvent>();
    app.add_event::<MouseClickedEvent>();
    app.add_event::<MouseDraggedEvent>();
}
