use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::EguiContext;

use notation_bevy::prelude::{MarkDownAsset, KbPageId, KbPage, KbPanel, EasyLinkEvent};
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme};

use notation_bevy::kb::chords_page::ChordsPage;
use notation_bevy::kb::notes_page::NotesPage;
use notation_bevy::kb::markdown_page::MarkDownPage;

#[derive(Clone, Debug)]
pub struct HelpPanel {
    pub open_times: usize,
    pub current_page_id: KbPageId,
    pub welcome: MarkDownPage,
    pub notes: NotesPage,
    pub chords: ChordsPage,
    pub usage: MarkDownPage,
}

impl Default for HelpPanel {
    fn default() -> Self {
        Self {
            open_times: 0,
            current_page_id: Self::WELCOME,
            welcome: MarkDownPage::new(Self::WELCOME_PATH),
            notes: Default::default(),
            chords: Default::default(),
            usage: MarkDownPage::new(Self::USAGE_PATH),
        }
    }
}

impl HelpPanel {
    pub const WELCOME_PATH: &'static str = "kb/welcome.md";
    pub const WELCOME: KbPageId = KbPageId::MarkDown(Self::WELCOME_PATH);
    pub const USAGE_PATH: &'static str = "kb/usage.md";
    pub const USAGE: KbPageId = KbPageId::MarkDown(Self::USAGE_PATH);
}

impl KbPanel for HelpPanel {
    fn get_title(&self) -> &str {
        "Help (F1, H)"
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
            (Self::USAGE, "Usage"),
        ]
    }
    fn get_page_mut(&mut self, page_id: KbPageId) -> &mut dyn KbPage {
        match page_id {
            KbPageId::Notes => &mut self.notes as &mut dyn KbPage,
            KbPageId::Chords => &mut self.chords as &mut dyn KbPage,
            Self::USAGE => &mut self.usage as &mut dyn KbPage,
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

impl HelpPanel {
    pub fn help_ui(
        egui_ctx: Res<EguiContext>,
        texts: Res<Assets<MarkDownAsset>>,
        assets: Res<NotationAssets>,
        mut state: ResMut<NotationState>,
        theme: Res<NotationTheme>,
        mut link_evts: EventWriter<EasyLinkEvent>,
        mut help: ResMut<HelpPanel>,
    ) {
        (&mut help).window_ui(&egui_ctx, &texts, &assets, &mut state, &theme, &mut link_evts);
    }
}
