pub mod entry;
pub mod entry_wrap;
pub mod line;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::entry::{Unit, Units, Duration, Entry};
    #[doc(hidden)]
    pub use crate::entry_wrap::{EntryWrap, ZeroEntryWrap};
    #[doc(hidden)]
    pub use crate::line::{Line, Slice};
}