use bevy::prelude::EventWriter;
use bevy_egui::egui::{self, *};
use super::easy_mark_parser::Style;

/// A clickable link, e.g. to `"https://github.com/emilk/egui"`.
/// urls starts with colon will be treated as internal link, e.g, `:do_something`
///
/// Based upon egui::Hyperlink, adding features to support internal links,
/// also make the hover text on top
///
/// ui.add(Easylink::new("https://github.com/emilk/egui").text("My favorite repo").small()).on_click(Fn<&str>);
/// ```
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct EasyLink {
    url: String,
    text: String,
    style: Style,
}

#[derive(Clone, Debug)]
pub struct EasyLinkEvent {
    pub link: String,
    pub new_tab: bool,
}

impl EasyLink {
    pub const INTERNAL_LINK_PREFIX: &'static str = ":";
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(url: impl ToString, text: impl ToString, style: Style) -> Self {
        let url = url.to_string();
        Self {
            url: url.to_string(),
            text: text.to_string(),
            style,
        }
    }

    pub fn ui(self, ui: &mut Ui, link_evts: &mut EventWriter<EasyLinkEvent>) -> Response {
        let EasyLink { url, text, style } = self;
        let label = super::easy_mark_viewer::label_from_style(&text, &style);
        let color = ui.visuals().hyperlink_color;
        let label = label.text_color(color);
        let response = ui.add(label.sense(Sense::click()));
        if response.hovered() {
            ui.ctx().output().cursor_icon = CursorIcon::PointingHand;
        }
        let is_internal = url.starts_with(EasyLink::INTERNAL_LINK_PREFIX);
        if response.clicked() {
            let modifiers = ui.ctx().input().modifiers;
            let new_tab = modifiers.any();
            if is_internal {
                link_evts.send(EasyLinkEvent{
                    link: url.clone(),
                    new_tab,
                });
            } else {
                ui.ctx().output().open_url = Some(egui::output::OpenUrl {
                    url: url.clone(),
                    new_tab,
                });
            }
        }
        if response.middle_clicked() {
            let new_tab = true;
            if is_internal {
                link_evts.send(EasyLinkEvent{
                    link: url.clone(),
                    new_tab,
                });
            } else {
                ui.ctx().output().open_url = Some(egui::output::OpenUrl {
                    url: url.clone(),
                    new_tab,
                });
            }
        }
        if is_internal {
            response
        } else {
            response.on_hover_ui_at_pointer(|ui| {
                ui.horizontal(|ui| {
                    ui.separator();
                    ui.label(url);
                });
            })
        }
    }
}
