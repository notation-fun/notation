pub mod midi_events;
pub mod midi_hub;
pub mod midi_message;
pub mod midi_plugin;
pub mod midi_settings;
pub mod midi_state;
pub mod midi_util;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub mod prelude {
    #[doc(hidden)]
    #[doc(hidden)]
    pub use crate::midi_events::{PlayControlEvt, SwitchTabEvent};
    #[doc(hidden)]
    pub use crate::midi_hub::MidiHub;
    #[doc(hidden)]
    pub use crate::midi_message::MidiMessage;
    #[doc(hidden)]
    pub use crate::midi_plugin::MidiPlugin;
    #[doc(hidden)]
    pub use crate::midi_settings::MidiSettings;
    #[doc(hidden)]
    pub use crate::midi_state::{MidiChannel, MidiState};
    #[doc(hidden)]
    pub use crate::midi_util::MidiUtil;
    #[doc(hidden)]
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::native::midi_synth::MidiSynth;
    #[cfg(target_arch = "wasm32")]
    pub use crate::wasm::midi_synth::MidiSynth;
}
