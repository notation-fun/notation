use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::egui::{Ui};

use notation_bevy::kb::markdown_page::MarkDownPage;
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme, MarkDownAsset, KbPage, KbContent, EasyLinkEvent};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HarmonicsSection {
    None,
    SingleString,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct HarmonicsPage {
    pub path: &'static str,
    pub section: HarmonicsSection,
}

impl KbPage for HarmonicsPage {
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        MarkDownPage::markdown_ui(ui, texts, assets, state, theme, link_evts, self.path);
    }
}

impl KbContent for HarmonicsPage {
    fn content_ui(
        &mut self,
        ui: &mut Ui,
        _texts: &Assets<MarkDownAsset>,
        _assets: &NotationAssets,
        _state: &NotationState,
        _theme: &NotationTheme,
        _link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        match self.section {
            HarmonicsSection::None => {
            },
            HarmonicsSection::SingleString => {
                ui.label("TODO");
            },
        }
    }
}

impl HarmonicsPage {
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
            section: HarmonicsSection::None,
        }
    }
    pub fn content_visible(&self) -> bool {
        match self.section {
            HarmonicsSection::None => false,
            HarmonicsSection::SingleString => true,
        }
    }
}
