pub mod entry;
pub mod fretboard;
pub mod hand;
pub mod pick;
pub mod strum;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::entry::FrettedEntry;
    #[doc(hidden)]
    pub use crate::fretboard::Fretboard;
    #[doc(hidden)]
    pub use crate::hand::{Finger, HandShape};
    #[doc(hidden)]
    pub use crate::pick::{PickNote, Pick};
    #[doc(hidden)]
    pub use crate::strum::Strum;
}
