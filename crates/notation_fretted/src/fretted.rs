use serde::{Serialize, Deserialize};

use notation_core::prelude::{Note, Semitones};
use super::hand::HandShape;

pub trait Fretted<const S: usize> : Copy + Clone + PartialEq + Eq {
    fn string_num(&self) -> usize;
    fn fret_num(&self) -> usize;
    fn open_notes(&self) -> [Note; S];
    fn open_note(&self, string: usize) -> Option<Note>;
    fn fretted_note(&self, string: usize, fret: u8) -> Option<Note> {
        if fret as usize >= self.fret_num() {
            None
        } else if fret == 0 {
            self.open_note(string)
        } else {
            self.open_note(string)
            .map(|note| (Semitones::from(note) + Semitones(fret as i8)).into())
        }
    }
    fn shape_notes(&self, shape: HandShape<S>) -> [Option<Note>; S] {
        let mut notes = self.open_notes().map(|x| Some(x));
        for (index, note) in notes.iter_mut().enumerate() {
            *note = match shape.string_fret(index as u8) {
                None => None,
                Some (0) => *note,
                Some (fret) => {
                    let open_note = (*note).unwrap();
                    Some((Semitones::from(open_note) + Semitones(fret as i8)).into())
                }
            }
        }
        notes
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Fretboard<const S: usize> {
    #[serde(with = "serde_arrays")]
    pub strings: [Note; S],
    pub fret_num: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct WithCapo<const S: usize> {
    pub fretboard: Fretboard<{ S }>,
    pub capo: u8,
}

impl<const S: usize> From<(Fretboard<S>, u8)> for WithCapo<S> {
    fn from(v: (Fretboard<S>, u8)) -> Self {
        Self {
            fretboard: v.0,
            capo: v.1,
        }
    }
}

impl<const S: usize> Fretted<S> for Fretboard<S> {
    fn string_num(&self) -> usize {
        self.strings.len()
    }
    fn fret_num(&self) -> usize {
        self.fret_num
    }
    fn open_notes(&self) -> [Note; S] {
        self.strings.clone()
    }
    fn open_note(&self, string: usize) -> Option<Note> {
        if string >= self.strings.len() {
            None
        } else {
            Some(self.strings[string])
        }
    }
}

impl<const S: usize> Fretted<S> for WithCapo<S> {
    fn string_num(&self) -> usize {
        self.fretboard.strings.len()
    }
    fn fret_num(&self) -> usize {
        self.fretboard.fret_num - self.capo as usize
    }
    fn open_notes(&self) -> [Note; S] {
        self.fretboard.strings.clone()
    }
    fn open_note(&self, string: usize) -> Option<Note> {
        self.fretboard.open_note(string)
            .map(|note| (Semitones::from(note) + Semitones(self.capo as i8)).into())
    }
}
