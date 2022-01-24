use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ThemeZ {
    pub beat: f32,
    pub grid: f32,
    pub string: f32,
    pub tone: f32,
    pub word: f32,
    pub pick: f32,
    pub bar_separator: f32,
    pub bar_indicator: f32,
    pub pos_indicator: f32,
    pub tab_control: f32,
    pub play_panel: f32,
    pub play_button: f32,
    pub guitar_view: f32,
    pub guitar_string: f32,
    pub guitar_capo: f32,
    pub guitar_barre: f32,
    pub tab_header: f32,
    pub rhythm_bar: f32,
    pub chord_diagram: f32,
    pub chord_note: f32,
    pub chord_text: f32,
    pub mini_map: f32,
    pub mini_bar: f32,
}

impl Default for ThemeZ {
    fn default() -> Self {
        Self {
            beat: 0.0,
            grid: 1.0,
            string: 1.0,
            tone: 8.0,
            word: 9.0,
            pick: 10.0,
            bar_separator: 2.0,
            bar_indicator: 19.0,
            pos_indicator: 20.0,
            tab_control: 20.0,
            play_panel: 25.0,
            play_button: 26.0,
            guitar_view: 20.0,
            guitar_string: 23.0,
            guitar_capo: 24.0,
            guitar_barre: 24.0,
            tab_header: 20.0,
            rhythm_bar: 22.0,
            chord_diagram: 22.0,
            chord_note: 24.0,
            chord_text: 26.0,
            mini_map: 40.0,
            mini_bar: 42.0,
        }
    }
}
