use std::path::PathBuf;
use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::prelude::{NotationState, NotationAssets, NotationTheme};

use super::kb_page::{KbPage};
use notation_bevy_utils::asset::markdown_asset::MarkDownAsset;
use notation_bevy_utils::easy_mark::easy_mark;

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
        _state: &NotationState,
        _theme: &NotationTheme,
    ) {
        let mut path_buf = PathBuf::new();
        path_buf.push(self.path);
        if let Some(handle) = assets.get_kb(path_buf) {
            if let Some(text) = texts.get(handle) {
                easy_mark(ui, text.text.as_str());
                return;
            }
        }
        ui.label(format!("Asset Not Found: {}", self.path));
    }
}

impl MarkDownPage {
    pub fn new(path: &'static str) -> Self {
        Self { path }
    }
}