pub use {notation_midi, notation_model};

pub use {bevy, bevy_prototype_lyon};

#[cfg(target_arch = "wasm32")]
pub use bevy_webgl2;

#[cfg(feature = "inspector")]
pub use bevy_inspector_egui;

pub mod chord;
pub mod entry;
pub mod tone;

pub mod bar;
pub mod line;
pub mod play;
pub mod tab;

pub mod fretted;
pub mod guitar;

pub mod config;
pub mod utils;

pub mod ext;
pub mod ui;

#[cfg(feature = "inspector")]
pub mod inspector;

#[cfg(feature = "dev")]
pub mod dev;

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
    pub use crate::entry::entry_events::AddEntryEvent;
    #[doc(hidden)]
    pub use crate::entry::entry_plugin::EntryPlugin;
    #[doc(hidden)]
    pub use crate::entry::entry_state::EntryState;
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
    pub use crate::play::play_plugin::PlayPlugin;
    #[doc(hidden)]
    pub use crate::play::play_state::PlayState;
    #[doc(hidden)]
    pub use crate::tab::tab_asset::TabAsset;
    #[doc(hidden)]
    pub use crate::tab::tab_bundle::TabBundle;
    #[doc(hidden)]
    pub use crate::tab::tab_events::AddTabEvent;
    #[doc(hidden)]
    pub use crate::tab::tab_plugin::TabPlugin;
    #[doc(hidden)]
    pub use crate::tab::tab_state::TabState;
    #[doc(hidden)]
    pub use crate::tab::tab_state_bundle::TabStateBundle;
    #[doc(hidden)]
    pub use crate::tone::tone_bundle::ToneBundle;
    #[doc(hidden)]
    pub use crate::utils::lyon_shape::{LyonShape, LyonShapeOp};

    use bevy::app::{PluginGroup, PluginGroupBuilder};
    use bevy::prelude::*;
    use notation_midi::midi_plugin::MidiPlugin;

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
            group.add(PlayPlugin);
            //crates plugins
            group.add(MidiPlugin);
            //external plugins
            group.add(bevy_prototype_lyon::prelude::ShapePlugin);
        }
    }

    pub fn new_notation_app(title: &str) -> AppBuilder {
        let mut app = App::build();
        ConfigPlugin::insert_window_descriptor(&mut app, String::from(title));
        app.insert_resource(Msaa { samples: 8 });
        app.add_plugins(DefaultPlugins);
        app.add_plugins(NotationPlugins);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        // When building for WASM, print panics to the browser console
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(crate::ext::bevy_web_fullscreen::FullViewportPlugin);

        #[cfg(feature = "inspector")]
        app.add_plugins(crate::inspector::NotationInspectorPlugins);

        #[cfg(feature = "dev")]
        app.add_plugins(crate::dev::NotationDevPlugins);

        app
    }
}
