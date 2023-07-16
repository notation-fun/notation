pub extern crate lazy_static;
pub extern crate bevy_asset_loader;

pub use {notation_model};

#[cfg(feature = "dsl")]
pub use notation_dsl;

#[cfg(feature = "midi")]
pub use notation_midi;

pub use edger_bevy_app;

pub mod chord;
pub mod entry;
pub mod tone;
pub mod word;

pub mod bar;
pub mod lane;
pub mod mini;
pub mod play;
pub mod tab;

pub mod guitar;
pub mod lyrics;
pub mod melody;
pub mod harmony;
pub mod rhythm;
pub mod shapes;
pub mod strings;

pub mod data;
pub mod settings;
pub mod theme;
pub mod notation;

#[cfg(feature = "with_egui")]
pub mod egui;

#[cfg(feature = "with_egui")]
pub mod kb;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(feature = "dev")]
pub mod dev;

#[cfg(feature = "midi")]
pub mod midi;

#[cfg(feature = "dsl")]
pub mod dsl;

pub mod prelude {
    #[doc(hidden)]
    pub use notation_model::prelude::*;
    #[doc(hidden)]
    pub use edger_bevy_app::prelude::*;
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
    pub use crate::entry::entry_playing::EntryPlaying;
    #[doc(hidden)]
    pub use crate::entry::entry_plugin::EntryPlugin;
    #[cfg(feature = "with_egui")]
    #[doc(hidden)]
    pub use crate::egui::egui_fonts::EguiFontSizes;
    #[cfg(feature = "with_egui")]
    #[doc(hidden)]
    pub use crate::egui::egui_plugin::EguiPlugin;
    #[doc(hidden)]
    pub use crate::guitar::guitar_view::GuitarView;
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
    pub use crate::harmony::harmony_grid::HarmonyGrid;
    #[doc(hidden)]
    pub use crate::harmony::harmony_plugin::HarmonyPlugin;
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
    pub use crate::tab::tab_asset::{TabAsset, TabError};
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
    pub use crate::theme::guitar_theme::GuitarTheme;
    #[doc(hidden)]
    pub use crate::theme::notation_theme::NotationTheme;
    #[doc(hidden)]
    pub use crate::theme::theme_colors::ThemeColors;
    #[doc(hidden)]
    pub use crate::theme::theme_z::ThemeZ;
    #[doc(hidden)]
    pub use crate::tone::tone_bundle::ToneBundle;
    #[doc(hidden)]
    pub use crate::tone::tone_mode::ToneMode;
    #[doc(hidden)]
    pub use crate::notation::app::{NotationApp, NotationPlugins};
    #[doc(hidden)]
    pub use crate::notation::args::{NotationArgs};
    #[doc(hidden)]
    pub use crate::notation::events::*;
    #[doc(hidden)]
    pub use crate::notation::state::{NotationState};
    #[doc(hidden)]
    pub use crate::notation::assets::{NotationAssets, NotationAssetsStates, ExtraAssets, NoExtraAssets};
    #[doc(hidden)]
    pub use crate::notation::layout::NotationLayout;
    #[doc(hidden)]
    pub use crate::notation::tab_viewer::TabViewer;
    #[doc(hidden)]
    pub use crate::notation::ui::NotationUiPlugin;
    #[doc(hidden)]
    pub use crate::notation::control::Control;
    #[cfg(feature = "with_egui")]
    #[doc(hidden)]
    pub use crate::notation::egui_control_panel::EguiControlPanel;
    #[cfg(feature = "with_egui")]
    #[doc(hidden)]
    pub use crate::kb::kb_page::{KbPage, KbPageId, KbContent};
    #[cfg(feature = "with_egui")]
    #[doc(hidden)]
    pub use crate::kb::page_helper::PageHelper;
    #[cfg(feature = "with_egui")]
    #[doc(hidden)]
    pub use crate::kb::kb_panel::KbPanel;
    #[cfg(feature = "midi")]
    #[doc(hidden)]
    pub use crate::notation_midi::prelude::*;
    #[cfg(feature = "midi")]
    #[doc(hidden)]
    pub use crate::midi::midi_control::MidiControl;
    #[cfg(feature = "dsl")]
    #[doc(hidden)]
    pub use crate::notation_dsl::prelude::*;
}
