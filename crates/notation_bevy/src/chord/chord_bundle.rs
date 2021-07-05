use bevy::prelude::*;

use notation_core::prelude::{ChordInversion, ChordQuality, Pitch, Syllable};

use crate::entry::entry_bundle::EntryBundle;

#[derive(Bundle)]
pub struct ChordBundle {
    pub syllable: Syllable,
    pub pitch: Pitch,
    pub quality: ChordQuality,
    pub inversion: ChordInversion,
    #[bundle]
    pub entry: EntryBundle,
}
