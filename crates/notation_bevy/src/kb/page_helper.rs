use bevy_egui::egui::color_picker::show_color;
use bevy_egui::egui::{Ui};
use notation_bevy_utils::egui::{label_from_style, EasyMarkStyle};
use notation_bevy_utils::prelude::{BevyUtil};
use notation_model::prelude::{Interval, Key, Pitch, Scale, Semitones, Syllable};

use crate::prelude::NotationTheme;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct PageHelper {}

impl PageHelper {
    pub fn add_strong_text(ui: &mut Ui, text: &String) {
        let strong_style = EasyMarkStyle {
            strong: true,
            ..EasyMarkStyle::default()
        };
        ui.add(label_from_style(text.as_str(), &strong_style));
    }
    pub fn add_maybe_strong_text(ui: &mut Ui, strong: bool, text: &String) {
        if strong {
            Self::add_strong_text(ui, text);
        } else {
            ui.label(text);
        }
    }
    pub fn add_scale_key(ui: &mut Ui, scale: &Scale, key: &Key) {
        ui.horizontal(|ui| {
            ui.label("Scale:");
            Self::add_strong_text(ui, &scale.to_string());
            ui.label("Key:");
            Self::add_strong_text(ui, &key.to_string());
        });
    }
    pub fn add_syllable_color(ui: &mut Ui, theme: &NotationTheme, syllable: &Syllable) {
        let color = BevyUtil::rgb_to_egui(&theme.colors.of_syllable(syllable.clone()));
        show_color(ui, color, ui.spacing().interact_size);
    }
    pub fn add_syllable(
        ui: &mut Ui,
        theme: &NotationTheme,
        with_color: bool,
        syllable: &Syllable,
        show_ident: bool,
        strong: bool,
    ) {
        if with_color {
            Self::add_syllable_color(ui, theme, syllable);
        }
        let text = if show_ident {
            syllable.to_ident()
        } else {
            syllable.to_string()
        };
        Self::add_maybe_strong_text(ui, strong, &text);
    }
    pub fn add_syllable_pitch(
        ui: &mut Ui,
        _theme: &NotationTheme,
        scale: &Scale,
        key: &Key,
        syllable: &Syllable,
        strong: bool,
    ) {
        let pitch = scale.calc_pitch(&key, &syllable);
        let text = pitch.to_string();
        Self::add_maybe_strong_text(ui, strong, &text);
    }
    pub fn add_syllable_pitch_with_transpose(
        ui: &mut Ui,
        theme: &NotationTheme,
        scale: &Scale,
        key: &Key,
        transpose: i8,
        syllable: &Syllable,
        strong: bool,
    ) {
        let key = Key::from(Semitones::from(*key) - Semitones(transpose));
        Self::add_syllable_pitch(ui, theme, scale, &key, syllable, strong);
    }
    pub fn add_interval_syllable(
        ui: &mut Ui,
        theme: &NotationTheme,
        with_color: bool,
        root: &Syllable,
        interval: &Interval,
        strong: bool,
    ) {
        let syllable = Syllable::from(Semitones::from(*root) + Semitones::from(*interval));
        Self::add_syllable(ui, theme, with_color, &syllable, false, strong);
    }
    pub fn add_interval(
        ui: &mut Ui,
        _theme: &NotationTheme,
        interval: &Interval,
        show_ident: bool,
        strong: bool,
    ) {
        let text = if show_ident {
            interval.to_ident()
        } else {
            interval.to_string()
        };
        Self::add_maybe_strong_text(ui, strong, &text);
    }
}
