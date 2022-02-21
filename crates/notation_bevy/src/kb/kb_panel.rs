use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use bevy_egui::EguiContext;
use notation_bevy_utils::asset::markdown_asset::MarkDownAsset;
use notation_bevy_utils::prelude::EasyLinkEvent;

use crate::prelude::{NotationState, NotationAssets, NotationTheme, KbPage, DockSide};

use super::kb_page::KbPageId;

pub trait KbPanel {
    fn window_id() -> egui::Id {
        egui::Id::new("KbPanel")
    }
    fn get_title(&self) -> &str {
        "Fun Notation - Knowledge Base"
    }
    fn get_current_page_id(&self) -> KbPageId;
    fn set_current_page_id(&mut self, page_id: KbPageId);
    fn get_page_tabs(&self) -> Vec<(KbPageId, &'static str)>;
    fn get_page_mut(&mut self, page_id: KbPageId) -> &mut dyn KbPage;
    fn on_close(&mut self) {}
    fn topic_tabs_ui(&mut self, ui: &mut Ui) {
        let mut new_current = None;
        let current_page_id = self.get_current_page_id();
        for (page_id, tab_label) in self.get_page_tabs() {
            if ui
                .selectable_label(page_id == current_page_id, tab_label)
                .clicked()
            {
                new_current = Some(page_id.clone());
            }
        }
        if let Some(current) = new_current {
            self.set_current_page_id(current);
        }
    }
    fn kb_panel_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &mut NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        ui.horizontal(|ui| {
            self.topic_tabs_ui(ui);
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            let page_id = self.get_current_page_id();
            self.get_page_mut(page_id)
                .page_ui(ui, texts, assets, state, theme, link_evts);
        });
    }
    fn window_ui(
        &mut self,
        egui_ctx: &mut EguiContext,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &mut NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        if !state.show_kb {
            return;
        }
        let mut window_open = true;
        let mut window = egui::Window::new(self.get_title())
            .collapsible(false)
            .id(Self::window_id());
        window = window.open(&mut window_open);
        window.show(egui_ctx.ctx_mut(), |ui| {
            self.kb_panel_ui(ui, texts, assets, state, theme, link_evts);
        });
        if !window_open {
            self.on_close();
            state.show_kb = false;
        }
    }
    fn side_ui(
        &mut self,
        egui_ctx: &mut EguiContext,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &mut NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
        side: DockSide,
        size: (f32, f32),
    ) {
        if !state.show_kb {
            return;
        }
        match side {
            DockSide::Top =>
                egui::TopBottomPanel::top(self.get_title())
                    .min_height(size.0)
                    .max_height(size.1)
                    .show(egui_ctx.ctx_mut(), |ui|{
                        self.kb_panel_ui(ui, texts, assets, state, theme, link_evts);
                    }),
            DockSide::Bottom =>
                egui::TopBottomPanel::bottom(self.get_title())
                    .min_height(size.0)
                    .max_height(size.1)
                    .show(egui_ctx.ctx_mut(), |ui|{
                        self.kb_panel_ui(ui, texts, assets, state, theme, link_evts);
                    }),
            DockSide::Left =>
                egui::SidePanel::left(self.get_title())
                    .min_width(size.0)
                    .max_width(size.1)
                    .show(egui_ctx.ctx_mut(), |ui|{
                        self.kb_panel_ui(ui, texts, assets, state, theme, link_evts);
                    }),
            DockSide::Right =>
                egui::SidePanel::right(self.get_title())
                    .min_width(size.0)
                    .max_width(size.1)
                    .show(egui_ctx.ctx_mut(), |ui|{
                        self.kb_panel_ui(ui, texts, assets, state, theme, link_evts);
                    }),
        };
    }
}
