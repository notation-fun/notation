use bevy::prelude::*;

use notation_core::prelude::{Octave, Pitch, Note, Syllable, Solfege};

#[derive(Bundle)]
pub struct NoteBundle {
    pub name: Name,
    pub syllable: Syllable,
    pub pitch: Pitch,
    pub octave: Octave,
}

impl From<Note> for NoteBundle {
    fn from(v: Note) -> Self {
        NoteBundle {
            name: Name::from("Do"), //TODO
            syllable: Syllable::Do, //TODO
            pitch: v.pitch,
            octave: v.octave,
        }
    }
}

impl From<Solfege> for NoteBundle {
    fn from(v: Solfege) -> Self {
        NoteBundle {
            name: Name::from("C"),
            syllable: v.syllable,
            pitch: Pitch::C, //TODO
            octave: v.octave,
        }
    }
}