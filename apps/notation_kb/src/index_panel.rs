use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::{egui, EguiContext};

use notation_bevy::prelude::{MarkDownAsset, KbPageId, KbPage, KbContent, KbPanel, DockSide};
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme};

use notation_bevy::kb::chords_page::ChordsPage;
use notation_bevy::kb::notes_page::NotesPage;
use notation_bevy::kb::markdown_page::MarkDownPage;

use crate::theory::harmonics::HarmonicsPage;

#[derive(Clone, Debug)]
pub struct IndexPanel {
    pub open_times: usize,
    pub current_page_id: KbPageId,
    pub welcome: MarkDownPage,
    pub notes: NotesPage,
    pub chords: ChordsPage,
    pub harmonics: HarmonicsPage,
}

impl Default for IndexPanel {
    fn default() -> Self {
        Self {
            open_times: 0,
            current_page_id: Self::WELCOME,
            welcome: MarkDownPage::new(Self::WELCOME_PATH),
            notes: Default::default(),
            chords: Default::default(),
            harmonics: Default::default(),
        }
    }
}

impl IndexPanel {
    pub const WELCOME_PATH: &'static str = "kb/welcome.md";
    pub const WELCOME: KbPageId = KbPageId::MarkDown(Self::WELCOME_PATH);
    pub const HARMONICS: KbPageId = KbPageId::Custom("harmonics");
}

impl KbPanel for IndexPanel {
    fn get_title(&self) -> &str {
        "Index (F1, H)"
    }
    fn get_current_page_id(&self) -> KbPageId {
        self.current_page_id.clone()
    }
    fn set_current_page_id(&mut self, page_id: KbPageId) {
        self.current_page_id = page_id;
    }
    fn get_page_tabs(&self) -> Vec<(KbPageId, &'static str)> {
        vec![
            (Self::WELCOME, "Welcome"),
            (KbPageId::Notes, "Notes"),
            (KbPageId::Chords, "Chords"),
            (Self::HARMONICS, "Harmonics"),
        ]
    }
    fn get_page_mut(&mut self, page_id: KbPageId) -> &mut dyn KbPage {
        match page_id {
            KbPageId::Notes => &mut self.notes as &mut dyn KbPage,
            KbPageId::Chords => &mut self.chords as &mut dyn KbPage,
            Self::HARMONICS => &mut self.harmonics as &mut dyn KbPage,
            _ => &mut self.welcome as &mut dyn KbPage,
        }
    }
    fn on_close(&mut self) {
        if self.open_times == 0 && self.current_page_id == Self::WELCOME {
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
            (&mut index).side_ui(&egui_ctx, &texts, &assets, &mut state, &theme, DockSide::Left, (min_width, max_width));
        } else {
            let min_height = state.window_height / 3.0;
            let max_height = state.window_height * 2.0 / 3.0;
            (&mut index).side_ui(&egui_ctx, &texts, &assets, &mut state, &theme, DockSide::Top, (min_height, max_height));
        }
        if let Some(content) = match index.get_current_page_id() {
            Self::HARMONICS => {
                Some(&mut (&mut index).harmonics as &mut dyn KbContent)
            },
            _ => None,
        } {
            egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
                content.content_ui(ui, &texts, &assets, &mut state, &theme);
            });
        }
    }
}
