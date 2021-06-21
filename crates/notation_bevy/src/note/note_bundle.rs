use bevy::prelude::*;

use notation_core::prelude::{Octave, Pitch, Syllable};

use crate::entry::entry_bundle::EntryBundle;

#[derive(Bundle)]
pub struct NoteBundle {
    pub syllable: Syllable,
    pub pitch: Pitch,
    pub octave: Octave,
    #[bundle]
    pub entry: EntryBundle,
}
