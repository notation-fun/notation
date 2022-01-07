#![feature(arc_new_cyclic)]

pub use notation_proto;

pub mod bar;
pub mod bar_lane;
pub mod form;
pub mod lane_entry;
pub mod lane_kind;
pub mod model_entry;
pub mod parse;
pub mod play;
pub mod section;
pub mod tab;
pub mod tab_bar;
pub mod tab_chord;
pub mod track;
pub mod util;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::bar::{Bar, BarLayer};
    #[doc(hidden)]
    pub use crate::bar_lane::{BarLane, BarLaneProps};
    #[doc(hidden)]
    pub use crate::form::Form;
    #[doc(hidden)]
    pub use crate::lane_entry::{LaneEntry, LaneEntryProps};
    #[doc(hidden)]
    pub use crate::lane_kind::LaneKind;
    #[doc(hidden)]
    pub use crate::model_entry::{ModelEntry, ModelEntryProps};
    #[doc(hidden)]
    pub use crate::parse::ParseError;
    #[doc(hidden)]
    pub use crate::play::play_clock::PlayClock;
    #[doc(hidden)]
    pub use crate::play::play_control::{PlayControl, PlaySpeed, TickResult};
    #[doc(hidden)]
    pub use crate::play::play_state::{PlayState, PlayingState};
    #[doc(hidden)]
    pub use crate::play::play_events::*;
    #[doc(hidden)]
    pub use crate::section::Section;
    #[doc(hidden)]
    pub use crate::tab::Tab;
    #[doc(hidden)]
    pub use crate::tab_bar::{TabBar, TabBarProps};
    #[doc(hidden)]
    pub use crate::tab_chord::TabChord;
    #[doc(hidden)]
    pub use crate::track::{Track, TrackProps};
    #[doc(hidden)]
    pub use notation_proto::prelude::Tab as ProtoTab;
    #[doc(hidden)]
    pub use notation_proto::prelude::*;
}
