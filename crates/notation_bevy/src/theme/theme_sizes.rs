use notation_bevy_utils::prelude::LayoutSize;
use notation_model::prelude::{LaneKind, Note, PlayingState, Semitones, Tab, TrackKind};
use serde::{Deserialize, Serialize};

use crate::prelude::NotationSettings;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
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

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ThemeSizes {
    pub bar: BarSizes,
    pub chord: ChordSizes,
    pub melody: NotesSizes,
    pub harmony: NotesSizes,
    pub lyrics: LyricsSizes,
    pub strings: StringsSizes,
    pub mini_map: MiniMapSizes,
    pub tab_control: TabControlSizes,
    pub layout: LayoutSizes,
}

impl Default for ThemeSizes {
    fn default() -> Self {
        Self {
            bar: Default::default(),
            chord: Default::default(),
            melody: Default::default(),
            harmony: NotesSizes::default_harmony(),
            lyrics: Default::default(),
            strings: Default::default(),
            mini_map: Default::default(),
            tab_control: Default::default(),
            layout: Default::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct BarSizes {
    pub beat_size_range: (f32, f32),
    pub bar_separator_extra: f32,
    pub bar_separator_size: f32,
    pub bar_beat_extra: f32,
    pub pos_indicator_size: f32,
    pub pos_indicator_outline: f32,
    pub pos_indicator_extra: f32,
    pub grid_line_width: f32,
    pub grid_root_line_width: f32,
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
            grid_line_width: 0.5,
            grid_root_line_width: 2.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
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
pub struct NotesSizes {
    pub note_height: f32,
    pub note_outline: PlayingSize,
    pub semitone_height: f32,
    pub lowest: Semitones,
    pub highest: Semitones,
    pub top_margin: f32,
    pub bottom_margin: f32,
}
impl Default for NotesSizes {
    fn default() -> Self {
        Self {
            note_height: 4.0,
            note_outline: PlayingSize::new(1.0, 1.5, 1.0),
            semitone_height: 2.0,
            lowest: Semitones(i8::MAX),
            highest: Semitones(i8::MIN),
            top_margin: 2.0,
            bottom_margin: 16.0,
        }
    }
}
impl NotesSizes {
    pub fn default_harmony() -> Self {
        Self {
            note_height: 4.0,
            semitone_height: 5.0,
            top_margin: 12.0,
            bottom_margin: 4.0,
            ..Default::default()
        }
    }
    pub fn update_with_tab_vocal(&mut self, tab: &Tab) {
        let default = Self::default();
        self.lowest = default.lowest;
        self.highest = default.highest;
        if let Some(track) = tab.get_track_of_kind(TrackKind::Vocal) {
            for entry in track.entries.iter() {
                if let Some(entry) = entry.proto.as_core() {
                    if let Some(tone) = entry.as_tone() {
                        for note in tone.get_notes() {
                            let v = Semitones::from(note);
                            if v < self.lowest {
                                self.lowest = v
                            }
                            if v > self.highest {
                                self.highest = v
                            }
                        }
                    }
                }
            }
            println!(
                "NotesSizes::update_with_tab_vocal: {} - {}",
                self.lowest.0, self.highest.0
            );
        }
    }
    pub fn update_with_tab_guitar(
        &mut self,
        tab: &Tab,
        track_index: Option<usize>,
    ) {
        let default = Self::default();
        self.lowest = default.lowest;
        self.highest = default.highest;
        for bar in tab.bars.iter() {
            if let Some(lane) = bar.get_lane_of_kind(LaneKind::Strings, track_index) {
                for entry in lane.entries.iter() {
                    if let Some(fretted_entry) = entry.model.proto.as_fretted6() {
                        if let Some(pick) = fretted_entry.as_pick() {
                            if let Some((fretboard, shape)) = bar.get_fretted_shape6(entry) {
                                let tone = fretboard.pick_tone(&shape, pick);
                                for note in tone.get_notes() {
                                    let v = Semitones::from(note);
                                    if v < self.lowest {
                                        self.lowest = v
                                    }
                                    if v > self.highest {
                                        self.highest = v
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!(
            "NotesSizes::update_with_tab_guitar: {} - {}",
            self.lowest.0, self.highest.0
        );
    }
    pub fn calc_note_y(&self, note: Note) -> f32 {
        let offset_semitones = self.highest - Semitones::from(note);
        let y = -1.0 * self.semitone_height * offset_semitones.0 as f32 - self.note_height;
        y - self.top_margin
    }
    pub fn layout_height(&self, _settings: &NotationSettings) -> f32 {
        let range = if self.highest > self.lowest {
            self.highest.0 - self.lowest.0 + 1
        } else {
            1
        };
        let height = range as f32 * self.semitone_height + self.note_height;
        height + self.top_margin + self.bottom_margin
    }
}
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
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
        self.string_space * 6.0
    }
    pub fn calc_string_y(&self, string: u8) -> f32 {
        -1.0 * self.string_space * (string as f32 - 0.5)
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
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
pub struct LayoutSizes {
    pub page_margin: f32,
    pub bar_margin: f32,
    pub lane_margin: f32,
    pub bar_min_height: f32,
    pub shapes_height: f32,
}

impl Default for LayoutSizes {
    fn default() -> Self {
        Self {
            page_margin: 12.0,
            bar_margin: 16.0,
            lane_margin: 3.0,
            bar_min_height: 24.0,
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
            LaneKind::Lyrics => {
                if settings.hide_lyrics_lane {
                    0.0
                } else {
                    self.lyrics.layout_height()
                }
            }
            LaneKind::Melody => {
                if settings.hide_melody_lane {
                    0.0
                } else {
                    self.melody.layout_height(settings)
                }
            }
            LaneKind::Harmony => {
                if settings.hide_harmony_lane {
                    0.0
                } else {
                    self.harmony.layout_height(settings)
                }
            }
            LaneKind::Strings => {
                if settings.hide_strings_lane {
                    0.0
                } else {
                    self.strings.layout_height()
                }
            }
            LaneKind::Shapes => {
                if settings.hide_shapes_lane {
                    0.0
                } else {
                    self.layout.shapes_height
                }
            }
            _ => 0.0,
        }
    }
}
