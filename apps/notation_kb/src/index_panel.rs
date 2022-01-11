use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::{egui, EguiContext};
use notation_bevy::prelude::{StereoStream, ProtoTab, NotationSettings, Control, MidiState, PlayControlEvent, MidiControl};

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
            current_page_id: Self::SCALE,
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

    pub const LINK_MIDI_PLAY: &'static str = ":midi:play";
    pub const LINK_MIDI_STOP: &'static str = ":midi:stop";
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
    pub fn check_reload(
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        index: Res<IndexPanel>,
    ) {
        if let Some(tab) = state.tab.as_ref() {
            let need_reload = match index.current_page_id {
                Self::SCALE => {
                    index.scale.check_reload(&tab)
                },
                _ => false,
            };
            if need_reload {
                Control::reload_tab(&mut state, &mut theme);
            }
        }
    }
    pub fn hack_settings(
        state: Res<NotationState>,
        mut theme: ResMut<NotationTheme>,
        mut settings: ResMut<NotationSettings>,
    ) {
        theme.sizes.melody.note_height = 8.0;
        theme.sizes.melody.semitone_height = 8.0;
        settings.hide_mini_map = true;
        settings.hide_bar_number = true;
        settings.layout.focus_bar_ease_ms = 0;
        if state.window_width > 0.0 && state.window_height > 0.0 {
            if state.window_width > state.window_height {
                let width = state.window_width / 3.0 + theme.sizes.layout.page_margin;
                settings.hide_guitar_view = false;
                settings.override_guitar_width = Some(width);
                settings.hide_chords_view = true;
            } else {
                settings.hide_guitar_view = true;
                settings.hide_chords_view = false;
                settings.override_guitar_width = None;
                let height = state.window_height / 3.0;
                settings.override_chord_size = Some(height);
            }
        }
    }
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
            let width = state.window_width / 3.0;
            (&mut index).side_ui(&egui_ctx, &texts, &assets, &mut state, &theme, &mut link_evts, DockSide::Left, (width, width));
        } else {
            let height = state.window_height / 3.0;
            (&mut index).side_ui(&egui_ctx, &texts, &assets, &mut state, &theme, &mut link_evts, DockSide::Top, (height, height));
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
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
        mut index: ResMut<IndexPanel>,
        mut evts: EventReader<EasyLinkEvent>,
    ) {
        for evt in evts.iter() {
            (&mut index).handle_link_evt(&mut midi_state, &mut play_control_evts, evt);
        }
    }
    fn handle_link_evt(
        &mut self,
        midi_state: &mut MidiState,
        play_control_evts: &mut EventWriter<PlayControlEvent>,
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
            },
            Self::LINK_MIDI_PLAY => {
                MidiControl::play(midi_state, play_control_evts)
            },
            Self::LINK_MIDI_STOP => {
                MidiControl::stop(midi_state, play_control_evts)
            },
            _ => (),
        }
    }
    fn _index_audio(
        &mut self,
        stream: &mut StereoStream,
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
        mut stream: ResMut<StereoStream>,
    ) {
        (&mut index)._index_audio(&mut stream);
    }
    pub fn make_tab(&self, _tab_path: String) -> Option<ProtoTab> {
        Some(match self.current_page_id {
            Self::SCALE => {
                self.scale.make_tab()
            },
            _ => Self::make_default_tab(),
        })
    }
    pub fn make_default_tab() -> ProtoTab {
        ProtoTab::new_empty()
    }
}
