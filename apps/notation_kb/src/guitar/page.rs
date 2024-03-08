use tab_viewer::edger_bevy::app::state::AppState;
use tab_viewer::edger_bevy::bevy_prelude::*;
use tab_viewer::edger_bevy::egui::*;

use tab_viewer::kb::markdown_page::MarkDownPage;
use tab_viewer::prelude::{NotationState, NotationAssets, NotationTheme, MarkDownAsset, KbPage, KbContent, EasyLinkEvent, Syllable};

#[derive(Clone, Debug)]
pub struct GuitarPage {
    pub path: String,
}

impl KbPage for GuitarPage {
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        app_state: &AppState,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        MarkDownPage::markdown_ui(ui, texts, app_state, state, theme, link_evts, self.path.as_str());
    }
}

impl KbContent for GuitarPage {
    fn content_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        app_state: &AppState,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        ui.label("TODO");
    }
}

impl GuitarPage {
    pub fn new(path: String) -> Self {
        Self {
            path,
        }
    }
}
