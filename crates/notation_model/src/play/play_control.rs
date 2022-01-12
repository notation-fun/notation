use notation_proto::prelude::TabPosition;

use crate::prelude::{BarPosition, Bpm, PlayState, Position, Tab, TabMeta, Units};

#[derive(Debug)]
pub struct TabPlayStateChanged();

#[derive(Copy, Clone, Debug)]
pub struct PlaySpeed {
    pub seconds_per_unit: f32,
    pub units_per_second: f32,
    factor: f32,
}

impl PlaySpeed {
    pub fn new(tab_meta: &TabMeta) -> Self {
        let units_per_second =
            Bpm::from(tab_meta.tempo) as f32 / 60.0 * Units::from(tab_meta.signature.beat_unit).0;
        Self {
            seconds_per_unit: 1.0 / units_per_second,
            units_per_second,
            factor: 1.0,
        }
    }
    pub fn factor(&self) -> f32 {
        self.factor
    }
    pub fn set_factor(&mut self, factor: f32) -> bool {
        if factor > 0.0 {
            self.factor = factor;
            true
        } else {
            println!("Invalid Speed Factor: {}", factor);
            false
        }
    }
    pub fn calc_units(&self, seconds: f32) -> Units {
        Units(seconds * self.units_per_second * self.factor)
    }
    pub fn calc_seconds(&self, units: Units) -> f32 {
        units.0 * self.seconds_per_unit / self.factor
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct UnitsToSeconds(pub f32);

#[derive(Clone, Debug)]
pub struct PlayControl {
    pub position: Position,
    pub bars: usize,
    pub begin_bar_ordinal: usize,
    pub end_bar_ordinal: usize,
    pub should_loop: bool,
    pub play_state: PlayState,
    pub play_speed: PlaySpeed,
}

#[derive(Copy, Clone, Debug)]
pub struct TickResult {
    pub changed: bool,
    pub end_passed: bool,
    pub stopped: bool,
    pub jumped: bool,
}
impl TickResult {
    pub fn new(changed: bool, end_passed: bool, stopped: bool, jumped: bool) -> Self {
        Self {
            changed,
            end_passed,
            stopped,
            jumped,
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
    fn _new(tab_meta: &TabMeta, bars: usize) -> Self {
        Self {
            position: Position::new(tab_meta.bar_units()),
            bars,
            begin_bar_ordinal: 0,
            end_bar_ordinal: if bars > 0 { bars - 1 } else { bars },
            should_loop: false,
            play_state: PlayState::default(),
            play_speed: PlaySpeed::new(tab_meta),
        }
    }
    pub fn get_last_car_ordinal(&self) -> usize {
        if self.bars > 0 {
            self.bars - 1
        } else {
            0
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
            //self.position
            //    .set_in_bar(self.position.bar.bar_ordinal, Units(0.0));
            true
        }
    }
    pub fn stop(&mut self) -> bool {
        if !self.play_state.is_stopped() {
            self.play_state = PlayState::Stopped;
        }
        self.position.set_in_bar(self.begin_bar_ordinal, Units(0.0));
        true
    }
    pub fn _tick_to_position(&mut self, jumped: bool, pos: TabPosition) -> TickResult {
        self.position.set_in_tab(pos.in_tab_pos);
        let end_passed = self.position.bar.bar_ordinal > self.end_bar_ordinal;
        let stopped = if end_passed {
            if self.should_loop {
                self.position
                    .set_in_bar(self.begin_bar_ordinal, self.position.bar.in_bar_pos);
                if self.position.bar.bar_ordinal > self.end_bar_ordinal {
                    self.stop() //Corner case for too small range
                } else {
                    false
                }
            } else {
                self.stop()
            }
        } else {
            false
        };
        TickResult::new(true, end_passed, stopped, jumped)
    }
    pub fn tick(&mut self, jumped: bool, delta_seconds: f32) -> TickResult {
        if self.play_state.is_playing() {
            let mut jumped = jumped;
            let delta_units = if self.position.bar.bar_ordinal < self.begin_bar_ordinal
                || self.position.bar.bar_ordinal > self.end_bar_ordinal
            {
                self.position.set_in_bar(self.begin_bar_ordinal, Units(0.0));
                jumped = true;
                Units(0.0)
            } else {
                self.play_speed.calc_units(delta_seconds)
            };
            self._tick_to_position(
                jumped,
                TabPosition::new(self.position.tab.in_tab_pos + delta_units),
            )
        } else {
            TickResult::new(false, false, false, jumped)
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
