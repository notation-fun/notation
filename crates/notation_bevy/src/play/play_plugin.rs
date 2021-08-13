use std::sync::Arc;

use notation_midi::prelude::PlayControlEvt;
use notation_model::prelude::{LaneEntry, PlayState, Position, Tab, TickResult};

use bevy::prelude::*;
use notation_model::prelude::{BarPosition, Duration};

use crate::prelude::{
    BarLayout, EntryState, LyonShapeOp, NotationSettings, NotationTheme, TabBars, TabState,
    WindowResizedEvent,
};

use crate::tab::tab_state::TabPlayStateChanged;

use super::pos_indicator::{PosIndicator, PosIndicatorData};

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
        app.add_system(on_tab_play_state_changed.system());
        app.add_system(on_play_control_evt.system());
        //app.add_system(add_midi_tone.system());
    }
}

impl PlayPlugin {
    pub fn spawn_pos_indicator(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Tab,
        bar_layout: Option<&BarLayout>,
    ) {
        if let Some(bar_layout) = bar_layout {
            let data = PosIndicatorData::new(tab.bar_units(), bar_layout);
            PosIndicator::create(commands, entity, &theme, data);
        }
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    indicator_query: Query<(Entity, &PosIndicatorData)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in indicator_query.iter() {
            PosIndicator::update(&mut commands, &theme, entity, data);
        }
    }
}

fn on_tab_play_state_changed(
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut query: Query<(Entity, &Arc<Vec<BarLayout>>, &TabState), Added<TabPlayStateChanged>>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData)>,
    mut entry_query: Query<(Entity, &Arc<LaneEntry>, &BarPosition, &mut EntryState)>,
    mut tab_bars_query: Query<(Entity, &mut Transform, &TabBars)>,
) {
    for (state_entity, bar_layouts, state) in query.iter_mut() {
        TabState::clear_play_state_changed(&mut commands, state_entity);
        if !state.play_control.play_state.is_playing() {
            PosIndicator::update_pos(
                &mut commands,
                &theme,
                &settings,
                &mut pos_indicator_query,
                bar_layouts,
                state.play_control.position,
            );
            settings.layout.focus_bar(
                &mut commands,
                &mut tab_bars_query,
                bar_layouts,
                theme.grid.bar_size,
                &state,
            );
            for (_entity, _entry, position, mut entry_state) in entry_query.iter_mut() {
                if state.play_control.play_state.is_stopped() {
                    if state.is_in_range(position) {
                        *entry_state = EntryState::Idle;
                    }
                } else if state.play_control.play_state.is_paused() {
                    if position.bar_ordinal == state.play_control.position.bar.bar_ordinal {
                        *entry_state = EntryState::Idle;
                    }
                }
            }
        }
    }
}

fn on_tick(
    commands: &mut Commands,
    settings: &NotationSettings,
    theme: &NotationTheme,
    pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData)>,
    entry_query: &mut Query<(
        Entity,
        &Arc<LaneEntry>,
        &Duration,
        &BarPosition,
        &mut EntryState,
    )>,
    tab_bars_query: &mut Query<(Entity, &mut Transform, &TabBars)>,
    state_entity: Entity,
    bar_layouts: &Arc<Vec<BarLayout>>,
    state: &mut TabState,
    new_position: &Position,
    tick_result: &TickResult,
) {
    let old_position = state.play_control.position;
    state.set_position(*new_position);
    let TickResult {
        changed,
        end_passed,
        stopped,
    } = tick_result;
    if *stopped {
        state.set_play_state(commands, state_entity, PlayState::Stopped);
    } else if *changed {
        let pos = state.play_control.position;
        PosIndicator::update_pos(
            commands,
            theme,
            settings,
            pos_indicator_query,
            bar_layouts,
            pos,
        );
        if settings.layout.should_focus_bar(&old_position, &pos) {
            settings.layout.focus_bar(
                commands,
                tab_bars_query,
                bar_layouts,
                theme.grid.bar_size,
                state,
            );
        }
        for (_entity, _entry, duration, position, mut entry_state) in entry_query.iter_mut() {
            if state.is_in_range(position) {
                if entry_state.is_playing() && pos.is_passed_with(position, duration) {
                    *entry_state = EntryState::Played;
                }
                if entry_state.is_idle() && pos.is_passed(position) {
                    *entry_state = EntryState::Playing;
                }
                if *end_passed {
                    if entry_state.is_played() || position.bar_ordinal > pos.bar.bar_ordinal {
                        *entry_state = EntryState::Idle;
                    }
                }
            }
        }
    }
}

fn on_play_control_evt(
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut evts: EventReader<PlayControlEvt>,
    mut query: Query<(Entity, &Arc<Vec<BarLayout>>, &mut TabState)>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData)>,
    mut entry_query: Query<(
        Entity,
        &Arc<LaneEntry>,
        &Duration,
        &BarPosition,
        &mut EntryState,
    )>,
    mut tab_bars_query: Query<(Entity, &mut Transform, &TabBars)>,
) {
    for evt in evts.iter() {
        for (state_entity, bar_layouts, mut state) in query.iter_mut() {
            if !state.under_control {
                continue;
            }
            match evt {
                PlayControlEvt::OnTick {
                    position,
                    tick_result,
                } => on_tick(
                    &mut commands,
                    &settings,
                    &theme,
                    &mut pos_indicator_query,
                    &mut entry_query,
                    &mut tab_bars_query,
                    state_entity,
                    bar_layouts,
                    &mut state,
                    position,
                    tick_result,
                ),
                PlayControlEvt::OnPlayState(play_state) => {
                    state.set_play_state(&mut commands, state_entity, *play_state);
                }
                PlayControlEvt::OnPlaySpeed(play_speed) => state.set_play_speed(*play_speed),
            }
        }
    }
}
