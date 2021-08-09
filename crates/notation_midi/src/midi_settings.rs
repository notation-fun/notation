pub struct MidiSettings {
    pub use_internal_synth: bool,
}

impl Default for MidiSettings {
    fn default() -> Self {
        Self {
            use_internal_synth: Self::default_use_internal_synth(),
        }
    }
}

impl MidiSettings {
    fn default_use_internal_synth() ->bool {
        #[cfg(not(target_arch = "wasm32"))]
        return true;
        #[cfg(target_arch = "wasm32")]
        return false;
    }
}