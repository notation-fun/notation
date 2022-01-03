use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::{egui, EguiContext};

use notation_bevy::prelude::{MarkDownAsset, KbPageId, KbPage, KbContent, KbPanel, DockSide, EasyLinkEvent};
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme};

use notation_bevy::kb::chords_page::ChordsPage;
use notation_bevy::kb::notes_page::NotesPage;
use notation_bevy::kb::markdown_page::MarkDownPage;

use crate::theory::harmonics::{HarmonicsPage, HarmonicsSection};

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
            welcome: MarkDownPage::new(Self::PATH_WELCOME),
            notes: Default::default(),
            chords: Default::default(),
            harmonics: HarmonicsPage::new(Self::PATH_HARMONICS),
        }
    }
}

impl IndexPanel {
    pub const WELCOME: KbPageId = KbPageId::MarkDown(Self::PATH_WELCOME);
    pub const HARMONICS: KbPageId = KbPageId::Custom("harmonics");

    pub const PATH_WELCOME: &'static str = "kb/welcome.md";
    pub const PATH_HARMONICS: &'static str = "kb/harmonics.md";

    pub const LINK_HARMONICS_SINGLE_STRING: &'static str = ":kb:harmonics:single_string";
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
        mut link_evts: EventWriter<EasyLinkEvent>,
        mut index: ResMut<IndexPanel>,
    ) {
        if state.window_width > state.window_height {
            let min_width = state.window_width / 3.0;
            let max_width = state.window_width * 2.0 / 3.0;
            (&mut index).side_ui(&egui_ctx, &texts, &assets, &mut state, &theme, &mut link_evts, DockSide::Left, (min_width, max_width));
        } else {
            let min_height = state.window_height / 3.0;
            let max_height = state.window_height * 2.0 / 3.0;
            (&mut index).side_ui(&egui_ctx, &texts, &assets, &mut state, &theme, &mut link_evts, DockSide::Top, (min_height, max_height));
        }
        (&mut index).content_ui(&egui_ctx, &texts, &assets, &state, &theme, &mut link_evts);
    }
    fn content_ui(
        &mut self,
        egui_ctx: &EguiContext,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        if let Some(content) = match self.current_page_id {
            Self::HARMONICS => {
                if self.harmonics.content_visible() {
                    Some(&mut self.harmonics as &mut dyn KbContent)
                } else {
                    None
                }
            },
            _ => None,
        } {
            egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
                content.content_ui(ui, texts, assets, state, theme, link_evts);
            });
        }

    }
    pub fn handle_link_evts(
        mut index: ResMut<IndexPanel>,
        mut evts: EventReader<EasyLinkEvent>,
    ) {
        for evt in evts.iter() {
            (&mut index).handle_link_evt(evt);
        }
    }
    fn handle_link_evt(
        &mut self,
        evt: &EasyLinkEvent,
    ) {
        println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA {:?}", evt);
        match evt.link.as_str() {
            Self::LINK_HARMONICS_SINGLE_STRING => {
                self.harmonics.section = HarmonicsSection::SingleString;
            }
            _ => (),
        }
    }
}
