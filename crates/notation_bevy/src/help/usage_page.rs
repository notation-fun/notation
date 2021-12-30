use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::prelude::{NotationAppState, NotationAssets, NotationTheme};

use super::help_panel::{HelpPage, HelpPageId};
use notation_bevy_utils::asset::markdown_asset::MarkDownAsset;
use notation_bevy_utils::easy_mark::easy_mark;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct UsagePage {}

impl HelpPage for UsagePage {
    fn page_id(&self) -> HelpPageId {
        HelpPageId::Usage
    }
    fn tab_label(&self) -> &'static str {
        "Usage"
    }
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        _state: &NotationAppState,
        _theme: &NotationTheme,
    ) {
        if let Some(text) = texts.get(assets.help_usage.clone()) {
            easy_mark(ui, text.text.as_str());
        }
    }
}
