use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::egui::{self, Ui};

use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme, MarkDownAsset, KbPage, KbPageId, KbContent};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct HarmonicsPage {}

impl KbPage for HarmonicsPage {
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        _texts: &Assets<MarkDownAsset>,
        _assets: &NotationAssets,
        _state: &NotationState,
        _theme: &NotationTheme,
    ) {
        ui.label("TODO");
    }
}

impl HarmonicsPage {
    pub const LABEL: &'static str = "Harmonics";
}

impl KbContent for HarmonicsPage {
    fn content_ui(
        &mut self,
        ui: &mut Ui,
        _texts: &Assets<MarkDownAsset>,
        _assets: &NotationAssets,
        _state: &NotationState,
        _theme: &NotationTheme,
    ) {
        ui.label("TODO");
    }
}
