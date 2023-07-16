pub mod guitar;
pub mod tuning;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::guitar::*;
    #[doc(hidden)]
    pub use crate::tuning::GuitarTuning;
}
