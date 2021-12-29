use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use bevy_egui::EguiContext;
use notation_bevy_utils::asset::markdown_asset::MarkDownAsset;

use crate::prelude::{NotationTheme, NotationAppState, NotationAssets};

use super::chords_page::ChordsPage;
use super::notes_page::NotesPage;
use super::usage_page::UsagePage;
use super::welcome_page::WelcomePage;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HelpPageId {
    Welcome,
    Notes,
    Chords,
    Usage,
}
impl Default for HelpPageId {
    fn default() -> Self {
        Self::Welcome
    }
}

pub trait HelpPage {
    fn page_id(&self) -> HelpPageId;
    fn tab_label(&self) -> &'static str;
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationAppState,
        theme: &NotationTheme
    );
}

#[derive(Clone, Debug, Default)]
pub struct HelpPanel {
    pub open_times: usize,
    pub current_page_id: HelpPageId,
    pub welcome: WelcomePage,
    pub notes: NotesPage,
    pub chords: ChordsPage,
    pub usage: UsagePage,
}

impl HelpPanel {
    pub fn window_id() -> egui::Id {
        egui::Id::new("HelpPanel")
    }
    fn get_pages(&self) -> impl Iterator<Item = &dyn HelpPage> {
        vec![
            &self.welcome as &dyn HelpPage,
            &self.notes as &dyn HelpPage,
            &self.chords as &dyn HelpPage,
            &self.usage as &dyn HelpPage,
        ].into_iter()
    }
    fn get_current_page(&mut self) -> &mut dyn HelpPage {
        match self.current_page_id {
            HelpPageId::Welcome => &mut self.welcome as &mut dyn HelpPage,
            HelpPageId::Notes => &mut self.notes as &mut dyn HelpPage,
            HelpPageId::Chords => &mut self.chords as &mut dyn HelpPage,
            HelpPageId::Usage => &mut self.usage as &mut dyn HelpPage,
        }
    }
    fn topic_tabs_ui(
        &mut self,
        ui: &mut Ui,
    ) {
        let mut new_current = None;
        for page in self.get_pages() {
            if ui.selectable_label(page.page_id() == self.current_page_id, page.tab_label()).clicked() {
                new_current = Some(page.page_id());
            }
        }
        if let Some(current) = new_current {
            self.current_page_id = current;
        }
    }
    fn help_ui_content(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &mut NotationAppState,
        theme: &NotationTheme,
    ) {
        ui.horizontal(|ui| {
            self.topic_tabs_ui(ui);
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            self.get_current_page().page_ui(ui, texts, assets, state, theme);
        });
    }
    pub fn help_ui(
        egui_ctx: Res<EguiContext>,
        texts: Res<Assets<MarkDownAsset>>,
        assets: Res<NotationAssets>,
        mut state: ResMut<NotationAppState>,
        theme: Res<NotationTheme>,
        mut help: ResMut<HelpPanel>,
    ) {
        if !state.show_help {
            return;
        }
        let mut window_open = true;
        let mut window = egui::Window::new("Help (F1, H)")
            .collapsible(false)
            .id(Self::window_id());
        window = window.open(&mut window_open);
        window.show(egui_ctx.ctx(), |ui| {
            (&mut help).help_ui_content(ui, &texts, &assets, &mut state, &theme);
        });
        if !window_open {
            let help_panel = &mut help;
            if help_panel.open_times == 0 && help_panel.current_page_id == HelpPageId::Welcome {
                help_panel.current_page_id = HelpPageId::Notes;
            }
            help_panel.open_times += 1;
            state.show_help = false;
        }
    }
}