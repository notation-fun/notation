use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::egui::{self, *};

use notation_bevy::kb::markdown_page::MarkDownPage;
use notation_bevy::kb::notes_page::NotesPage;
use notation_bevy::prelude::*;

use crate::index_panel::IndexPanel;

#[derive(Copy, Clone, Debug)]
pub struct ScalePage {
    pub path: &'static str,
    pub scale: Scale,
    pub key: notation_bevy::prelude::Key,
    pub transpose: i8,
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
        let scale = self.scale.clone();
        let key = self.key.clone();
        ui.horizontal(|ui| {
            ui.label("Scale:");
            egui::ComboBox::from_id_source("scale")
            .width(128.0)
            .selected_text(scale.to_ident())
            .show_ui(ui, |ui| {
                for s in Scale::ALL.iter() {
                    if ui.selectable_label(*s == scale, s.to_ident()).clicked() {
                        self.scale = s.clone();
                    }
                }
            });
            ui.label("Key:");
            egui::ComboBox::from_id_source("key")
            .width(64.0)
            .selected_text(key.to_string())
            .show_ui(ui, |ui| {
                for k in scale.get_keys().iter() {
                    if ui.selectable_label(*k == key, k.to_string()).clicked() {
                        self.key = k.clone();
                    }
                }
            });
        });
        ui.separator();
        NotesPage::notes_ui(ui, texts, assets, state, theme, link_evts, self.scale, self.key, self.transpose);
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("play").clicked() {
                link_evts.send(EasyLinkEvent::from(IndexPanel::LINK_MIDI_PLAY));
            }
            if ui.button("stop").clicked() {
                link_evts.send(EasyLinkEvent::from(IndexPanel::LINK_MIDI_STOP));
            }
        });
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
            scale: Default::default(),
            key: Default::default(),
            transpose: 0,
        }
    }
    pub fn make_tab(&self) -> ProtoTab {
        let meta = TabMeta::new(
            self.key.clone(), self.scale.clone(),
            Signature::_4_4, Tempo::Bpm(60),
        );
        let mut entries = vec![];
        let duration = Duration::_1_4;
        let mut add_note = |syllable: &Syllable, semitones: Option<Semitones>| {
            let note = Note::from(
                Semitones::from(Octave::CENTER)
                + Semitones::from(self.key.clone())
                + Semitones::from(self.scale.calc_syllable_for_sort(syllable))
                + semitones.unwrap_or(Semitones(0))
            );
            entries.push(ProtoEntry::from(CoreEntry::from((Tone::from(note), duration))));
        };
        let mut syllables = self.scale.get_syllables();
        for syllable in syllables.iter() {
            add_note(syllable, None);
        }
        add_note(&self.scale.calc_root_syllable(), Some(Semitones(12)));
        add_note(&self.scale.calc_root_syllable(), Some(Semitones(12)));
        syllables.reverse();
        for syllable in syllables.iter() {
            add_note(syllable, None);
        }
        let new_bar = |index: usize| {
            ProtoBar::new(
                vec![
                    ProtoBarLayer::new("notes".to_owned(), vec![
                        Slice::new(SliceBegin::Index(index), SliceEnd::Count(4), None),
                    ])
                ],
            )
        };
        let track = ProtoTrack::new("notes".to_owned(), TrackKind::Vocal, entries);
        let bars = vec![
            new_bar(0),
            new_bar(4),
            new_bar(8),
            new_bar(12),
        ];
        let section = ProtoSection::new("notes".to_owned(), SectionKind::Rest, bars);
        ProtoTab::new(
            ProtoTab::new_uuid().as_str(),
            meta,
            vec![track],
            vec![section],
            ProtoForm{ sections: vec!["notes".to_owned()]},
        )
    }
    pub fn check_reload(&self, tab: &Tab) -> bool {
        self.scale != tab.meta.scale || self.key != tab.meta.key
    }
}
