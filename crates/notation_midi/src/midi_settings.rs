use midi_msg::GMSoundSet;
use notation_model::prelude::{Octave, TrackKind};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MidiSettings {
    pub bypass_hub: bool,
    pub click_mute: bool,
    pub click_velocity: u8,
    pub click_octave: Octave,
    pub vocal_mute: bool,
    pub vocal_velocity: u8,
    pub guitar_mute: bool,
    pub guitar_velocity: u8,
    pub piano_mute: bool,
    pub piano_velocity: u8,
    pub use_internal_synth: bool,
    pub click_sound: u8,
    pub vocal_sound: u8,
    pub guitar_sound: u8,
    pub piano_sound: u8,
    pub seeking_track: TrackKind,
    pub seeking_init_channel: bool,
}

impl Default for MidiSettings {
    fn default() -> Self {
        Self {
            bypass_hub: false,
            click_mute: true,
            click_velocity: 100,
            click_octave: Octave::P7,
            vocal_mute: false,
            vocal_velocity: 110,
            guitar_mute: false,
            guitar_velocity: 120,
            piano_mute: false,
            piano_velocity: 110,
            use_internal_synth: Self::default_use_internal_synth(),
            click_sound: GMSoundSet::Dulcimer as u8,
            vocal_sound: GMSoundSet::Cello as u8,
            guitar_sound: GMSoundSet::AcousticGuitarSteel as u8,
            piano_sound: GMSoundSet::AcousticGrandPiano as u8,
            seeking_track: TrackKind::Guitar,
            seeking_init_channel: true,
        }
    }
}

impl MidiSettings {
    fn default_use_internal_synth() -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        return true;
        #[cfg(target_arch = "wasm32")]
        return true;
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
    pub fn get_click_channel_params(&self) -> (u8, u8) {
        (self.click_sound, self.click_velocity)
    }
}
