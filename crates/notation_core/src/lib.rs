extern crate notation_base;

pub mod note;
pub mod scale;
pub mod solfege;
pub mod interval;
pub mod chord;
pub mod rhythm;
pub mod form;

pub mod prelude {
    #[doc(hidden)]
    pub use notation_base::entry::{Unit, Units, Duration, Entry};
    #[doc(hidden)]
    pub use notation_base::line::{Line, Slice};
    #[doc(hidden)]
    pub use notation_base::entry_wrap::{EntryWrap, ZeroEntryWrap};

    #[doc(hidden)]
    pub use crate::note::{PitchName, PitchSign, Pitch, Octave, Semitones, Note, NoteEntry};
    #[doc(hidden)]
    pub use crate::scale::{Scale, ScaleEntry, Key, KeyEntry};
    #[doc(hidden)]
    pub use crate::solfege::{Syllable, Solfege, SolfegeEntry};
    #[doc(hidden)]
    pub use crate::chord::{ChordQuality, ChordInversion, Chord, ChordEntry, Roman, RomanEntry};
    #[doc(hidden)]
    pub use crate::rhythm::{Tempo, Bpm, BpmRange, Signature, Beats};
}