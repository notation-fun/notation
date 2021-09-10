pub mod chord;
pub mod chord_symbol;
pub mod core_entry;
pub mod duration;
pub mod entry;
pub mod interval;
pub mod intervals;
pub mod key;
pub mod meta_entry;
pub mod note;
pub mod octave;
pub mod pitch;
pub mod rhythm;
pub mod scale;
pub mod syllable;
pub mod syllable_note;
pub mod tone;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::chord::Chord;
    #[doc(hidden)]
    pub use crate::chord_symbol::{ChordInversion, ChordQuality, ChordSymbol};
    #[doc(hidden)]
    pub use crate::core_entry::CoreEntry;
    #[doc(hidden)]
    pub use crate::duration::{Duration, Unit, Units};
    #[doc(hidden)]
    pub use crate::entry::{EntryPassMode, Entry};
    #[doc(hidden)]
    pub use crate::interval::{Interval, IntervalQuality};
    #[doc(hidden)]
    pub use crate::intervals::Intervals;
    #[doc(hidden)]
    pub use crate::key::Key;
    #[doc(hidden)]
    pub use crate::meta_entry::MetaEntry;
    #[doc(hidden)]
    pub use crate::note::Note;
    #[doc(hidden)]
    pub use crate::octave::Octave;
    #[doc(hidden)]
    pub use crate::pitch::{Pitch, PitchName, PitchSign, Semitones};
    #[doc(hidden)]
    pub use crate::rhythm::{Beats, Bpm, BpmRange, Signature, Tempo};
    #[doc(hidden)]
    pub use crate::scale::Scale;
    #[doc(hidden)]
    pub use crate::syllable::Syllable;
    #[doc(hidden)]
    pub use crate::syllable_note::SyllableNote;
    #[doc(hidden)]
    pub use crate::tone::Tone;
}
