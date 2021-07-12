#![feature(arc_new_cyclic)]

pub use notation_proto;

pub mod bar;
pub mod line;
pub mod section;
pub mod tab;
pub mod track;
pub mod util;
pub mod parse;

pub mod prelude {
    #[doc(hidden)]
    pub use notation_proto::prelude::*;
    #[doc(hidden)]
    pub use notation_proto::prelude::Tab as ProtoTab;
    #[doc(hidden)]
    pub use crate::line::{Line, Slice};
    #[doc(hidden)]
    pub use crate::track::Track;
    #[doc(hidden)]
    pub use crate::bar::{Bar, BarLayer};
    #[doc(hidden)]
    pub use crate::section::{Section, Form};
    #[doc(hidden)]
    pub use crate::tab::{Tab, TabBar};
    #[doc(hidden)]
    pub use crate::parse::{ParseError};
}
