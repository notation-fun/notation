pub mod midi_events;
pub mod midi_settings;
pub mod midi_hub;
pub mod midi_plugin;
pub mod midi_util;
pub mod midi_audio_stream;

#[cfg(not(target_arch = "wasm32"))]
pub mod native_midi_synth;

#[cfg(target_arch = "wasm32")]
pub mod wasm_midi_synth;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::midi_events::{PlayToneEvent, StopToneEvent};
    #[doc(hidden)]
    pub use crate::midi_settings::MidiSettings;
    #[doc(hidden)]
    pub use crate::midi_plugin::MidiPlugin;
    #[doc(hidden)]
    pub use crate::midi_util::MidiUtil;
    #[doc(hidden)]
    pub use crate::midi_audio_stream::{AudioBuffer, DoubleAudioBuffer, MidiAudioStream};
    #[doc(hidden)]
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::native_midi_synth::MidiSynth;
    #[cfg(target_arch = "wasm32")]
    pub use crate::wasm_midi_synth::MidiSynth;
    #[doc(hidden)]
    pub use crate::midi_hub::MidiHub;
}
