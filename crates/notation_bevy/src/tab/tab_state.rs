use crate::prelude::PlayState;
use notation_model::prelude::{BarPosition, Bpm, Position, Tab, Units};
use bevy::prelude::*;

#[derive(Debug)]
pub struct TabPlayStateChanged();

#[derive(Debug)]
pub struct TabState {
    pub second_to_units: f32,
    pub pos: Position,
    pub begin_bar_ordinal: usize,
    pub end_bar_ordinal: usize,
    pub should_loop: bool,
    pub play_state: PlayState,
    pub play_speed: f32,
}

impl TabState {
    pub fn new(tab: &Tab) -> Self {
        let second_to_units =
            Bpm::from(tab.meta.tempo) as f32 / 60.0 * Units::from(tab.meta.signature.beat_unit).0;
        Self {
            second_to_units,
            pos: Position::new(tab.bar_units()),
            begin_bar_ordinal: 1,
            end_bar_ordinal: tab.bars.len(),
            should_loop: true,
            play_state: PlayState::default(),
            play_speed: 1.0,
        }
    }
    pub fn clear_play_state_changed(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<TabPlayStateChanged>();
    }
    fn add_play_state_changed(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(TabPlayStateChanged());
    }
    pub fn play(&mut self, commands: &mut Commands, entity: Entity) -> bool {
        if self.play_state.is_playing() {
            false
        } else {
            self.play_state = PlayState::Playing;
            Self::add_play_state_changed(commands, entity);
            true
        }
    }
    pub fn pause(&mut self, commands: &mut Commands, entity: Entity) -> bool {
        if self.play_state.is_paused() {
            false
        } else {
            self.play_state = PlayState::Paused;
            self.pos.set_in_bar(self.pos.bar.bar_ordinal, Units(0.0));
            Self::add_play_state_changed(commands, entity);
            true
        }
    }
    pub fn stop(&mut self, commands: &mut Commands, entity: Entity) -> bool {
        if self.play_state.is_stopped() {
            false
        } else {
            self.play_state = PlayState::Stopped;
            self.pos.set_in_bar(self.begin_bar_ordinal, Units(0.0));
            Self::add_play_state_changed(commands, entity);
            true
        }
    }
    pub fn tick(&mut self, commands: &mut Commands, entity: Entity, delta_seconds: f32) -> (bool, bool) {
        if self.play_state.is_playing() {
            let delta_units = delta_seconds * self.second_to_units;
            self.pos.tick(Units(delta_units * self.play_speed));
            let end_passed = self.pos.bar.bar_ordinal > self.end_bar_ordinal;
            if end_passed {
                if self.should_loop {
                    self.pos
                        .set_in_bar(self.begin_bar_ordinal, self.pos.bar.in_bar_pos);
                    if self.pos.bar.bar_ordinal > self.end_bar_ordinal {
                        self.stop(commands, entity); //Corner case for too smal range
                    }
                } else {
                    self.stop(commands, entity);
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
