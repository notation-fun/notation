use crate::prelude::{BarPosition, Bpm, PlayState, Position, Tab, TabMeta, Units};

#[derive(Debug)]
pub struct TabPlayStateChanged();

#[derive(Debug)]
pub struct PlayControl {
    pub second_to_units: f32,
    pub position: Position,
    pub begin_bar_ordinal: usize,
    pub end_bar_ordinal: usize,
    pub should_loop: bool,
    pub play_state: PlayState,
    pub play_speed: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct TickResult {
    pub changed: bool,
    pub end_passed: bool,
    pub stopped: bool,
}
impl TickResult {
    pub fn new(changed: bool, end_passed: bool, stopped: bool) -> Self {
        Self {
            changed,
            end_passed,
            stopped,
        }
    }
}
impl Default for PlayControl {
    fn default() -> Self {
        let tab_meta = TabMeta::default();
        Self::_new(&tab_meta, 0)
    }
}

impl PlayControl {
    pub fn _new(tab_meta: &TabMeta, bars: usize) -> Self {
        let second_to_units =
            Bpm::from(tab_meta.tempo) as f32 / 60.0 * Units::from(tab_meta.signature.beat_unit).0;
        Self {
            second_to_units,
            position: Position::new(tab_meta.bar_units()),
            begin_bar_ordinal: 1,
            end_bar_ordinal: bars,
            should_loop: true,
            play_state: PlayState::default(),
            play_speed: 1.0,
        }
    }
    pub fn new(tab: &Tab) -> Self {
        Self::_new(&tab.meta, tab.bars.len())
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
            self.position
                .set_in_bar(self.position.bar.bar_ordinal, Units(0.0));
            true
        }
    }
    pub fn stop(&mut self) -> bool {
        if self.play_state.is_stopped() {
            false
        } else {
            self.play_state = PlayState::Stopped;
            self.position.set_in_bar(self.begin_bar_ordinal, Units(0.0));
            true
        }
    }
    pub fn tick(&mut self, delta_seconds: f32) -> TickResult {
        if self.play_state.is_playing() {
            let delta_units = delta_seconds * self.second_to_units;
            self.position.tick(Units(delta_units * self.play_speed));
            let end_passed = self.position.bar.bar_ordinal > self.end_bar_ordinal;
            let stopped = if end_passed {
                if self.should_loop {
                    self.position
                        .set_in_bar(self.begin_bar_ordinal, self.position.bar.in_bar_pos);
                    if self.position.bar.bar_ordinal > self.end_bar_ordinal {
                        self.stop() //Corner case for too smal range
                    } else {
                        false
                    }
                } else {
                    self.stop()
                }
            } else {
                false
            };
            TickResult::new(true, end_passed, stopped)
        } else {
            TickResult::new(false, false, false)
        }
    }
    pub fn is_bar_in_range(&self, bar_ordinal: usize) -> bool {
        self.end_bar_ordinal >= self.begin_bar_ordinal
            && bar_ordinal >= self.begin_bar_ordinal
            && bar_ordinal <= self.end_bar_ordinal
    }
    pub fn begin_bar_position(&self) -> BarPosition {
        BarPosition::new(
            self.position.bar.bar_units,
            self.begin_bar_ordinal,
            Units(0.0),
        )
    }
}
