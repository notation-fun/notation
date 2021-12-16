use std::sync::Arc;

use bevy::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, LayoutSize, ShapeOp};
use notation_model::prelude::{
    Chord, Finger, Fretboard6, HandShape6, Interval, ModelEntryProps, Note, Pick, Syllable, TabMeta,
};

use crate::chord::chord_note::{ChordNoteData, ChordNoteExtra, ChordNoteValue};
use crate::prelude::{NotationTheme, NotationAssets, NotationSettings};

#[derive(Clone, Debug)]
pub struct FretFingerExtra {
    pub visible: bool,
    pub string: u8,
    pub pick: bool,
    pub fret: Option<u8>,
    pub finger: Option<Finger>,
    pub capo: u8,
    pub in_chord: bool,
    pub guitar_size: LayoutSize,
}

pub type FretFingerData = ChordNoteData<FretFingerExtra>;

impl FretFingerExtra {
    pub fn new(string: u8, pick: bool, fret: Option<u8>, finger: Option<Finger>) -> Self {
        Self {
            visible: false,
            string,
            pick,
            fret,
            finger,
            capo: 0,
            in_chord: false,
            guitar_size: LayoutSize::ZERO,
        }
    }
}

impl FretFingerData {
    pub fn new_data(
        entry_props: ModelEntryProps,
        root: Syllable,
        interval: Interval,
        string: u8,
        pick: bool,
        fret: Option<u8>,
        finger: Option<Finger>,
    ) -> Self {
        let extra = FretFingerExtra::new(string, pick, fret, finger);
        Self::from((
            entry_props,
            ChordNoteValue::<FretFingerExtra>::new(root, interval, extra),
        ))
    }
    pub fn reset(&mut self) {
        self.value.extra.visible = false;
        self.value.extra.capo = 0;
        self.value.root = Syllable::Fi;
        self.value.interval = Interval::Tritone;
        self.value.extra.in_chord = false;
    }
    fn set_chord_note(&mut self, chord: &Chord, meta: &TabMeta, note: &Note) {
        self.value.extra.visible = true;
        self.value.root = chord.root;
        let syllable_note = meta.calc_syllable_note(&note);
        if let Some(interval) = chord.calc_interval(syllable_note.syllable) {
            self.value.interval = interval;
            self.value.extra.in_chord = true;
        } else {
            self.value.interval = Interval::from((chord.root, syllable_note.syllable));
            self.value.extra.in_chord = false;
        }
        //println!("set_chord_note {}, {} -> {} {}", chord, note, self.value.interval, self.value.extra.in_chord);
    }
    fn set_note(&mut self, meta: &TabMeta, note: &Note) {
        self.value.extra.visible = true;
        self.value.extra.in_chord = false;
        let syllable_note = meta.calc_syllable_note(&note);
        self.value.root = syllable_note.syllable;
    }
    fn set_chord_meta_note(
        &mut self,
        chord: Option<Chord>,
        meta: Option<Arc<TabMeta>>,
        note: Option<Note>,
    ) {
        if let Some(note) = note {
            if let (Some(chord), Some(meta)) = (chord, meta.clone()) {
                self.set_chord_note(&chord, &meta, &note);
            } else if let Some(meta) = meta {
                self.set_note(&meta, &note);
            }
        } else if !self.value.extra.pick {
            //Muted
            self.value.extra.visible = true;
            self.value.extra.in_chord = false;
        }
    }
    pub fn update_pick(
        &mut self,
        fretboard: Option<Fretboard6>,
        chord: Option<Chord>,
        pick: Pick,
        meta: Option<Arc<TabMeta>>,
    ) -> bool {
        let visible = self.value.extra.visible;
        let pick_note = pick.get_pick_note(self.value.extra.string);
        if self.value.extra.pick {
            self.reset();
            self.value.extra.fret = pick_note.and_then(|x| x.fret);
            if let Some(fretboard) = fretboard {
                self.value.extra.capo = fretboard.capo;
                let note = pick_note.and_then(|x| {
                    x.fret
                        .and_then(|f| fretboard.fretted_note(self.value.extra.string, f))
                });
                self.set_chord_meta_note(chord, meta, note);
                true
            } else {
                visible != self.value.extra.visible
            }
        } else {
            let pick_fret = pick_note.and_then(|x| x.fret);
            self.value.extra.visible = pick_fret.is_none() || self.value.extra.fret.is_none() || pick_fret.unwrap() > self.value.extra.fret.unwrap();
            visible != self.value.extra.visible
        }
    }
    pub fn update_value(
        &mut self,
        shape: &HandShape6,
        fretboard: Option<Fretboard6>,
        chord: Option<Chord>,
        pick: Option<Pick>,
        meta: Option<Arc<TabMeta>>,
    ) {
        self.reset();
        let pick_note = pick.and_then(|x| x.get_pick_note(self.value.extra.string));
        if self.value.extra.pick {
            self.value.extra.fret = pick_note.and_then(|x| x.fret);
        } else {
            self.value.extra.fret = shape.string_fret(self.value.extra.string);
        }
        if let Some(fretboard) = fretboard {
            self.value.extra.capo = fretboard.capo;
            let note = if self.value.extra.pick {
                pick_note.and_then(|x| x.fret.and_then(|_| fretboard.shape_pick_note(shape, x)))
            } else {
                fretboard.shape_note(shape, self.value.extra.string)
            };
            self.set_chord_meta_note(chord, meta, note);
        }
    }
    pub fn update_with_syllable(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
    ) {
        self.update(commands, theme, entity);
        let scale = theme.guitar.calc_scale(self.value.extra.guitar_size.width);
        if settings.show_guitar_syllable {
            theme.guitar.syllable_text.spawn_scaled_syllable_text(commands, entity, assets, settings, &self.value.calc_syllable(), scale)
        }
    }
}

impl ChordNoteExtra for FretFingerExtra {
    fn set_diagram_radius(&mut self, _diagram_radius: f32) {}
    fn radius(&self, theme: &NotationTheme) -> f32 {
        theme.guitar.string_x_factor * self.guitar_size.width / 2.0
    }
    fn offset(&self, theme: &NotationTheme) -> Vec2 {
        if !self.visible || self.guitar_size.width <= 0.0 {
            return BevyUtil::offscreen_offset_2();
        }
        let x = theme
            .guitar
            .calc_string_x(self.string, self.guitar_size.width);
        let fret = self.fret.unwrap_or(0);
        let y = theme
            .guitar
            .calc_fret_y(fret + self.capo, self.guitar_size.height);
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
        theme.z.chord_note
    }
    fn show_dots(&self) -> bool {
        self.in_chord
    }
}
