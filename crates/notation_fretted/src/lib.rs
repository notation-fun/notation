pub mod entry;
pub mod fretboard;
pub mod hand;
pub mod pick;
pub mod strum;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::entry::{FrettedEntry6, FrettedEntry4};
    #[doc(hidden)]
    pub use crate::fretboard::{Fretboard6, Fretboard4};
    #[doc(hidden)]
    pub use crate::hand::{Finger, HandShape6, HandShape4};
    #[doc(hidden)]
    pub use crate::pick::{Pick, PickNote};
    #[doc(hidden)]
    pub use crate::strum::Strum;
}
