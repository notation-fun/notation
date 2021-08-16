use bevy::prelude::*;

use notation_model::prelude::{PlayControl, PlayState, Position, Tab};

#[derive(Debug)]
pub struct TabPlayStateChanged();

#[derive(Debug)]
pub struct TabState {
    pub under_control: bool,
    pub play_control: PlayControl,
}

impl TabState {
    pub fn new(tab: &Tab) -> Self {
        Self {
            under_control: true,
            play_control: PlayControl::new(tab),
        }
    }
    pub fn clear_play_state_changed(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<TabPlayStateChanged>();
    }
    fn add_play_state_changed(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(TabPlayStateChanged());
    }
    pub fn set_play_state(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        play_state: PlayState,
    ) -> bool {
        if self.play_control.play_state != play_state {
            self.play_control.play_state = play_state;
            Self::add_play_state_changed(commands, entity);
            true
        } else {
            false
        }
    }
    pub fn set_speed_factor(&mut self, speed_factor: f32) {
        self.play_control.play_speed.set_factor(speed_factor);
    }
    pub fn set_position(&mut self, position: Position) {
        self.play_control.position = position;
    }
    pub fn is_bar_in_range(&self, bar_ordinal: usize) -> bool {
        self.play_control.is_bar_in_range(bar_ordinal)
    }
}
