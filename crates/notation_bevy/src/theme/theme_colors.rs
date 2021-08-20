use notation_model::prelude::{Octave, PlayingState, Semitones, Syllable};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct PlayingColors {
    pub idle: Color,
    pub current: Color,
    pub played: Color,
}
impl PlayingColors {
    pub fn new(idle: Color, current: Color, played: Color) -> Self {
        Self {
            idle,
            current,
            played,
        }
    }
    pub fn of_state(&self, state: &PlayingState) -> Color {
        match state {
            PlayingState::Idle => self.idle,
            PlayingState::Current => self.current,
            PlayingState::Played => self.played,
        }
    }
}
impl Default for PlayingColors {
    fn default() -> Self {
        Self::new(
            color_of_hex("FFFFFF"),
            color_of_hex("FF00FF"),
            color_of_hex("555555"),
        )
    }
}

pub fn color_of_hex(hex: &str) -> Color {
    Color::hex(hex).unwrap()
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ThemeColors {
    pub syllables: SyllableColors,
    pub lyrics: LyricsColors,
    pub sections: SectionColors,
    pub strings: StringsColors,
    pub mini_map: MiniMapColors,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct SyllableColors {
    pub syllables: [Color; 12],
    pub outline: PlayingColors,
}
//https://meyerweb.com/eric/tools/color-blend/
impl Default for SyllableColors {
    fn default() -> Self {
        Self {
            outline: PlayingColors::default(),
            syllables: [
                color_of_hex("E94F4F"), // Do
                color_of_hex("99572C"), // Di, Ra
                color_of_hex("FFEB34"), // Re
                color_of_hex("558C7F"), // Ri, Me
                color_of_hex("59D7FF"), // Mi
                color_of_hex("C31F6E"), // Fa
                color_of_hex("992D42"), // Fi, Se
                color_of_hex("FF8F28"), // So
                color_of_hex("A17C2B"), // Si, Le
                color_of_hex("A3DC5B"), // La
                color_of_hex("5F785A"), // Li, Te
                color_of_hex("7C87E8"), // Ti
            ],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LyricsColors {
    pub line: PlayingColors,
    pub text: PlayingColors,
}
impl Default for LyricsColors {
    fn default() -> Self {
        Self {
            line: PlayingColors::default(),
            text: PlayingColors::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct StringsColors {
    pub outline: PlayingColors,
}
impl Default for StringsColors {
    fn default() -> Self {
        Self {
            outline: PlayingColors::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct SectionColors {
    pub sections: [Color; 6],
}
impl Default for SectionColors {
    fn default() -> Self {
        Self {
            sections: [
                color_of_hex("CC4444"),
                color_of_hex("44CC44"),
                color_of_hex("4444CC"),
                color_of_hex("CCCC44"),
                color_of_hex("44CCCC"),
                color_of_hex("CC44CC"),
            ],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MiniMapColors {
    pub back: Color,
    pub bar_outline: PlayingColors,
}
impl Default for MiniMapColors {
    fn default() -> Self {
        Self {
            back: color_of_hex("FFF9F2"),
            bar_outline: PlayingColors::default(),
        }
    }
}

impl SyllableColors {
    pub fn of_semitones(&self, v: Semitones) -> Color {
        let pos_val = if v.0 >= 0 { v.0 % 12 } else { v.0 % 12 + 12 } as usize;
        self.syllables[pos_val]
    }
    pub fn of_syllable(&self, v: Syllable) -> Color {
        self.of_semitones(Semitones::from(v))
    }
    pub fn of_syllable_octave(&self, v: Syllable, _o: Octave) -> Color {
        self.of_semitones(Semitones::from(v))
    }
}

impl SectionColors {
    pub fn of_section(&self, v: usize) -> Color {
        self.sections[v % self.sections.len()]
    }
}

impl ThemeColors {
    pub fn of_syllable(&self, v: Syllable) -> Color {
        self.syllables.of_syllable(v)
    }
    pub fn of_section(&self, v: usize) -> Color {
        self.sections.of_section(v)
    }
}
