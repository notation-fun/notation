pub mod entry;
pub mod fretboard;
pub mod hand;
pub mod pick;
pub mod strum;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::entry::{FrettedEntry4, FrettedEntry6};
    #[doc(hidden)]
    pub use crate::fretboard::{Fretboard4, Fretboard6};
    #[doc(hidden)]
    pub use crate::hand::{Finger, HandShape4, HandShape6};
    #[doc(hidden)]
    pub use crate::pick::{Pick, PickNote};
    #[doc(hidden)]
    pub use crate::strum::Strum;
}
