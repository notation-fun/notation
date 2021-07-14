use notation_model::prelude::{Tab, Units};

pub enum PlayState {
    Playing,
    Paused,
    Stopped,
}
impl PlayState {
    /// Returns `true` if the play_state is [`Playing`].
    pub fn is_playing(&self) -> bool {
        matches!(self, Self::Playing)
    }
    /// Returns `true` if the play_state is [`Stopped`].
    pub fn is_stopped(&self) -> bool {
        matches!(self, Self::Stopped)
    }
    /// Returns `true` if the play_state is [`Paused`].
    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Paused)
    }
}
impl Default for PlayState {
    fn default() -> Self {
        Self::Playing
    }
}

pub struct TabState {
    pub pos: Units,
    pub begin_pos: Units,
    pub end_pos: Units,
    pub play_state: PlayState,
    pub play_speed: f32,
}

impl TabState {
    pub fn new(tab: &Tab) -> Self {
        let end_pos = Units(tab.bar_units().0 * tab.bars.len() as f32);
        Self {
            pos: Units::from(0.0),
            begin_pos: Units::from(0.0),
            end_pos,
            play_state: PlayState::default(),
            play_speed: 0.4,
        }
    }
    pub fn play(&mut self) -> bool {
        if self.play_state.is_playing() {
            false
        } else {
            self.play_state = PlayState::Playing;
            true
        }
    }
    pub fn pause(&mut self) -> bool {
        if self.play_state.is_paused() {
            false
        } else {
            self.play_state = PlayState::Paused;
            true
        }
    }
    pub fn stop(&mut self) -> bool {
        if self.play_state.is_stopped() {
            false
        } else {
            self.play_state = PlayState::Stopped;
            true
        }
    }
    pub fn tick(&mut self, delta_seconds: f32) -> bool {
        if self.play_state.is_playing() {
            let mut pos = self.pos.0 + delta_seconds * self.play_speed;
            if pos > self.end_pos.0 {
                pos = self.begin_pos.0 + pos - self.end_pos.0;
                if pos > self.end_pos.0 {
                    self.stop();
                }
            }
            self.pos = Units::from(pos);
            true
        } else {
            false
        }
    }
}
