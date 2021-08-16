use std::sync::Arc;

use notation_midi::prelude::PlayControlEvt;
use notation_model::prelude::{LaneEntry, PlayState, PlayingState, Position, Tab, TickResult};

use bevy::prelude::*;
use notation_model::prelude::Entry;

use crate::prelude::{
    BarLayout, BarPlaying, EntryPlaying, LyonShapeOp, NotationSettings, NotationTheme, TabBars,
    TabState, WindowResizedEvent,
};

use crate::tab::tab_state::TabPlayStateChanged;

use super::bar_indicator::{BarIndicator, BarIndicatorData};
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
    pub fn spawn_indicators(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Tab,
        bar_layout: Option<&BarLayout>,
    ) {
        if let Some(bar_layout) = bar_layout {
            let bar_data = BarIndicatorData::new(tab.bar_units(), bar_layout);
            BarIndicator::create(commands, entity, &theme, bar_data);
            let pos_data = PosIndicatorData::new(tab.bar_units(), bar_layout);
            PosIndicator::create(commands, entity, &theme, pos_data);
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

fn update_bar_playings(
    tab_state: &TabState,
    playing_bar_ordinal: usize,
    bar_playing_query: &mut Query<(Entity, &mut BarPlaying)>,
) {
    for (_entity, mut bar_playing) in bar_playing_query.iter_mut() {
        let bar_ordinal = bar_playing.bar_props.bar_ordinal;
        if tab_state.is_bar_in_range(bar_ordinal) {
            if bar_ordinal == playing_bar_ordinal {
                if bar_playing.value != PlayingState::Current {
                    bar_playing.value = PlayingState::Current;
                }
            } else if bar_ordinal < playing_bar_ordinal {
                if bar_playing.value != PlayingState::Played {
                    bar_playing.value = PlayingState::Played;
                }
            } else {
                if bar_playing.value != PlayingState::Idle {
                    bar_playing.value = PlayingState::Idle;
                }
            }
        }
    }
}

fn on_tab_play_state_changed(
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut query: Query<(Entity, &Arc<Vec<BarLayout>>, &TabState), Added<TabPlayStateChanged>>,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData)>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData)>,
    mut bar_playing_query: Query<(Entity, &mut BarPlaying)>,
    mut entry_playing_query: Query<(Entity, &Arc<LaneEntry>, &mut EntryPlaying)>,
    mut tab_bars_query: Query<(Entity, &mut Transform, &TabBars)>,
) {
    for (state_entity, bar_layouts, tab_state) in query.iter_mut() {
        TabState::clear_play_state_changed(&mut commands, state_entity);
        if !tab_state.play_control.play_state.is_playing() {
            BarIndicator::update_pos(
                &mut commands,
                &theme,
                &settings,
                &mut bar_indicator_query,
                bar_layouts,
                tab_state.play_control.position,
            );
            PosIndicator::update_pos(
                &mut commands,
                &theme,
                &settings,
                &mut pos_indicator_query,
                bar_layouts,
                tab_state.play_control.position,
            );
            settings.layout.focus_bar(
                &mut commands,
                &mut tab_bars_query,
                bar_layouts,
                theme.grid.bar_size,
                &tab_state,
            );
            let playing_bar_ordinal = tab_state.play_control.position.bar.bar_ordinal;
            update_bar_playings(tab_state, playing_bar_ordinal, &mut bar_playing_query);
            for (_entity, _entry, mut entry_state) in entry_playing_query.iter_mut() {
                if tab_state.play_control.play_state.is_stopped() {
                    if tab_state.is_bar_in_range(entry_state.bar_props.bar_ordinal) {
                        entry_state.value = PlayingState::Idle;
                    }
                } else if tab_state.play_control.play_state.is_paused() {
                    if entry_state.bar_props.bar_ordinal
                        == tab_state.play_control.position.bar.bar_ordinal
                    {
                        entry_state.value = PlayingState::Idle;
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
    bar_indicator_query: &mut Query<(Entity, &mut BarIndicatorData)>,
    pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData)>,
    bar_playing_query: &mut Query<(Entity, &mut BarPlaying)>,
    entry_playing_query: &mut Query<(Entity, &Arc<LaneEntry>, &mut EntryPlaying)>,
    tab_bars_query: &mut Query<(Entity, &mut Transform, &TabBars)>,
    state_entity: Entity,
    bar_layouts: &Arc<Vec<BarLayout>>,
    tab_state: &mut TabState,
    new_position: &Position,
    tick_result: &TickResult,
) {
    let old_position = tab_state.play_control.position;
    tab_state.set_position(*new_position);
    let TickResult {
        changed,
        end_passed,
        stopped,
    } = tick_result;
    if *stopped {
        tab_state.set_play_state(commands, state_entity, PlayState::Stopped);
    } else if *changed {
        BarIndicator::update_pos(
            commands,
            theme,
            settings,
            bar_indicator_query,
            bar_layouts,
            *new_position,
        );
        PosIndicator::update_pos(
            commands,
            theme,
            settings,
            pos_indicator_query,
            bar_layouts,
            *new_position,
        );
        if settings
            .layout
            .should_focus_bar(&old_position, new_position)
        {
            settings.layout.focus_bar(
                commands,
                tab_bars_query,
                bar_layouts,
                theme.grid.bar_size,
                tab_state,
            );
        }
        let playing_bar_ordinal = new_position.bar.bar_ordinal;
        update_bar_playings(tab_state, playing_bar_ordinal, bar_playing_query);
        for (_entity, mut bar_playing) in bar_playing_query.iter_mut() {
            let bar_ordinal = bar_playing.bar_props.bar_ordinal;
            if tab_state.is_bar_in_range(bar_ordinal) {
                if bar_ordinal == playing_bar_ordinal {
                    if bar_playing.value != PlayingState::Current {
                        bar_playing.value = PlayingState::Current;
                    }
                } else if bar_ordinal < playing_bar_ordinal {
                    if bar_playing.value != PlayingState::Played {
                        bar_playing.value = PlayingState::Played;
                    }
                } else {
                    if bar_playing.value != PlayingState::Idle {
                        bar_playing.value = PlayingState::Idle;
                    }
                }
            }
        }
        for (_entity, entry, mut entry_playing) in entry_playing_query.iter_mut() {
            let bar_ordinal = entry_playing.bar_props.bar_ordinal;
            if tab_state.is_bar_in_range(bar_ordinal) {
                if entry_playing.value.is_current()
                    && new_position
                        .is_passed_with(&entry_playing.bar_position(), entry.tied_units())
                {
                    if entry_playing.value != PlayingState::Played {
                        entry_playing.value = PlayingState::Played;
                    }
                }
                if entry_playing.value.is_idle()
                    && new_position.is_passed(&entry_playing.bar_position())
                {
                    if entry_playing.value != PlayingState::Current {
                        entry_playing.value = PlayingState::Current;
                    }
                }
                if *end_passed {
                    if entry_playing.value.is_played() || bar_ordinal > playing_bar_ordinal {
                        if entry_playing.value != PlayingState::Idle {
                            entry_playing.value = PlayingState::Idle;
                        }
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
    mut tab_state_query: Query<(Entity, &Arc<Vec<BarLayout>>, &mut TabState)>,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData)>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData)>,
    mut bar_playing_query: Query<(Entity, &mut BarPlaying)>,
    mut entry_playing_query: Query<(Entity, &Arc<LaneEntry>, &mut EntryPlaying)>,
    mut tab_bars_query: Query<(Entity, &mut Transform, &TabBars)>,
) {
    for evt in evts.iter() {
        for (state_entity, bar_layouts, mut tab_state) in tab_state_query.iter_mut() {
            if !tab_state.under_control {
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
                    &mut bar_indicator_query,
                    &mut pos_indicator_query,
                    &mut bar_playing_query,
                    &mut entry_playing_query,
                    &mut tab_bars_query,
                    state_entity,
                    bar_layouts,
                    &mut tab_state,
                    position,
                    tick_result,
                ),
                PlayControlEvt::OnPlayState(play_state) => {
                    tab_state.set_play_state(&mut commands, state_entity, *play_state);
                }
                PlayControlEvt::OnSpeedFactor(play_speed) => tab_state.set_speed_factor(*play_speed),
            }
        }
    }
}
