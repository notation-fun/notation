//! Experimental markup language

mod easy_mark_editor;
pub mod easy_mark_parser;
mod easy_mark_viewer;
mod easy_link;

pub use easy_link::{EasyLink, EasyLinkEvent};
pub use easy_mark_editor::EasyMarkEditor;
pub use easy_mark_parser as parser;
pub use easy_mark_parser::Style as EasyMarkStyle;
pub use easy_mark_viewer::{easy_mark, label_from_style};

pub use bevy_egui::egui;

pub struct EguiUtil {}

impl EguiUtil {
    /*
    pub fn on_hover_ui_above(res: egui::Response, add_contents: impl FnOnce(&mut egui::Ui)) -> egui::Response {
        if res.should_show_hover_ui() {
            egui::containers::show_tooltip_at(
                &res.ctx,
                res.id.with("__tooltip"),
                Some(res.rect.left_top()),
                add_contents)
        }
        res
    }
    pub fn on_hover_text_above(res: egui::Response, text: impl ToString) -> egui::Response {
        res.on_hover_ui_at_pointer(|ui| {
            ui.label(text);
        })
    }
     */
}
