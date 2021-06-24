use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;

use notation_proto::prelude::{Units, Entry};
use crate::prelude::{AddEntryEvent, EntryBundle, GridConfig, NoteBundle};

pub struct EntryDevPlugin;

impl Plugin for EntryDevPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(InspectorPlugin::<GridConfig>::new())
            .add_startup_system(crate::note::note_dev::register_inspectors.system());
    }
}
