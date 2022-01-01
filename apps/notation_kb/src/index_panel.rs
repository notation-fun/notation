use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::EguiContext;

use notation_bevy::prelude::{MarkDownAsset, KbPageId, KbPage, KbPanel, DockSide};
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme};

use notation_bevy::kb::chords_page::ChordsPage;
use notation_bevy::kb::notes_page::NotesPage;
use notation_bevy::kb::usage_page::UsagePage;
use notation_bevy::kb::welcome_page::WelcomePage;

#[derive(Clone, Debug, Default)]
pub struct IndexPanel {
    pub open_times: usize,
    pub current_page_id: KbPageId,
    pub welcome: WelcomePage,
    pub notes: NotesPage,
    pub chords: ChordsPage,
    pub usage: UsagePage,
}

impl KbPanel for IndexPanel {
    fn get_title(&self) -> &str {
        "Index (F5, H)"
    }
    fn get_current_page_id(&self) -> KbPageId {
        self.current_page_id.clone()
    }
    fn set_current_page_id(&mut self, page_id: KbPageId) {
        self.current_page_id = page_id;
    }
    fn get_page_tabs(&self) -> Vec<(KbPageId, &'static str)> {
        vec![
            (WelcomePage::ID, WelcomePage::LABEL),
            (NotesPage::ID, NotesPage::LABEL),
            (ChordsPage::ID, ChordsPage::LABEL),
            (UsagePage::ID, UsagePage::LABEL),
        ]
    }
    fn get_page_mut(&mut self, page_id: KbPageId) -> &mut dyn KbPage {
        match page_id {
            KbPageId::Notes => &mut self.notes as &mut dyn KbPage,
            KbPageId::Chords => &mut self.chords as &mut dyn KbPage,
            KbPageId::Usage => &mut self.usage as &mut dyn KbPage,
            _ => &mut self.welcome as &mut dyn KbPage,
        }
    }
    fn on_close(&mut self) {
        if self.open_times == 0 && self.current_page_id == KbPageId::Welcome {
            self.set_current_page_id(KbPageId::Notes);
        }
        self.open_times += 1;
    }
}

impl IndexPanel {
    pub fn index_ui(
        egui_ctx: Res<EguiContext>,
        texts: Res<Assets<MarkDownAsset>>,
        assets: Res<NotationAssets>,
        mut state: ResMut<NotationState>,
        theme: Res<NotationTheme>,
        mut index: ResMut<IndexPanel>,
    ) {
        if state.window_width > state.window_height {
            let min_width = state.window_width / 3.0;
            let max_width = state.window_width * 2.0 / 3.0;
            (&mut index).side_ui(&egui_ctx, &texts, &assets, &mut state, &theme, DockSide::Right, (min_width, max_width));
        } else {
            let min_height = state.window_height / 3.0;
            let max_height = state.window_height * 2.0 / 3.0;
            (&mut index).side_ui(&egui_ctx, &texts, &assets, &mut state, &theme, DockSide::Bottom, (min_height, max_height));
        }
    }
}
