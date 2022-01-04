use std::f64::consts::{PI, FRAC_PI_2};

use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::egui::{self, *};
use notation_bevy::bevy_egui::egui::plot::*;

use notation_bevy::kb::markdown_page::MarkDownPage;
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme, MarkDownAsset, KbPage, KbContent, EasyLinkEvent, BevyUtil, Syllable};

#[derive(Copy, Clone, Debug)]
pub struct ScalePage {
    pub path: &'static str,
}

impl KbPage for ScalePage {
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
        ui.separator();
    }
}

impl KbContent for ScalePage {
    fn content_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
    }
}

impl ScalePage {
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
        }
    }
}
