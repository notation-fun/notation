use std::fmt::Display;
use std::sync::Arc;

use notation_proto::prelude::{BarPosition, Note, SyllableNote, TrackKind};

use crate::prelude::{
    Form, Pitch, Section, Signature, Syllable, TabBar, TabMeta, Track, Unit, Units,
};

#[derive(Debug)]
pub struct Tab {
    pub meta: Arc<TabMeta>,
    pub tracks: Vec<Arc<Track>>,
    pub sections: Vec<Arc<Section>>,
    pub form: Form,
    pub bars: Vec<Arc<TabBar>>,
}
impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Tab>({} T:{} S:{} F:{} B:{})",
            self.meta,
            self.tracks.len(),
            self.sections.len(),
            self.form.sections.len(),
            self.bars.len(),
        )
    }
}
impl Tab {
    pub fn bar_units(&self) -> Units {
        self.meta.bar_units()
    }
    pub fn bar_beats(&self) -> u8 {
        self.meta.signature.bar_beats
    }
    pub fn signature(&self) -> Signature {
        self.meta.signature
    }
    pub fn beat_unit(&self) -> Unit {
        self.meta.signature.beat_unit
    }
    pub fn calc_syllable(&self, pitch: &Pitch) -> Syllable {
        self.meta.calc_syllable(pitch)
    }
    pub fn calc_syllable_note(&self, note: &Note) -> SyllableNote {
        self.meta.calc_syllable_note(note)
    }
    pub fn get_track_of_kind(&self, kind: TrackKind) -> Option<Arc<Track>> {
        for track in self.tracks.iter() {
            if track.kind == kind {
                return Some(track.clone());
            }
        }
        None
    }
    pub fn get_bar_of_ordinal(&self, bar_ordinal: usize) -> Option<Arc<TabBar>> {
        self.bars
            .get(bar_ordinal - 1)
            .map(|x| x.clone())
    }
    pub fn get_bar(&self, pos: BarPosition) -> Option<Arc<TabBar>> {
        self.get_bar_of_ordinal(pos.bar_ordinal)
    }
}
