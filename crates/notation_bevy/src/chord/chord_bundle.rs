use bevy::prelude::*;

use notation_core::prelude::{Pitch, Syllable, ChordQuality, ChordInversion};

use crate::entry::entry_bundle::EntryBundle;

#[derive(Bundle)]
pub struct NoteBundle {
    pub syllable: Syllable,
    pub pitch: Pitch,
    pub quality: ChordQuality,
    pub inversion: ChordInversion,
    #[bundle]
    pub entry: EntryBundle,
}
