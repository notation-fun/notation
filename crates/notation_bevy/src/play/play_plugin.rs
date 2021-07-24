use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use instant::Instant as StdInstant;
#[cfg(target_arch = "wasm32")]
use instant::Duration as StdDuration;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant as StdInstant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration as StdDuration;

use bevy::prelude::*;
use notation_model::prelude::{BarPosition, Duration, ProtoEntry, Tab};

use crate::prelude::{BevyConfig, ConfigChangedEvent, EntryState, LyonShapeOp, TabState};

use super::pos_indicator::{PosIndicator, PosIndicatorData};

pub struct PlayPlugin;

pub struct NotationTime {
    last: StdInstant,
    pub delta: StdDuration,
}

impl Default for NotationTime {
    fn default() -> Self {
        NotationTime {
            last: StdInstant::now(),
            delta: StdDuration::new(0, 0),
        }
    }
}
impl NotationTime {
    pub fn tick(&mut self) {
        let now = StdInstant::now();
        self.delta = now.duration_since(self.last);
        self.last = now;
    }
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
}

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<NotationTime>();
        app.add_system(on_config_changed.system());
        app.add_system(on_add_tab_state.system());
        app.add_system(on_stop.system());
        app.add_system(on_time.system());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<ConfigChangedEvent>,
    config: Res<BevyConfig>,
    indicator_query: Query<(Entity, &PosIndicatorData)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in indicator_query.iter() {
            PosIndicator::update(&mut commands, &config, entity, data);
        }
    }
}

fn on_add_tab_state(
    mut commands: Commands,
    config: Res<BevyConfig>,
    state_query: Query<(Entity, &TabState), Added<TabState>>,
) {
    for (entity, _state) in state_query.iter() {
        PosIndicator::create(&mut commands, entity, &config, PosIndicatorData::default());
    }
}

fn on_stop(
    _commands: Commands,
    config: Res<BevyConfig>,
    mut query: Query<(&Arc<Tab>, &TabState, &mut Transform), Changed<TabState>>,
    mut entry_query: Query<(
        Entity,
        &Arc<ProtoEntry>,
        &BarPosition,
        &mut EntryState,
    )>,
) {
    for (tab, state, mut transform) in query.iter_mut() {
        if !state.play_state.is_playing() {
            *transform = config.grid.calc_pos_transform(tab, state.pos.tab);
            for (_entity, _entry, position, mut entry_state) in entry_query.iter_mut() {
                if state.play_state.is_stopped() {
                    if state.is_in_range(position) {
                        *entry_state = EntryState::Idle;
                    }
                } else if state.play_state.is_paused() {
                    if position.bar_ordinal == state.pos.bar.bar_ordinal {
                        *entry_state = EntryState::Idle;
                    }
                }
            }
        }
    }
}


fn on_time(
    _commands: Commands,
    config: Res<BevyConfig>,
    mut time: ResMut<NotationTime>,
    mut query: Query<(&Arc<Tab>, &mut TabState, &mut Transform)>,
    mut entry_query: Query<(
        Entity,
        &Arc<ProtoEntry>,
        &Duration,
        &BarPosition,
        &mut EntryState,
    )>,
) {
    time.tick();
    for (tab, mut state, mut transform) in query.iter_mut() {
        let (changed, end_passed) = state.tick(time.delta_seconds());
        if changed {
            *transform = config.grid.calc_pos_transform(tab, state.pos.tab);
            for (_entity, _entry, duration, position, mut entry_state) in entry_query.iter_mut() {
                if state.is_in_range(position) {
                    if entry_state.is_playing()
                        && state.pos.is_passed_with(position, duration) {
                        *entry_state = EntryState::Played;
                    }
                    if entry_state.is_idle() && state.pos.is_passed(position) {
                        *entry_state = EntryState::Playing;
                    }
                    if end_passed {
                        if entry_state.is_played() || position.bar_ordinal > state.pos.bar.bar_ordinal {
                            *entry_state = EntryState::Idle;
                        }
                    }
                }
            }
        }
    }
}
