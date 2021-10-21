use bevy_utils::prelude::LayoutSize;
use notation_model::prelude::{LaneKind, PlayingState};
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

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
            beat_size_range: (64.0, 256.0),
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
            chord_size_range: (48.0, 80.0),
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
}
impl Default for MelodySizes {
    fn default() -> Self {
        Self {
            note_height: 3.0,
            note_outline: PlayingSize::new(0.5, 1.0, 0.5),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LyricsSizes {
    pub line_height: PlayingSize,
}
impl Default for LyricsSizes {
    fn default() -> Self {
        Self {
            line_height: PlayingSize::new(2.0, 3.0, 2.0),
        }
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
            string_space: 10.0,
            note_height: 4.0,
            note_outline: PlayingSize::new(0.5, 1.0, 0.5),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct TabControlSizes {
    pub control_width: f32,
    pub control_height: f32,
    pub dock_top_width: f32,
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
            control_width: 240.0,
            control_height: 80.0,
            dock_top_width: 400.0,
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
            bar_outline: PlayingSize::new(0.5, 2.0, 1.0),
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
    pub margin: f32,
    pub bar_margin: f32,
    pub lane_margin: f32,
    pub shapes_height: f32,
    pub strings_height: f32,
    pub lyrics_height: f32,
    pub melody_height: f32,
}

impl Default for LayoutSizes {
    fn default() -> Self {
        Self {
            margin: 20.0,
            bar_margin: 12.0,
            lane_margin: 2.0,
            shapes_height: 46.0,
            strings_height: 60.0,
            lyrics_height: 20.0,
            melody_height: 36.0,
        }
    }
}
impl LayoutSizes {
    pub fn calc_lane_height(&self, lane_kind: LaneKind) -> f32 {
        match lane_kind {
            LaneKind::Lyrics => self.lyrics_height,
            LaneKind::Melody => self.melody_height,
            LaneKind::Strings => self.strings_height,
            LaneKind::Shapes => self.shapes_height,
            _ => 0.0,
        }
    }
    pub fn bar_margin(&self) -> LayoutSize {
        LayoutSize::new(0.0, self.bar_margin)
    }
}
