pub mod duration;
pub mod note;
pub mod scale;
pub mod solfege;
pub mod interval;
pub mod chord;
pub mod rhythm;
pub mod entry;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::duration::{Unit, Units, Duration};
    #[doc(hidden)]
    pub use crate::note::{PitchName, PitchSign, Pitch, Octave, Semitones, Note};
    #[doc(hidden)]
    pub use crate::scale::{Scale, Key};
    #[doc(hidden)]
    pub use crate::solfege::{Syllable, Solfege};
    #[doc(hidden)]
    pub use crate::chord::{ChordQuality, ChordInversion, Chord, Roman};
    #[doc(hidden)]
    pub use crate::rhythm::{Tempo, Bpm, BpmRange, Signature, Beats};
    #[doc(hidden)]
    pub use crate::entry::{Entry, CoreEntry};
}