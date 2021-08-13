use midi_msg::GMSoundSet;
use notation_model::prelude::TrackKind;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MidiSettings {
    pub use_internal_synth: bool,
    pub vocal_sound: u8,
    pub vocal_velocity: u8,
    pub guitar_sound: u8,
    pub guitar_velocity: u8,
    pub piano_sound: u8,
    pub piano_velocity: u8,
}

impl Default for MidiSettings {
    fn default() -> Self {
        Self {
            use_internal_synth: Self::default_use_internal_synth(),
            vocal_sound: GMSoundSet::Trumpet as u8,
            vocal_velocity: 90,
            guitar_sound: GMSoundSet::AcousticGuitarSteel as u8,
            guitar_velocity: 80,
            piano_sound: GMSoundSet::AcousticGrandPiano as u8,
            piano_velocity: 80,
        }
    }
}

impl MidiSettings {
    fn default_use_internal_synth() -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        return true;
        #[cfg(target_arch = "wasm32")]
        return false;
    }
}
impl MidiSettings {
    pub fn get_track_channel_params(&self, kind: &TrackKind) -> Option<(u8, u8)> {
        match kind {
            TrackKind::Vocal => Some((self.vocal_sound, self.vocal_velocity)),
            TrackKind::Guitar => Some((self.guitar_sound, self.guitar_velocity)),
            TrackKind::Piano => Some((self.piano_sound, self.piano_velocity)),
            _ => None,
        }
    }
}
