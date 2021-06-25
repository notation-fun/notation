pub use notation_core;
pub use notation_fretted;
pub use notation_guitar;

pub mod entry;
pub mod note;
pub mod chord;

pub mod line;

pub mod grid;
pub mod art;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::entry::entry_bundle::{EntryBundle};
    #[doc(hidden)]
    pub use crate::entry::entry_events::{AddEntryEvent};
    #[doc(hidden)]
    pub use crate::entry::entry_plugin::{EntryPlugin};
    #[doc(hidden)]
    pub use crate::entry::entry_dev::{EntryDevPlugin};
    #[doc(hidden)]
    pub use crate::note::note_bundle::{NoteBundle};
    #[doc(hidden)]
    pub use crate::chord::chord_bundle::{ChordBundle};
    #[doc(hidden)]
    pub use crate::line::line_bundle::{LineBundle};
    #[doc(hidden)]
    pub use crate::line::line_events::{AddLineEvent};
    #[doc(hidden)]
    pub use crate::line::line_plugin::{LinePlugin};
    #[doc(hidden)]
    pub use crate::grid::grid_config::{GridConfig};
    #[doc(hidden)]
    pub use crate::grid::grid_plugin::{GridPlugin};
    #[doc(hidden)]
    pub use crate::art::theme::{Theme};
    #[doc(hidden)]
    pub use crate::art::art_plugin::{ArtPlugin};

    use bevy::app::{PluginGroup, PluginGroupBuilder};
    use bevy_prototype_lyon::prelude::*;

    pub struct NotationPlugins;
    impl PluginGroup for NotationPlugins {
        fn build(&mut self, group: &mut PluginGroupBuilder) {
            group.add(GridPlugin);
            group.add(ArtPlugin);
            group.add(EntryPlugin);
            group.add(LinePlugin);
            //external plugins
            group.add(ShapePlugin);
        }
    }

    use bevy_inspector_egui::WorldInspectorPlugin;

    pub struct NotationDevPlugins;
    impl PluginGroup for NotationDevPlugins {
        fn build(&mut self, group: &mut PluginGroupBuilder) {
            group.add(EntryDevPlugin);
            //external plugins
            group.add(WorldInspectorPlugin::new());
        }
    }
}