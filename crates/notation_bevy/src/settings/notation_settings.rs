pub struct NotationSettings {
    pub window_width: f32,
    pub window_height: f32,
    pub play_speed: f32,
    pub bars_in_row: u8,
    pub always_show_fret: bool,
    pub melody_piano_mode: bool,
}

impl Default for NotationSettings {
    fn default() -> Self {
        Self {
            window_width: 1280.0,
            window_height: 720.0,
            play_speed: 1.0,
            bars_in_row: 4,
            always_show_fret: false,
            melody_piano_mode: false,
        }
    }
}
