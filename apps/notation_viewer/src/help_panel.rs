use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::EguiContext;

use notation_bevy::prelude::{MarkDownAsset, KbPageId, KbPage, KbPanel, EasyLinkEvent, NotationSettings};
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme};

use notation_bevy::kb::chords_page::ChordsPage;
use notation_bevy::kb::notes_page::NotesPage;
use notation_bevy::kb::markdown_page::MarkDownPage;

use crate::assets::NotationViewerAssets;

#[derive(Clone, Debug)]
pub struct HelpPanel {
    pub skip_frames: usize,
    pub open_times: usize,
    pub current_page_id: KbPageId,
    pub welcome: MarkDownPage,
    pub notes: NotesPage,
    pub chords: ChordsPage,
    pub usage: MarkDownPage,
}

impl FromWorld for HelpPanel {
    fn from_world(world: &mut World) -> Self {
        let settings = world.get_resource::<NotationSettings>().unwrap();
        Self {
            skip_frames: 2,
            open_times: 0,
            current_page_id: Self::WELCOME,
            welcome: MarkDownPage::new(NotationViewerAssets::get_welcome_path(&settings)),
            notes: Default::default(),
            chords: Default::default(),
            usage: MarkDownPage::new(NotationViewerAssets::get_usage_path(&settings)),
        }
    }
}

impl HelpPanel {
    pub const WELCOME: KbPageId = KbPageId::MarkDown("kb_welcome");
    pub const USAGE: KbPageId = KbPageId::MarkDown("kb_usage");

    pub const LINK_NOTES: &'static str = ":kb:notes";
    pub const LINK_CHORDS: &'static str = ":kb:chords";
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
        mut egui_ctx: ResMut<EguiContext>,
        texts: Res<Assets<MarkDownAsset>>,
        assets: Res<NotationAssets>,
        mut state: ResMut<NotationState>,
        theme: Res<NotationTheme>,
        mut link_evts: EventWriter<EasyLinkEvent>,
        mut help: ResMut<HelpPanel>,
    ) {
        if help.skip_frames > 0 {
            help.skip_frames -= 1;
            return;
        }
        (&mut help).window_ui(&mut egui_ctx, &texts, &assets, &mut state, &theme, &mut link_evts);
    }
    pub fn handle_link_evts(
        mut index: ResMut<HelpPanel>,
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
        println!("handle_link_evt {:?}", evt);
        match evt.link.as_str() {
            Self::LINK_NOTES => {
                self.current_page_id = KbPageId::Notes;
            },
            Self::LINK_CHORDS => {
                self.current_page_id = KbPageId::Chords;
            },
            _ => (),
        }
    }}
