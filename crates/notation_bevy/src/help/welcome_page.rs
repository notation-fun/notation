use bevy::prelude::*;
use bevy_egui::egui::{Ui};

use crate::prelude::{NotationTheme, NotationAssets, NotationAppState};

use super::help_panel::{HelpPageId, HelpPage};
use notation_bevy_utils::{easy_mark::easy_mark, asset::markdown_asset::MarkDownAsset};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct WelcomePage {
}

impl HelpPage for WelcomePage {
    fn page_id(&self) -> HelpPageId {
        HelpPageId::Welcome
    }
    fn tab_label(&self) -> &'static str {
        "Welcome"
    }
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        _state: &NotationAppState,
        _theme: &NotationTheme,
    ) {
        if let Some(text) = texts.get(assets.help_welcome.clone()) {
            easy_mark(ui, text.text.as_str());
        }
    }
}