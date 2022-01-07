use std::sync::Arc;

use crate::prelude::*;

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
pub struct JumpToBarEvent {
    pub bar_props: TabBarProps,
}
impl JumpToBarEvent {
    pub fn new(bar_props: TabBarProps) -> Self {
        Self { bar_props }
    }
}

#[derive(Debug)]
pub enum PlayControlEvent {
    OnTick {
        position: Position,
        tick_result: TickResult,
    },
    OnPlayState(PlayState),
    OnSpeedFactor(f32),
    OnShouldLoop(bool),
    OnBeginEnd(usize, usize),
}
impl PlayControlEvent {
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
    pub fn on_should_loop(should_loop: bool) -> Self {
        Self::OnShouldLoop(should_loop)
    }
    pub fn on_begin_end(begin_bar_ordinal: usize, end_bar_ordinal: usize) -> Self {
        Self::OnBeginEnd(begin_bar_ordinal, end_bar_ordinal)
    }
}
