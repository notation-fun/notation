use notation_model::prelude::{
    Chord, IntervalQuality, Octave, PlayingState, Semitones, Signature, Syllable,
};
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
            hex_linear("FFFFFF88"),
            hex_linear("000000CC"),
            hex_linear("00000066"),
        )
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct IntervalColors {
    pub perfect: Color,
    pub major: Color,
    pub minor: Color,
    pub augmented: Color,
    pub diminishd: Color,
    pub tritone: Color,
}
impl IntervalColors {
    pub fn of_quality(&self, quality: &IntervalQuality) -> Color {
        match quality {
            IntervalQuality::Perfect => self.perfect,
            IntervalQuality::Major => self.major,
            IntervalQuality::Minor => self.minor,
            IntervalQuality::Augmented => self.augmented,
            IntervalQuality::Diminished => self.diminishd,
            IntervalQuality::Tritone => self.tritone,
        }
    }
}
impl Default for IntervalColors {
    fn default() -> Self {
        Self {
            perfect: hex_linear("FFFFFF"),
            major: hex_linear("FFFFFF"),
            minor: hex_linear("666666"),
            augmented: hex_linear("FF00FFAA"),
            diminishd: hex_linear("66666644"),
            tritone: hex_linear("FF00FFAA"),
        }
    }
}

pub fn hex_linear(hex: &str) -> Color {
    let color = Color::hex(hex).unwrap();
    color.as_rgba_linear()
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ThemeColors {
    pub syllables: SyllableColors,
    pub bar: BarColors,
    pub chord: ChordColors,
    pub lyrics: LyricsColors,
    pub section: SectionColors,
    pub strings: StringsColors,
    pub rhythm: RhythmColors,
    pub mini_map: MiniMapColors,
    pub ui: UiColors,
}

impl ThemeColors {
    pub fn hex_linear(hex: &str) -> Color {
        hex_linear(hex)
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct SyllableColors {
    pub outline: PlayingColors,
    pub syllables: [Color; 12],
    pub no_syllable: Color,
}
//https://meyerweb.com/eric/tools/color-blend/
impl Default for SyllableColors {
    fn default() -> Self {
        Self {
            outline: PlayingColors::default(),
            syllables: [
                hex_linear("EF7071"), // Do
                hex_linear("99572C"), // Di, Ra
                hex_linear("EECB16"), // Re
                hex_linear("558C7F"), // Ri, Me
                hex_linear("94D8FF"), // Mi
                hex_linear("F65EBA"), // Fa
                hex_linear("992D42"), // Fi, Se
                hex_linear("F4A963"), // So
                hex_linear("A17C2B"), // Si, Le
                hex_linear("A3DC5B"), // La
                hex_linear("5F785A"), // Li, Te
                hex_linear("8E99FF"), // Ti
            ],
            no_syllable: hex_linear("888888"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct BarColors {
    pub bar_indicator: Color,
    pub bar_separator_color: Color,
    pub selected_beat_color0: Color,
    pub selected_beat_color1: Color,
    pub selected_beat_color2: Color,
    pub beat_color0: Color,
    pub beat_color1: Color,
    pub beat_color2: Color,
    pub pos_indicator_color: Color,
}
impl Default for BarColors {
    fn default() -> Self {
        Self {
            bar_indicator: hex_linear("000000AA"),
            bar_separator_color: ThemeColors::hex_linear("D3B59C"),
            selected_beat_color0: ThemeColors::hex_linear("FFFFFF88"),
            selected_beat_color1: ThemeColors::hex_linear("FFFFFF44"),
            selected_beat_color2: ThemeColors::hex_linear("FFFFFF88"),
            beat_color0: ThemeColors::hex_linear("00000000"),
            beat_color1: ThemeColors::hex_linear("00000010"),
            beat_color2: ThemeColors::hex_linear("00000000"),
            pos_indicator_color: ThemeColors::hex_linear("00000077"),
        }
    }
}
impl BarColors {
    pub fn get_beat_color(&self, signature: &Signature, beat: u8, selected: bool) -> Color {
        if selected {
            self._get_beat_color(signature, beat, self.selected_beat_color0, self.selected_beat_color1, self.selected_beat_color2)
        } else {
            self._get_beat_color(signature, beat, self.beat_color0, self.beat_color1, self.beat_color2)
        }
    }
    pub fn _get_beat_color(&self, signature: &Signature, beat: u8,
        color0: Color,
        color1: Color,
        color2: Color,
    ) -> Color {
        if beat == 0 {
            return color0;
        }
        if signature.bar_beats % 4 == 0 {
            match beat % 4 {
                1 => color1,
                2 => color2,
                3 => color1,
                _ => color0,
            }
        } else if signature.bar_beats % 3 == 0 {
            match beat % 3 {
                1 => color1,
                2 => color2,
                _ => color0,
            }
        } else {
            match beat % 2 {
                1 => color1,
                _ => color0,
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ChordColors {
    pub background: Color,
    pub diagram_outline: PlayingColors,
    pub dot: IntervalColors,
    pub dot_outline: IntervalColors,
}
impl Default for ChordColors {
    fn default() -> Self {
        Self {
            background: hex_linear("00000055"),
            diagram_outline: PlayingColors::new(
                hex_linear("00000066"),
                hex_linear("FFFFFFAA"),
                hex_linear("00000066"),
            ),
            dot: IntervalColors::default(),
            dot_outline: IntervalColors {
                perfect: hex_linear("000000"),
                major: hex_linear("000000"),
                minor: hex_linear("FFFFFF"),
                augmented: hex_linear("FF00FF"),
                diminishd: hex_linear("333333"),
                tritone: hex_linear("FF00FF"),
            },
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
            line: PlayingColors::new(
                hex_linear("A68A73"),
                hex_linear("804718"),
                hex_linear("795641"),
            ),
            text: PlayingColors::new(
                hex_linear("A68A73"),
                hex_linear("804718"),
                hex_linear("795641"),
            ),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct StringsColors {
    pub outline: PlayingColors,
    pub hit: Color,
    pub muted: Color,
    pub string: PlayingColors,
    pub fret: PlayingColors,
    pub capo: Color,
}
impl Default for StringsColors {
    fn default() -> Self {
        Self {
            outline: PlayingColors::default(),
            hit: hex_linear("FFFFFF"),
            muted: hex_linear("333333"),
            string: PlayingColors::new(
                hex_linear("D3B59C"),
                hex_linear("FFFFFF"),
                hex_linear("D3B59C"),
            ),
            fret: PlayingColors::new(
                hex_linear("000000"),
                hex_linear("000000"),
                hex_linear("555555"),
            ),
            capo: hex_linear("333333"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct SectionColors {
    pub sections: [Color; 12],
}
impl Default for SectionColors {
    fn default() -> Self {
        let saturation = 1.0;
        let lightness = 0.5;
        let alpha = 0.8;
        Self {
            sections: [
                Color::hsla(0.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(1.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(2.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(3.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(4.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(5.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(6.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(7.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(8.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(9.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(10.0 * 30.0, saturation, lightness, alpha),
                Color::hsla(11.0 * 30.0, saturation, lightness, alpha),
            ],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct RhythmColors {
    pub beats: [Color; 3],
    pub indicator: Color,
}
impl Default for RhythmColors {
    fn default() -> Self {
        Self {
            beats: [
                hex_linear("00000077"),
                hex_linear("00000099"),
                hex_linear("00000088"),
            ],
            indicator: hex_linear("00000077"),
        }
    }
}

impl RhythmColors {
    pub fn get_beat_color(&self, signature: &Signature, beat: u8) -> Color {
        if beat == 0 {
            return self.beats[0];
        }
        if signature.bar_beats % 4 == 0 {
            match beat % 4 {
                1 => self.beats[1],
                2 => self.beats[2],
                3 => self.beats[1],
                _ => self.beats[0],
            }
        } else if signature.bar_beats % 3 == 0 {
            match beat % 3 {
                1 => self.beats[1],
                2 => self.beats[2],
                _ => self.beats[0],
            }
        } else {
            match beat % 2 {
                1 => self.beats[1],
                _ => self.beats[0],
            }
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
            back: hex_linear("AAAAAA"),
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
    pub fn of_option_syllable(&self, v: Option<Syllable>) -> Color {
        if let Some(syllable) = v {
            self.of_syllable(syllable)
        } else {
            self.no_syllable
        }
    }
    pub fn of_option_chord(&self, v: Option<Chord>) -> Color {
        if let Some(chord) = v {
            self.of_syllable(chord.root)
        } else {
            self.no_syllable
        }
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
    pub fn of_option_syllable(&self, v: Option<Syllable>) -> Color {
        self.syllables.of_option_syllable(v)
    }
    pub fn of_option_chord(&self, v: Option<Chord>) -> Color {
        self.syllables.of_option_chord(v)
    }
    pub fn of_section(&self, v: usize) -> Color {
        self.section.of_section(v)
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct UiColors {
    pub control_background: Color,
    pub button_on: Color,
    pub button_off: Color,
}
impl Default for UiColors {
    fn default() -> Self {
        Self {
            control_background: hex_linear("FFF9F2"),
            button_on: hex_linear("F27D7A"),
            button_off: hex_linear("888888"),
        }
    }
}
impl UiColors {
    pub fn of_button(&self, on: bool) -> Color {
        if on {
            self.button_on
        } else {
            self.button_off
        }
    }
}
