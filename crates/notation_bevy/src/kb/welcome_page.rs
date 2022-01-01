use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::prelude::{NotationState, NotationAssets, NotationTheme};

use super::kb_page::{KbPage, KbPageId};
use notation_bevy_utils::asset::markdown_asset::MarkDownAsset;
use notation_bevy_utils::easy_mark::easy_mark;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct WelcomePage {}

impl KbPage for WelcomePage {
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        _state: &NotationState,
        _theme: &NotationTheme,
    ) {
        if let Some(text) = texts.get(assets.help_welcome.clone()) {
            easy_mark(ui, text.text.as_str());
        }
    }
}

impl WelcomePage {
    pub const ID: KbPageId = KbPageId::Welcome;
    pub const LABEL: &'static str = "Welcome";
}
