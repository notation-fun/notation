use std::path::PathBuf;
use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::prelude::{NotationState, NotationAssets, NotationTheme};

use super::kb_page::{KbPage};
use notation_bevy_utils::prelude::{MarkDownAsset, easy_mark, EasyLinkEvent};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct MarkDownPage {
    path: &'static str,
}

impl KbPage for MarkDownPage {
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        Self::markdown_ui(ui, texts, assets, state, theme, link_evts, self.path);
    }
}

impl MarkDownPage {
    pub fn new(path: &'static str) -> Self {
        Self { path }
    }
    pub fn markdown_ui(
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        _state: &NotationState,
        _theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
        path: &str,
    ) {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        if let Some(handle) = assets.get_kb(path_buf) {
            if let Some(text) = texts.get(handle) {
                easy_mark(ui, text.text.as_str(), link_evts);
                return;
            }
        }
        ui.label(format!("Asset Not Found: {}", path));
    }
}