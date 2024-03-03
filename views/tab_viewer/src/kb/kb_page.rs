use edger_bevy::bevy_prelude::*;
use edger_bevy::egui::Ui;
use edger_bevy::prelude::{MarkDownAsset, EasyLinkEvent};

use crate::prelude::{NotationState, NotationAssets, NotationTheme};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum KbPageId {
    Notes,
    Chords,
    MarkDown(&'static str),
    Custom(&'static str),
}
impl Default for KbPageId {
    fn default() -> Self {
        Self::Notes
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
        link_evts: &mut EventWriter<EasyLinkEvent>,
    );
}

pub trait KbContent {
    fn content_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    );
}