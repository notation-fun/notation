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
pub mod lane;
pub mod mini;
pub mod play;
pub mod tab;

pub mod lyrics;
pub mod melody;
pub mod shapes;
pub mod strings;

pub mod data;
pub mod settings;
pub mod theme;

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
    pub use crate::bar::bar_layout::BarLayoutData;
    #[doc(hidden)]
    pub use crate::bar::bar_playing::BarPlaying;
    #[doc(hidden)]
    pub use crate::bar::bar_plugin::BarPlugin;
    #[doc(hidden)]
    pub use crate::chord::chord_bundle::ChordBundle;
    #[doc(hidden)]
    pub use crate::data::bar_data::BarData;
    #[doc(hidden)]
    pub use crate::data::entry_data::EntryData;
    #[doc(hidden)]
    pub use crate::data::lane_data::LaneData;
    #[doc(hidden)]
    pub use crate::data::model_entry_data::ModelEntryData;
    #[doc(hidden)]
    pub use crate::entry::entry_bundle::EntryBundle;
    #[doc(hidden)]
    pub use crate::entry::entry_events::AddEntryEvent;
    #[doc(hidden)]
    pub use crate::entry::entry_playing::EntryPlaying;
    #[doc(hidden)]
    pub use crate::entry::entry_plugin::EntryPlugin;
    #[doc(hidden)]
    pub use crate::lane::lane_bundle::LaneBundle;
    #[doc(hidden)]
    pub use crate::lane::lane_layout::LaneLayoutData;
    #[doc(hidden)]
    pub use crate::lyrics::lyrics_grid::LyricsGrid;
    #[doc(hidden)]
    pub use crate::lyrics::lyrics_plugin::LyricsPlugin;
    #[doc(hidden)]
    pub use crate::melody::melody_grid::MelodyGrid;
    #[doc(hidden)]
    pub use crate::melody::melody_plugin::MelodyPlugin;
    #[doc(hidden)]
    pub use crate::mini::mini_plugin::MiniPlugin;
    #[doc(hidden)]
    pub use crate::play::play_plugin::PlayPlugin;
    #[doc(hidden)]
    pub use crate::settings::notation_settings::NotationSettings;
    #[doc(hidden)]
    pub use crate::shapes::shapes_plugin::ShapesPlugin;
    #[doc(hidden)]
    pub use crate::strings::strings_grid::{StringsGrid4, StringsGrid6};
    #[doc(hidden)]
    pub use crate::strings::strings_plugin::StringsPlugin;
    #[doc(hidden)]
    pub use crate::tab::tab_asset::TabAsset;
    #[doc(hidden)]
    pub use crate::tab::tab_bars::TabBars;
    #[doc(hidden)]
    pub use crate::tab::tab_bundle::TabBundle;
    #[doc(hidden)]
    pub use crate::tab::tab_events::AddTabEvent;
    #[doc(hidden)]
    pub use crate::tab::tab_plugin::TabPlugin;
    #[doc(hidden)]
    pub use crate::tab::tab_state::TabState;
    #[doc(hidden)]
    pub use crate::theme::core_theme::CoreTheme;
    #[doc(hidden)]
    pub use crate::theme::grid_theme::GridTheme;
    #[doc(hidden)]
    pub use crate::theme::guitar_theme::GuitarTheme;
    #[doc(hidden)]
    pub use crate::theme::notation_theme::NotationTheme;
    #[doc(hidden)]
    pub use crate::theme::strings_theme::StringsTheme;
    #[doc(hidden)]
    pub use crate::theme::theme_colors::ThemeColors;
    #[doc(hidden)]
    pub use crate::tone::tone_bundle::ToneBundle;
    #[doc(hidden)]
    pub use crate::tone::tone_mode::ToneMode;
    #[doc(hidden)]
    pub use crate::ui::layout::NotationLayout;
    #[doc(hidden)]
    pub use crate::ui::NotationUiPlugin;
    #[doc(hidden)]
    pub use crate::viewer::run_notation_viewer;
    #[doc(hidden)]
    pub use bevy_utils::prelude::*;
}
