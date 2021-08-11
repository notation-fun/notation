use std::sync::Arc;

use bevy::render::camera::OrthographicProjection;
use notation_midi::prelude::{AddToneEvent, PlayControlEvt};
use notation_model::prelude::{PlayState, Position, Tone, Units, Entry, TickResult};

use bevy::prelude::*;
use notation_model::prelude::{BarPosition, Duration, ModelEntry};

use crate::prelude::{
    BarLayout, EntryState, LyonShapeOp, NotationSettings, NotationTheme, TabState,
    WindowResizedEvent,
};

use crate::tab::tab_state::TabPlayStateChanged;

use super::pos_indicator::{PosIndicator, PosIndicatorData};

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
        app.add_system(on_add_tab_state.system());
        app.add_system(on_tab_play_state_changed.system());
        app.add_system(on_play_control_evt.system());
        app.add_system(add_midi_tone.system());
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

fn on_add_tab_state(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    state_query: Query<(Entity, &Arc<Vec<BarLayout>>, &TabState), Added<TabState>>,
) {
    for (entity, bar_layouts, state) in state_query.iter() {
        if let Some(bar_layout) = bar_layouts.get(0) {
            let pos = state.play_control.position;
            let data = PosIndicatorData::new(pos.bar.bar_units, bar_layout);
            PosIndicator::create(&mut commands, entity, &theme, data);
        }
    }
}

fn on_tab_play_state_changed(
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut query: Query<
        (Entity, &Arc<Vec<BarLayout>>, &TabState, &Children),
        Added<TabPlayStateChanged>,
    >,
    mut pos_indicator_query: Query<&mut PosIndicatorData>,
    mut entry_query: Query<(Entity, &Arc<ModelEntry>, &BarPosition, &mut EntryState)>,
    mut camera_query: Query<(Entity, &mut Transform, &OrthographicProjection)>,
) {
    for (state_entity, bar_layouts, state, children) in query.iter_mut() {
        TabState::clear_play_state_changed(&mut commands, state_entity);
        if !state.play_control.play_state.is_playing() {
            PosIndicator::update_pos(
                &mut commands,
                &theme,
                children,
                &settings,
                &mut pos_indicator_query,
                bar_layouts,
                state.play_control.position,
            );
            settings.layout.focus_camera(
                &mut commands,
                &mut camera_query,
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
    pos_indicator_query: &mut Query<&mut PosIndicatorData>,
    entry_query: &mut Query<(
        Entity,
        &Arc<ModelEntry>,
        &Duration,
        &BarPosition,
        &mut EntryState,
    )>,
    camera_query: &mut Query<(Entity, &mut Transform, &OrthographicProjection)>,
    state_entity: Entity,
    bar_layouts: &Arc<Vec<BarLayout>>,
    state: &mut TabState,
    children: &Children,
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
            children,
            settings,
            pos_indicator_query,
            bar_layouts,
            pos,
        );
        //settings.layout.focus_camera(&mut camera_query, bar_layouts, state.pos, theme.grid.bar_size);
        if settings.layout.should_focus_camera(&old_position, &pos) {
            settings.layout.focus_camera(
                commands,
                camera_query,
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
    mut query: Query<(Entity, &Arc<Vec<BarLayout>>, &mut TabState, &Children)>,
    mut pos_indicator_query: Query<&mut PosIndicatorData>,
    mut entry_query: Query<(
        Entity,
        &Arc<ModelEntry>,
        &Duration,
        &BarPosition,
        &mut EntryState,
    )>,
    mut camera_query: Query<(Entity, &mut Transform, &OrthographicProjection)>,
) {
    for evt in evts.iter() {
        for (state_entity, bar_layouts, mut state, children) in query.iter_mut() {
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
                    &mut camera_query,
                    state_entity,
                    bar_layouts,
                    &mut state,
                    children,
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

/*
fn play_stop_tone(
    mut _commands: Commands,
    _theme: Res<NotationTheme>,
    query: Query<(&Arc<ModelEntry>, &Tone, &EntryState), Changed<EntryState>>,
    mut play_note_evts: EventWriter<PlayToneEvent>,
    mut stop_note_evts: EventWriter<StopToneEvent>,
) {
    for (entry, tone, state) in query.iter() {
        if !tone.is_none() {
            if state.is_played() || state.is_idle() {
                stop_note_evts.send(StopToneEvent::new(
                    entry.track_id(),
                    entry.track_kind(),
                    *tone,
                ));
            } else if state.is_playing() {
                play_note_evts.send(PlayToneEvent::new(
                    entry.track_id(),
                    entry.track_kind(),
                    *tone,
                ));
            }
        }
    }
}
*/

fn add_midi_tone(
    query: Query<(&Arc<ModelEntry>, &Tone, &BarPosition, &Units), Added<Tone>>,
    mut add_note_evts: EventWriter<AddToneEvent>,
) {
    for (entry, tone, position, tied_units) in query.iter() {
        if !tone.is_none() && !entry.as_ref().prev_is_tie() {
            add_note_evts.send(AddToneEvent::new(
                entry.track_id(),
                entry.track_kind(),
                *tone,
                *position,
                *tied_units,
            ));
        }
    }
}
