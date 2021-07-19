use crate::prelude::PlayState;
use notation_model::prelude::{BarPosition, Position, Tab, Units};

pub struct TabState {
    pub pos: Position,
    pub begin_bar_ordinal: usize,
    pub end_bar_ordinal: usize,
    pub should_loop: bool,
    pub play_state: PlayState,
    pub play_speed: f32,
}

impl TabState {
    pub fn new(tab: &Tab) -> Self {
        Self {
            pos: Position::new(tab.bar_units()),
            begin_bar_ordinal: 1,
            end_bar_ordinal: tab.bars.len(),
            should_loop: true,
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
            self.pos.set_in_bar(self.pos.bar.bar_ordinal, Units(0.0));
            true
        }
    }
    pub fn stop(&mut self) -> bool {
        if self.play_state.is_stopped() {
            false
        } else {
            self.play_state = PlayState::Stopped;
            self.pos.set_in_bar(self.begin_bar_ordinal, Units(0.0));
            true
        }
    }
    pub fn tick(&mut self, delta_seconds: f32) -> (bool, bool) {
        if self.play_state.is_playing() {
            self.pos.tick(Units(delta_seconds * self.play_speed));
            let end_passed = self.pos.bar.bar_ordinal > self.end_bar_ordinal;
            if end_passed {
                if self.should_loop {
                    self.pos
                        .set_in_bar(self.begin_bar_ordinal, self.pos.bar.in_bar_pos);
                    if self.pos.bar.bar_ordinal > self.end_bar_ordinal {
                        self.stop(); //Corner case for too smal range
                    }
                } else {
                    self.stop();
                }
            }
            (true, end_passed)
        } else {
            (false, false)
        }
    }
    pub fn is_in_range(&self, pos: &BarPosition) -> bool {
        self.end_bar_ordinal >= self.begin_bar_ordinal
            && pos.bar_ordinal >= self.begin_bar_ordinal
            && pos.bar_ordinal <= self.end_bar_ordinal
    }
}
