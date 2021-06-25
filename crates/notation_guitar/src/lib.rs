pub mod tuning;
pub mod guitar;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::guitar::*;
    #[doc(hidden)]
    pub use crate::tuning::{Tuning};
}