use bevy_egui::egui::Label;

pub mod easy_link;

pub use crate::easy_mark::*;
pub use crate::easy_mark::easy_mark_parser::Style as EasyMarkStyle;

pub use easy_link::{EasyLink, EasyLinkEvent};

pub fn label_from_style(text: &str, style: &EasyMarkStyle) -> Label {
    Label::new(easy_mark_viewer::rich_text_from_style(text, &style))
}
