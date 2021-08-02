pub use {notation_midi, notation_model};

pub use {bevy, bevy_prototype_lyon};

#[cfg(target_arch = "wasm32")]
pub use bevy_webgl2;

#[cfg(feature = "inspector")]
pub use bevy_inspector_egui;

pub mod chord;
pub mod entry;
pub mod tone;
pub mod word;

pub mod bar;
pub mod play;
pub mod tab;

pub mod fretted;
pub mod guitar;
pub mod lyrics;
pub mod melody;

pub mod settings;
pub mod theme;
pub mod utils;

pub mod app;
pub mod ext;
pub mod ui;
pub mod viewer;

#[cfg(feature = "inspector")]
pub mod inspector;

#[cfg(feature = "dev")]
pub mod dev;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::app::notation_app::{NotationApp, NotationPlugins};
    #[doc(hidden)]
    pub use crate::app::notation_app_events::WindowResizedEvent;
    #[doc(hidden)]
    pub use crate::app::notation_app_state::{NotationAppState, TabPathes};
    #[doc(hidden)]
    pub use crate::bar::bar_bundle::BarBundle;
    #[doc(hidden)]
    pub use crate::bar::bar_plugin::BarPlugin;
    #[doc(hidden)]
    pub use crate::bar::layer_bundle::LayerBundle;
    #[doc(hidden)]
    pub use crate::chord::chord_bundle::ChordBundle;
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
    pub use crate::lyrics::lyrics_grid::LyricsGrid;
    #[doc(hidden)]
    pub use crate::lyrics::lyrics_layer_bundle::LyricsLayerBundle;
    #[doc(hidden)]
    pub use crate::lyrics::lyrics_plugin::LyricsPlugin;
    #[doc(hidden)]
    pub use crate::melody::melody_grid::MemoryGrid;
    #[doc(hidden)]
    pub use crate::melody::melody_layer_bundle::MelodyLayerBundle;
    #[doc(hidden)]
    pub use crate::melody::melody_plugin::MelodyPlugin;
    #[doc(hidden)]
    pub use crate::play::play_plugin::PlayPlugin;
    #[doc(hidden)]
    pub use crate::play::play_state::PlayState;
    #[doc(hidden)]
    pub use crate::settings::notation_settings::NotationSettings;
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
    pub use crate::theme::core_theme::CoreTheme;
    #[doc(hidden)]
    pub use crate::theme::fretted_theme::FrettedTheme;
    #[doc(hidden)]
    pub use crate::theme::grid_theme::{GridCol, GridRow, GridTheme};
    #[doc(hidden)]
    pub use crate::theme::guitar_theme::GuitarTheme;
    #[doc(hidden)]
    pub use crate::theme::notation_theme::NotationTheme;
    #[doc(hidden)]
    pub use crate::theme::syllable_theme::SyllableTheme;
    #[doc(hidden)]
    pub use crate::tone::tone_bundle::ToneBundle;
    #[doc(hidden)]
    pub use crate::tone::tone_mode::ToneMode;
    #[doc(hidden)]
    pub use crate::word::word_bundle::WordBundle;
    #[doc(hidden)]
    pub use crate::ui::NotationUiPlugin;
    #[doc(hidden)]
    pub use crate::utils::lyon_shape::{LyonShape, LyonShapeOp};
    #[doc(hidden)]
    pub use crate::viewer::run_notation_viewer;
}
