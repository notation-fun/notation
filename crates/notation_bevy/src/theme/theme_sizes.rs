use notation_bevy_utils::prelude::LayoutSize;
use notation_model::prelude::{LaneKind, PlayingState, Octave, SyllableNote, Semitones};
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::prelude::NotationSettings;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct PlayingSize {
    pub idle: f32,
    pub current: f32,
    pub played: f32,
}
impl PlayingSize {
    pub fn new(idle: f32, current: f32, played: f32) -> Self {
        Self {
            idle,
            current,
            played,
        }
    }
    pub fn of_state(&self, state: &PlayingState) -> f32 {
        match state {
            PlayingState::Idle => self.idle,
            PlayingState::Current => self.current,
            PlayingState::Played => self.played,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ThemeSizes {
    pub bar: BarSizes,
    pub chord: ChordSizes,
    pub melody: MelodySizes,
    pub lyrics: LyricsSizes,
    pub strings: StringsSizes,
    pub mini_map: MiniMapSizes,
    pub tab_control: TabControlSizes,
    pub layout: LayoutSizes,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct BarSizes {
    pub beat_size_range: (f32, f32),
    pub bar_separator_extra: f32,
    pub bar_separator_size: f32,
    pub bar_beat_extra: f32,
    pub pos_indicator_size: f32,
    pub pos_indicator_outline: f32,
    pub pos_indicator_extra: f32,
}
impl Default for BarSizes {
    fn default() -> Self {
        Self {
            beat_size_range: (80.0, 256.0),
            bar_separator_extra: 3.0,
            bar_separator_size: 3.0,
            bar_beat_extra: 0.0,
            pos_indicator_size: 2.0,
            pos_indicator_outline: 0.5,
            pos_indicator_extra: 8.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ChordSizes {
    pub max_chord_rows: usize,
    pub chord_size_range: (f32, f32),
    pub diagram_factor: f32,
    pub diagram_outline: PlayingSize,
    pub diagram_interval_radius_factor: f32,
    pub diagram_interval_offset_factor: f32,
    pub diagram_base_factor: f32,
    pub diagram_base_y_factor: f32,
    pub interval_dot_radius_factor: f32,
    pub interval_dot_big_radius_factor: f32,
    pub interval_dot_outline: f32,
    pub interval_dot_offset_2_factor: f32,
    pub interval_dot_offset_3_4_factor: f32,
    pub interval_dot_offset_5_7_factor: f32,
    pub interval_dot_offset_6_factor: (f32, f32),
}
impl Default for ChordSizes {
    fn default() -> Self {
        Self {
            max_chord_rows: 2,
            chord_size_range: (64.0, 128.0),
            diagram_factor: 0.45,
            diagram_outline: PlayingSize::new(0.5, 2.0, 1.0),
            diagram_interval_radius_factor: 0.33,
            diagram_interval_offset_factor: 0.45,
            diagram_base_factor: 0.25,
            diagram_base_y_factor: 3.4,
            interval_dot_radius_factor: 0.22,
            interval_dot_big_radius_factor: 0.40,
            interval_dot_outline: 0.5,
            interval_dot_offset_2_factor: 0.3,
            interval_dot_offset_3_4_factor: 0.4,
            interval_dot_offset_5_7_factor: 0.6,
            interval_dot_offset_6_factor: (0.4, 0.5),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MelodySizes {
    pub note_height: f32,
    pub note_outline: PlayingSize,
    pub semitone_height: f32,
    pub semitones: usize,
}
impl Default for MelodySizes {
    fn default() -> Self {
        Self {
            note_height: 4.0,
            note_outline: PlayingSize::new(1.0, 1.5, 1.0),
            semitone_height: 2.0,
            semitones: 24,
        }
    }
}
impl MelodySizes {
    pub fn calc_note_y(&self, syllable_note: SyllableNote) -> f32 {
        let center_octave = Octave::default(); //TODO
        let center_semitons = Semitones::from(center_octave);
        let offset_semitones = Semitones::from(syllable_note) - center_semitons;
        let center_y = self.semitones as f32 * self.semitone_height / 2.0;
        -center_y + self.semitone_height * offset_semitones.0 as f32
    }
    pub fn layout_height(&self) -> f32 {
        self.semitones as f32 * self.semitone_height + self.note_height
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LyricsSizes {
    pub line_height: PlayingSize,
    pub word_gap: f32,
}
impl Default for LyricsSizes {
    fn default() -> Self {
        Self {
            line_height: PlayingSize::new(20.0, 24.0, 20.0),
            word_gap: 2.0,
        }
    }
}
impl LyricsSizes {
    pub fn layout_height(&self) -> f32 {
        self.line_height.of_state(&PlayingState::Current)
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct StringsSizes {
    pub string_space: f32,
    pub note_height: f32,
    pub note_outline: PlayingSize,
}
impl Default for StringsSizes {
    fn default() -> Self {
        Self {
            string_space: 12.0,
            note_height: 6.0,
            note_outline: PlayingSize::new(1.0, 1.5, 1.0),
        }
    }
}
impl StringsSizes {
    pub fn layout_height(&self) -> f32 {
        self.string_space * 6.0 + self.note_height
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct TabControlSizes {
    pub control_width_factor: f32,
    pub tab_control_range: (f32, f32),
    pub button_size_range: (f32, f32),
    pub button_scale_factor: f32,
    pub rhythm_bar_radius_factor: f32,
    pub rhythm_bar_radius_extra: f32,
    pub rhythm_beat_radius_factor: f32,
    pub rhythm_beat_offset_factor: f32,
    pub rhythm_beat_max_scale: f32,
    pub rhythm_indicator_radius_factor: f32,
    pub rhythm_indicator_width_factor: f32,
    pub rhythm_indicator_line_width: f32,
}
impl Default for TabControlSizes {
    fn default() -> Self {
        Self {
            control_width_factor: 0.20,
            tab_control_range: (96.0, 512.0),
            button_size_range: (32.0, 64.0),
            button_scale_factor: 0.75,
            rhythm_bar_radius_factor: 0.45,
            rhythm_bar_radius_extra: 2.0,
            rhythm_beat_radius_factor: 0.10,
            rhythm_beat_offset_factor: 0.75,
            rhythm_beat_max_scale: 2.5,
            rhythm_indicator_radius_factor: 1.05,
            rhythm_indicator_width_factor: 0.2,
            rhythm_indicator_line_width: 2.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MiniMapSizes {
    pub bar_height: f32,
    pub bar_width_range: (f32, f32),
    pub bar_margin: (f32, f32),
    pub bar_outline: PlayingSize,
    pub section_separator: f32,
}

impl Default for MiniMapSizes {
    fn default() -> Self {
        Self {
            bar_height: 24.0,
            bar_width_range: (4.0, 1024.0),
            bar_margin: (0.0, 2.0),
            bar_outline: PlayingSize::new(1.0, 2.0, 1.0),
            section_separator: 2.0,
        }
    }
}
impl MiniMapSizes {
    pub fn bar_margin(&self) -> LayoutSize {
        LayoutSize::new(self.bar_margin.0, self.bar_margin.1)
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LayoutSizes {
    pub page_margin: f32,
    pub bar_margin: f32,
    pub lane_margin: f32,
    pub shapes_height: f32,
}

impl Default for LayoutSizes {
    fn default() -> Self {
        Self {
            page_margin: 12.0,
            bar_margin: 16.0,
            lane_margin: 2.0,
            shapes_height: 52.0,
        }
    }
}
impl ThemeSizes {
    pub fn cell_margin(&self, setting: &NotationSettings) -> LayoutSize {
        let height = if setting.layout.video_recording_mode {
            self.layout.bar_margin * 4.0
        } else {
            self.layout.bar_margin
        };
        LayoutSize::new(0.0, height)
    }
    pub fn calc_lane_height(&self, settings: &NotationSettings, lane_kind: LaneKind) -> f32 {
        match lane_kind {
            LaneKind::Lyrics => if settings.hide_lyrics_lane { 0.0 } else { self.lyrics.layout_height() },
            LaneKind::Melody => if settings.hide_melody_lane { 0.0 } else { self.melody.layout_height() },
            LaneKind::Strings => if settings.hide_strings_lane { 0.0 } else { self.strings.layout_height() },
            LaneKind::Shapes => if settings.hide_shapes_lane { 0.0 } else { self.layout.shapes_height },
            _ => 0.0,
        }
    }
}