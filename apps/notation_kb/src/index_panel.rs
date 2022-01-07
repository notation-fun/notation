use notation_audio::prelude::MonoStream;
use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::{egui, EguiContext};

use notation_bevy::prelude::{MarkDownAsset, KbPageId, KbPage, KbContent, KbPanel, DockSide, EasyLinkEvent};
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme};

use notation_bevy::kb::markdown_page::MarkDownPage;

use crate::theory::scale::ScalePage;
use crate::theory::sound::{SoundPage, SoundSection};

#[derive(Clone, Debug)]
pub struct IndexPanel {
    pub skip_frames: usize,
    pub current_page_id: KbPageId,
    pub welcome: MarkDownPage,
    pub sound: SoundPage,
    pub scale: ScalePage,
}

impl Default for IndexPanel {
    fn default() -> Self {
        Self {
            /* if the first page displayed is using the chinese font, will crash, this is a hack around this
            wgpu error: Validation Error

Caused by:
    In CommandEncoder::copy_buffer_to_texture
    Copy error
    copy of Y 0..256 would end up overruning the bounds of the Destination texture of Y size 128
             */
            skip_frames: 2,

            #[cfg(debug_assertions)]
            current_page_id: Self::SOUND,
            #[cfg(not(debug_assertions))]
            current_page_id: Self::WELCOME,

            welcome: MarkDownPage::new(Self::PATH_WELCOME),
            sound: SoundPage::new(Self::PATH_SOUND),
            scale: ScalePage::new(Self::PATH_SCALE),
        }
    }
}

impl IndexPanel {
    pub const WELCOME: KbPageId = KbPageId::MarkDown(Self::PATH_WELCOME);
    pub const SOUND: KbPageId = KbPageId::Custom("sound");
    pub const SCALE: KbPageId = KbPageId::Custom("scale");

    pub const PATH_WELCOME: &'static str = "kb/welcome.md";
    pub const PATH_SOUND: &'static str = "kb/sound.md";
    pub const PATH_SCALE: &'static str = "kb/scale.md";

    pub const LINK_SOUND: &'static str = ":kb:sound";
    pub const LINK_SCALE: &'static str = ":kb:scale";
    pub const LINK_SOUND_SINGLE_STRING: &'static str = ":kb:sound:single_string";
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
            (Self::SOUND, "Sound"),
            (Self::SCALE, "Scale"),
        ]
    }
    fn get_page_mut(&mut self, page_id: KbPageId) -> &mut dyn KbPage {
        match page_id {
            Self::SOUND => &mut self.sound as &mut dyn KbPage,
            Self::SCALE => &mut self.scale as &mut dyn KbPage,
            _ => &mut self.welcome as &mut dyn KbPage,
        }
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
        if index.skip_frames > 0 {
            index.skip_frames -= 1;
            return;
        }

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
            Self::SOUND => {
                if self.sound.content_visible() {
                    Some(&mut self.sound as &mut dyn KbContent)
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
        println!("handle_link_evt {:?}", evt);
        match evt.link.as_str() {
            Self::LINK_SOUND => {
                self.current_page_id = Self::SOUND;
            },
            Self::LINK_SCALE => {
                self.current_page_id = Self::SCALE;
            },
            Self::LINK_SOUND_SINGLE_STRING => {
                self.current_page_id = Self::SOUND;
                self.sound.section = SoundSection::SingleString(Default::default());
            }
            _ => (),
        }
    }
    fn _index_audio(
        &mut self,
        stream: &mut MonoStream,
    ) {
        match self.current_page_id {
            Self::SOUND => {
                self.sound.audio(stream);
            },
            _ => {},
        }
    }
    pub fn index_audio(
        mut index: ResMut<IndexPanel>,
        mut stream: ResMut<MonoStream>,
    ) {
        (&mut index)._index_audio(&mut stream);
    }
}
