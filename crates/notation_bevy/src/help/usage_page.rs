use bevy::prelude::*;
use bevy_egui::egui::{Ui};

use crate::prelude::{NotationTheme, NotationAssets, NotationAppState};

use super::help_panel::{HelpPageId, HelpPage};
use notation_bevy_utils::{easy_mark::easy_mark, asset::markdown_asset::MarkDownAsset};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct UsagePage {
}

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