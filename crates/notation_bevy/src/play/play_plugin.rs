use std::sync::Arc;

use bevy_utils::prelude::{GridData, LayoutData};
use notation_midi::prelude::PlayControlEvt;
use notation_model::prelude::{
    LaneEntry, PlayState, PlayingState, Position, Tab, TabBarProps, TickResult,
};

use bevy::prelude::*;

use crate::bar::bar_view::BarView;
use crate::chord::chord_playing::ChordPlaying;
use crate::prelude::{BarPlaying, EntryPlaying, LyonShapeOp, NotationAssetsStates, NotationSettings, NotationTheme, TabBars, TabState};

use crate::settings::layout_settings::LayoutMode;
use crate::tab::tab_events::TabResizedEvent;
use crate::tab::tab_state::TabPlayStateChanged;

use super::bar_indicator::{BarIndicator, BarIndicatorData};
use super::pos_indicator::{PosIndicator, PosIndicatorData};

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(NotationAssetsStates::Loaded)
            .with_system(on_bar_playing_changed.system())
            .with_system(on_tab_play_state_changed.system())
            .with_system(on_play_control_evt.system())
            .with_system(on_tab_resized.system())
        );
    }
}

impl PlayPlugin {
    pub fn spawn_indicators(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Tab,
    ) {
        let bar_data = BarIndicatorData::new();
        BarIndicator::create(commands, &theme, entity, bar_data);
        let pos_data = PosIndicatorData::new(tab.bar_units());
        PosIndicator::create(commands, &theme, entity, pos_data);
    }
}

fn update_indicators(
    commands: &mut Commands,
    theme: &NotationTheme,
    settings: &NotationSettings,
    bar_indicator_query: &mut Query<(Entity, &mut BarIndicatorData)>,
    pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData)>,
    tab_bars_query: &mut Query<(
        Entity,
        &mut Transform,
        &Arc<TabBars>,
        &LayoutData,
        &Arc<GridData>,
    )>,
    bar_props: TabBarProps,
    bar_layout: LayoutData,
) {
    for (entity, mut data) in bar_indicator_query.iter_mut() {
        data.bar_props = bar_props;
        data.bar_layout = bar_layout;
        BarIndicator::update(commands, &theme, entity, &data);
    }
    for (entity, mut data) in pos_indicator_query.iter_mut() {
        data.bar_props = bar_props;
        data.bar_layout = bar_layout;
        PosIndicator::update(commands, &theme, entity, &data);
        settings
            .layout
            .focus_bar(commands, theme, tab_bars_query, &data);
    }
}

fn on_tab_resized(
    mut evts: EventReader<TabResizedEvent>,
    mut commands: Commands,
    theme: Res<NotationTheme>,
    settings: Res<NotationSettings>,
    mut query: Query<(Entity, &BarPlaying, &Arc<BarView>, &LayoutData)>,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData)>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData)>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &Arc<TabBars>,
        &LayoutData,
        &Arc<GridData>,
    )>,
) {
    for _evt in evts.iter() {
        for (_entity, playing, _view, layout) in query.iter_mut() {
            if playing.value == PlayingState::Current {
                update_indicators(
                    &mut commands,
                    &theme,
                    &settings,
                    &mut bar_indicator_query,
                    &mut pos_indicator_query,
                    &mut tab_bars_query,
                    playing.bar_props,
                    layout.clone(),
                );
                break;
            }
        }
    }
}

fn on_bar_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    settings: Res<NotationSettings>,
    mut query: Query<(Entity, &BarPlaying, &Arc<BarView>, &LayoutData), Changed<BarPlaying>>,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData)>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData)>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &Arc<TabBars>,
        &LayoutData,
        &Arc<GridData>,
    )>,
) {
    for (_entity, playing, _view, layout) in query.iter_mut() {
        if playing.value == PlayingState::Current {
            update_indicators(
                &mut commands,
                &theme,
                &settings,
                &mut bar_indicator_query,
                &mut pos_indicator_query,
                &mut tab_bars_query,
                playing.bar_props,
                layout.clone(),
            );
            break;
        }
    }
}

fn on_tab_play_state_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    settings: Res<NotationSettings>,
    mut query: Query<(Entity, &TabState), Added<TabPlayStateChanged>>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData)>,
    mut bar_playing_query: Query<(Entity, &mut BarPlaying)>,
    mut entry_playing_query: Query<(Entity, &Arc<LaneEntry>, &mut EntryPlaying)>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &Arc<TabBars>,
        &LayoutData,
        &Arc<GridData>,
    )>,
) {
    for (state_entity, tab_state) in query.iter_mut() {
        TabState::clear_play_state_changed(&mut commands, state_entity);
        if let Some(pos_data) = PosIndicator::update_pos(
            &mut commands,
            &theme,
            &mut pos_indicator_query,
            tab_state.play_control.position,
        ) {
            settings
                .layout
                .focus_bar(&mut commands, &theme, &mut tab_bars_query, &pos_data);
        }
        if !tab_state.play_control.play_state.is_playing() {
            let playing_bar_ordinal = tab_state.play_control.position.bar.bar_ordinal;
            BarPlaying::update(&mut bar_playing_query, tab_state, playing_bar_ordinal);
            EntryPlaying::update(&mut entry_playing_query, tab_state);
        }
    }
}

fn on_tick(
    commands: &mut Commands,
    theme: &NotationTheme,
    settings: &NotationSettings,
    pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData)>,
    bar_playing_query: &mut Query<(Entity, &mut BarPlaying)>,
    entry_playing_query: &mut Query<(Entity, &Arc<LaneEntry>, &mut EntryPlaying)>,
    chord_playing_query: &mut Query<(Entity, &mut ChordPlaying)>,
    tab_bars_query: &mut Query<(
        Entity,
        &mut Transform,
        &Arc<TabBars>,
        &LayoutData,
        &Arc<GridData>,
    )>,
    state_entity: Entity,
    tab_state: &mut TabState,
    new_position: &Position,
    tick_result: &TickResult,
) {
    tab_state.set_position(*new_position);
    let TickResult {
        changed,
        end_passed,
        stopped,
    } = tick_result;
    if *stopped {
        tab_state.set_play_state(commands, state_entity, PlayState::Stopped);
    } else if *changed {
        if let Some(pos_data) =
            PosIndicator::update_pos(commands, theme, pos_indicator_query, *new_position)
        {
            if settings.layout.mode == LayoutMode::Line && pos_data.is_synced() {
                settings
                    .layout
                    .focus_bar(commands, theme, tab_bars_query, &pos_data);
            }
        }
        let playing_bar_ordinal = new_position.bar.bar_ordinal;
        BarPlaying::update(bar_playing_query, tab_state, playing_bar_ordinal);
        EntryPlaying::update_with_pos(entry_playing_query, tab_state, new_position, *end_passed);
        ChordPlaying::update(chord_playing_query, tab_state, new_position);
    }
}

fn on_play_control_evt(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    settings: Res<NotationSettings>,
    mut evts: EventReader<PlayControlEvt>,
    mut tab_state_query: Query<(Entity, &mut TabState)>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData)>,
    mut bar_playing_query: Query<(Entity, &mut BarPlaying)>,
    mut entry_playing_query: Query<(Entity, &Arc<LaneEntry>, &mut EntryPlaying)>,
    mut chord_playing_query: Query<(Entity, &mut ChordPlaying)>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &Arc<TabBars>,
        &LayoutData,
        &Arc<GridData>,
    )>,
) {
    for evt in evts.iter() {
        for (state_entity, mut tab_state) in tab_state_query.iter_mut() {
            if !tab_state.under_control {
                continue;
            }
            match evt {
                PlayControlEvt::OnTick {
                    position,
                    tick_result,
                } => on_tick(
                    &mut commands,
                    &theme,
                    &settings,
                    &mut pos_indicator_query,
                    &mut bar_playing_query,
                    &mut entry_playing_query,
                    &mut chord_playing_query,
                    &mut tab_bars_query,
                    state_entity,
                    &mut tab_state,
                    position,
                    tick_result,
                ),
                PlayControlEvt::OnPlayState(play_state) => {
                    tab_state.set_play_state(&mut commands, state_entity, *play_state);
                }
                PlayControlEvt::OnSpeedFactor(play_speed) => {
                    tab_state.set_speed_factor(*play_speed)
                }
            }
        }
    }
}
