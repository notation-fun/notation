pub mod chord;
pub mod duration;
pub mod entry;
pub mod interval;
pub mod note;
pub mod notes;
pub mod rhythm;
pub mod scale;
pub mod solfege;
pub mod solfeges;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::chord::{Chord, ChordInversion, ChordQuality, Roman};
    #[doc(hidden)]
    pub use crate::duration::{Duration, Unit, Units};
    #[doc(hidden)]
    pub use crate::entry::{CoreEntry, Entry};
    #[doc(hidden)]
    pub use crate::note::{Note, Octave, Pitch, PitchName, PitchSign, Semitones};
    #[doc(hidden)]
    pub use crate::notes::Notes;
    #[doc(hidden)]
    pub use crate::rhythm::{Beats, Bpm, BpmRange, Signature, Tempo};
    #[doc(hidden)]
    pub use crate::scale::{Key, Scale};
    #[doc(hidden)]
    pub use crate::solfege::{Solfege, Syllable};
    #[doc(hidden)]
    pub use crate::solfeges::Solfeges;
}
