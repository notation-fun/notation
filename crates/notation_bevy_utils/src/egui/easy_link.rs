use bevy::prelude::EventWriter;
use bevy_egui::egui::{self, *};
use crate::easy_mark::easy_mark_parser::Style;

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
impl EasyLinkEvent {
    pub fn new(link: String, new_tab: bool) -> Self {
        Self { link, new_tab, }
    }
}
impl From<String> for EasyLinkEvent {
    fn from(link: String) -> Self {
        Self::new(link, false)
    }
}
impl From<&str> for EasyLinkEvent {
    fn from(link: &str) -> Self {
        Self::new(link.to_owned(), false)
    }
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
        let label = super::link_from_style(&text, &style, ui);
        let response = ui.add(label);
        if response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }
        let is_internal = url.starts_with(EasyLink::INTERNAL_LINK_PREFIX);
        let (clicked, new_tab) =
            if response.clicked() {
                let modifiers = ui.ctx().input(|i| i.modifiers);
                (true, modifiers.any())
            } else if response.middle_clicked() {
                (true, true)
            } else {
                (false, false)
            };
        if clicked {
            if is_internal {
                link_evts.send(EasyLinkEvent{
                    link: url.clone(),
                    new_tab,
                });
            } else {
                ui.ctx().output_mut(|o| {
                    o.open_url = Some(egui::output::OpenUrl {
                        url: url.clone(),
                        new_tab,
                    });
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
