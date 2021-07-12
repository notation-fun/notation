pub use notation_model;

pub mod chord;
pub mod entry;
pub mod note;

pub mod bar;
pub mod line;
pub mod tab;

pub mod fretted;
pub mod guitar;

pub mod config;
pub mod utils;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::bar::bar_bundle::BarBundle;
    #[doc(hidden)]
    pub use crate::bar::bar_plugin::BarPlugin;
    #[doc(hidden)]
    pub use crate::bar::layer_bundle::LayerBundle;
    #[doc(hidden)]
    pub use crate::chord::chord_bundle::ChordBundle;
    #[doc(hidden)]
    pub use crate::config::bevy_config::BevyConfig;
    #[doc(hidden)]
    pub use crate::config::bevy_theme::BevyTheme;
    #[doc(hidden)]
    pub use crate::config::config_events::ConfigChangedEvent;
    #[doc(hidden)]
    pub use crate::config::config_plugin::ConfigPlugin;
    #[doc(hidden)]
    pub use crate::config::grid_config::{GridCol, GridConfig, GridRow};
    #[doc(hidden)]
    pub use crate::entry::entry_bundle::EntryBundle;
    #[doc(hidden)]
    pub use crate::entry::entry_dev::EntryDevPlugin;
    #[doc(hidden)]
    pub use crate::entry::entry_events::AddEntryEvent;
    #[doc(hidden)]
    pub use crate::entry::entry_plugin::EntryPlugin;
    #[doc(hidden)]
    pub use crate::fretted::fretted_grid::FrettedGrid;
    #[doc(hidden)]
    pub use crate::fretted::fretted_plugin::FrettedPlugin;
    #[doc(hidden)]
    pub use crate::guitar::guitar_layer_bundle::GuitarLayerBundle;
    #[doc(hidden)]
    pub use crate::guitar::guitar_plugin::GuitarPlugin;
    #[doc(hidden)]
    pub use crate::line::line_bundle::LineBundle;
    #[doc(hidden)]
    pub use crate::line::line_events::AddLineEvent;
    #[doc(hidden)]
    pub use crate::line::line_plugin::LinePlugin;
    #[doc(hidden)]
    pub use crate::note::note_bundle::NoteBundle;
    #[doc(hidden)]
    pub use crate::tab::tab_bundle::TabBundle;
    #[doc(hidden)]
    pub use crate::tab::tab_events::AddTabEvent;
    #[doc(hidden)]
    pub use crate::tab::tab_plugin::TabPlugin;
    #[doc(hidden)]
    pub use crate::utils::lyon_shape::{LyonShape, LyonShapeOp};

    use bevy::app::{PluginGroup, PluginGroupBuilder};
    use bevy_prototype_lyon::prelude::*;

    pub struct NotationPlugins;
    impl PluginGroup for NotationPlugins {
        fn build(&mut self, group: &mut PluginGroupBuilder) {
            group.add(ConfigPlugin);
            group.add(EntryPlugin);
            group.add(LinePlugin);
            group.add(BarPlugin);
            group.add(FrettedPlugin);
            group.add(GuitarPlugin);
            group.add(TabPlugin);
            //external plugins
            group.add(ShapePlugin);
        }
    }

    use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
    use bevy_inspector_egui::WorldInspectorPlugin;

    pub struct NotationDevPlugins;
    impl PluginGroup for NotationDevPlugins {
        fn build(&mut self, group: &mut PluginGroupBuilder) {
            group.add(EntryDevPlugin);
            //external plugins
            group.add(WorldInspectorPlugin::new());
            group.add(LogDiagnosticsPlugin::default());
            group.add(FrameTimeDiagnosticsPlugin::default());
        }
    }
}
