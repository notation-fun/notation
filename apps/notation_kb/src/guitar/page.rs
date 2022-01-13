use std::f64::consts::{PI, FRAC_PI_2};

use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::egui::*;
use notation_bevy::bevy_egui::egui::plot::*;

use notation_bevy::kb::markdown_page::MarkDownPage;
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme, MarkDownAsset, KbPage, KbContent, EasyLinkEvent, BevyUtil, Syllable};

#[derive(Copy, Clone, Debug)]
pub struct GuitarPage {
    pub path: &'static str,
}

impl KbPage for GuitarPage {
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

impl KbContent for GuitarPage {
    fn content_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        ui.label("TODO");
    }
}

impl GuitarPage {
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
        }
    }
}
