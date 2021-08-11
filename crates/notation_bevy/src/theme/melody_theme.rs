use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use notation_model::prelude::{Note, Octave, Semitones, SyllableNote};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MelodyTheme {
    pub syllable_mode: bool,
    pub center_y: f32,
    pub note_height: f32,
    pub note_outline: f32,
    pub note_outline_color: Color,
    pub semitone_height: f32,
}

impl Default for MelodyTheme {
    fn default() -> Self {
        Self {
            syllable_mode: true,
            center_y: -18.0,
            note_height: 3.0,
            note_outline: 1.0,
            note_outline_color: Color::hex("AAAAAA").unwrap(),
            semitone_height: 1.0,
        }
    }
}

impl MelodyTheme {
    pub fn calc_note_y(&self, note: Note, syllable_note: SyllableNote) -> f32 {
        let center_octave = Octave::default(); //TODO
        let center_semitons = Semitones::from(center_octave);
        let offset_semitones = if self.syllable_mode {
            Semitones::from(syllable_note)
        } else {
            Semitones::from(note)
        } - center_semitons;
        self.center_y + self.semitone_height * offset_semitones.0 as f32
    }
}
