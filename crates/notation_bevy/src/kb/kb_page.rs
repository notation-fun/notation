use bevy::prelude::*;
use bevy_egui::egui::{Ui};
use notation_bevy_utils::asset::markdown_asset::MarkDownAsset;

use crate::prelude::{NotationState, NotationAssets, NotationTheme};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum KbPageId {
    Welcome,
    Notes,
    Chords,
    Usage,
    Custom(String),
}
impl Default for KbPageId {
    fn default() -> Self {
        Self::Welcome
    }
}

pub trait KbPage {
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
    );
}