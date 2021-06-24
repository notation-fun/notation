use bevy::prelude::*;

use super::theme::Theme;

pub struct ArtPlugin;

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(ClearColor(Theme::default().background_color))
            .init_resource::<super::theme::Theme>()
        ;
    }
}
