use std::sync::Arc;

use notation_model::play::play_control::TickResult;
use notation_model::prelude::*;

#[derive(Debug)]
pub struct SwitchTabEvent {
    pub tab: Arc<Tab>,
}
impl SwitchTabEvent {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}

#[derive(Debug)]
pub enum PlayControlEvt {
    OnTick {
        position: Position,
        tick_result: TickResult,
    },
    OnPlayState(PlayState),
    OnSpeedFactor(f32),
}
impl PlayControlEvt {
    pub fn on_tick(position: Position, tick_result: TickResult) -> Self {
        Self::OnTick {
            position,
            tick_result,
        }
    }
    pub fn on_play_state(play_state: PlayState) -> Self {
        Self::OnPlayState(play_state)
    }
    pub fn on_speed_factor(play_speed: f32) -> Self {
        Self::OnSpeedFactor(play_speed)
    }
}
