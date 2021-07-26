pub mod midi_events;
pub mod midi_hub;
pub mod midi_plugin;
pub mod midi_util;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::midi_events::{PlayToneEvent, StopToneEvent};
    #[doc(hidden)]
    pub use crate::midi_hub::MidiHub;
    #[doc(hidden)]
    pub use crate::midi_plugin::MidiPlugin;
    #[doc(hidden)]
    pub use crate::midi_util::MidiUtil;
}
