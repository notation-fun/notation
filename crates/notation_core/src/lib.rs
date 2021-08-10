pub mod chord;
pub mod duration;
pub mod entry;
pub mod interval;
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
    pub use crate::chord::{Chord, ChordInversion, ChordQuality};
    #[doc(hidden)]
    pub use crate::duration::{Duration, Unit, Units};
    #[doc(hidden)]
    pub use crate::entry::{CoreEntry, Entry};
    #[doc(hidden)]
    pub use crate::note::Note;
    #[doc(hidden)]
    pub use crate::octave::Octave;
    #[doc(hidden)]
    pub use crate::pitch::{Pitch, PitchName, PitchSign, Semitones};
    #[doc(hidden)]
    pub use crate::rhythm::{Beats, Bpm, BpmRange, Signature, Tempo};
    #[doc(hidden)]
    pub use crate::scale::{Key, Scale};
    #[doc(hidden)]
    pub use crate::syllable::Syllable;
    #[doc(hidden)]
    pub use crate::syllable_note::SyllableNote;
    #[doc(hidden)]
    pub use crate::tone::Tone;
}
