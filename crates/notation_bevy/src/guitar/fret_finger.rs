use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{BevyUtil, LayoutSize};
use notation_model::prelude::{Chord, Finger, Fretboard6, HandShape6, Interval, ModelEntryProps, Syllable, TabMeta};

use crate::{chord::chord_note::{ChordNote, ChordNoteData, ChordNoteExtra, ChordNoteValue}, prelude::{NotationTheme}};

#[derive(Clone, Debug)]
pub struct FretFingerExtra {
    pub string: u8,
    pub fret: Option<u8>,
    pub finger: Option<Finger>,
    pub capo: u8,
    pub in_chord: bool,
    pub guitar_size: LayoutSize,
}

pub type FretFingerData = ChordNoteData<FretFingerExtra>;
pub type FretFinger<'a> = ChordNote<'a, FretFingerExtra>;

impl FretFingerExtra {
    pub fn new(string: u8, fret: Option<u8>, finger: Option<Finger>) -> Self {
        Self {
            string,
            fret,
            finger,
            capo: 0,
            in_chord: false,
            guitar_size: LayoutSize::ZERO,
        }
    }
    pub fn should_hide(&self, _default_fret: u8) -> bool {
        false
        //self.fret.is_some() && self.fret.unwrap() == default_fret
    }
}

impl FretFingerData {
    pub fn new_data(
        entry_props: ModelEntryProps,
        root: Syllable,
        interval: Interval,
        string: u8,
        fret: Option<u8>,
        finger: Option<Finger>,
    ) -> Self {
        let extra = FretFingerExtra::new(string, fret, finger);
        Self::from((
            entry_props,
            ChordNoteValue::<FretFingerExtra>::new(root, interval, extra),
        ))
    }
    pub fn update(&mut self,
        shape: &HandShape6,
        fretboard: Option<Fretboard6>,
        chord: Option<Chord>,
        meta: Option<Arc<TabMeta>>,
    ) {
        self.value.extra.capo = 0;
        self.value.root = Syllable::Fi;
        self.value.interval = Interval::Tritone;
        self.value.extra.in_chord = false;
        self.value.extra.fret = shape.string_fret(self.value.extra.string);

        if let Some(fretboard) = fretboard {
            self.value.extra.capo = fretboard.capo;
            let note = fretboard.shape_note(shape, self.value.extra.string);
            if let (Some(chord), Some(meta)) = (chord, meta.clone()) {
                if let Some(note) = note {
                    self.value.root = chord.root;
                    let syllable_note = meta.calc_syllable_note(&note);
                    if let Some(interval) = chord.calc_interval(syllable_note.syllable) {
                        self.value.interval = interval;
                        self.value.extra.in_chord = true;
                    } else {
                        self.value.interval = Interval::from((chord.root, syllable_note.syllable));
                        println!("FretFingerData.update(): not in chord: {}, {} - {}: {} -> {} -> {} {}", shape, self.value.extra.string, chord, note, syllable_note, self.value.extra.in_chord, self.value.interval);
                    }
                }
            } else {
                if let (Some(note), Some(meta)) = (note, meta) {
                    let syllable_note = meta.calc_syllable_note(&note);
                    self.value.root = syllable_note.syllable;
                    println!("FretFingerData.update(): chord not found: {}, {}", shape, self.value.extra.string);
                } else {
                    println!("FretFingerData.update(): chord and meta not found: {}, {}", shape, self.value.extra.string);
                }
            }
        } else {
            println!("FretFingerData.update(): fretboard not found: {}, {}", shape, self.value.extra.string);

        }
    }
}

impl ChordNoteExtra for FretFingerExtra {
    fn set_diagram_radius(&mut self, _diagram_radius: f32) {}
    fn radius(&self, theme: &NotationTheme) -> f32 {
        theme.guitar.string_x_factor * self.guitar_size.width / 2.0
    }
    fn offset(&self, theme: &NotationTheme) -> Vec2 {
        //TODO: calc with fretboard information for muted fret
        if self.should_hide(0) || self.guitar_size.width <= 0.0 {
            return BevyUtil::offscreen_offset();
        }
        let x = theme.guitar.calc_string_x(self.string, self.guitar_size.width);
        let fret = self.fret.unwrap_or(0);
        let y = theme.guitar.calc_fret_y(fret + self.capo, self.guitar_size.height);
        Vec2::new(x, y)
    }
    fn get_color(&self, theme: &NotationTheme, color: Color) -> Color {
        if self.fret.is_none() {
            theme.shapes.shape_finger_mute_color
        } else {
            color
        }
    }
    fn get_z(&self, theme: &NotationTheme) -> f32 {
        theme.core.mini_bar_z + 2.0
    }
    fn show_dots(&self) -> bool {
        self.in_chord
    }
}