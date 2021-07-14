use bevy_inspector_egui::{egui, Context};

use notation_model::prelude::Syllable;

pub fn inspector_ui(v: &mut Syllable, ui: &mut egui::Ui, context: &Context) -> bool {
    let mut changed = false;
    egui::ComboBox::from_id_source(context.id())
        .selected_text(format!("{:?}", v))
        .show_ui(ui, |ui| {
            if ui.selectable_label(false, "Do").clicked() {
                *v = Syllable::Do;
                changed = true;
            }
            if ui.selectable_label(false, "Re").clicked() {
                *v = Syllable::Re;
                changed = true;
            }
            if ui.selectable_label(false, "So").clicked() {
                *v = Syllable::So;
                changed = true;
            }
        });
    changed
}