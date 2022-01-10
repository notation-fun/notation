use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::egui::{self, *};

use notation_bevy::kb::markdown_page::MarkDownPage;
use notation_bevy::kb::notes_page::NotesPage;
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme, MarkDownAsset, KbPage, KbContent, EasyLinkEvent, Scale, Key};

#[derive(Copy, Clone, Debug)]
pub struct ScalePage {
    pub path: &'static str,
    pub scale: Scale,
    pub key: Key,
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
            ui.label("Key:");
            egui::ComboBox::from_id_source("key")
            .width(64.0)
            .selected_text(key.to_string())
            .show_ui(ui, |ui| {
                for k in Key::ALL.iter() {
                    if ui.selectable_label(*k == key, k.to_string()).clicked() {
                        self.key = k.clone();
                    }
                }
            });
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
        });
        ui.separator();
        NotesPage::notes_ui(ui, texts, assets, state, theme, link_evts, self.scale, self.key);
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
        }
    }
}
