use std::sync::Arc;

use bevy_utils::prelude::{DoLayoutEvent, GridData, LayoutData, ShapeOp};
use notation_midi::prelude::PlayControlEvent;
use notation_model::prelude::{
    LaneEntry, PlayState, PlayingState, Position, Tab, TabBarProps, TickResult,
};

use bevy::prelude::*;

use crate::bar::bar_beat::BarBeatData;
use crate::bar::bar_view::BarView;
use crate::chord::chord_playing::ChordPlaying;
use crate::prelude::{
    BarPlaying, EntryPlaying, NotationAssetsStates, NotationSettings, NotationTheme,
    TabBars, TabState,
};

use crate::settings::layout_settings::LayoutMode;
use crate::tab::tab_events::TabBarsResizedEvent;
use crate::tab::tab_state::TabPlayStateChanged;
use crate::ui::layout::NotationLayout;

use super::bar_indicator::{BarIndicatorData};
use super::play_button::PlayButton;
use super::play_panel::PlayPanel;
use super::pos_indicator::{PosIndicatorData};

pub type PlayPanelDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, PlayPanel>;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        PlayPanelDoLayoutEvent::setup(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(PlayPanel::do_layout.system())
                .with_system(PlayButton::on_layout_changed.system())
                .with_system(on_bar_playing_changed.system())
                .with_system(on_tab_play_state_changed.system())
                .with_system(on_play_control_evt.system())
                .with_system(on_tab_resized.system()),
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
        bar_data.create(commands, &theme, entity);
        let pos_data = PosIndicatorData::new(tab.bar_units());
        pos_data.create(commands, &theme, entity);
    }
}

fn update_indicators(
    commands: &mut Commands,
    theme: &NotationTheme,
    settings: &mut NotationSettings,
    bar_indicator_query: &mut Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
    pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
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
        data.update(commands, &theme, entity);
    }
    for (entity, mut data) in pos_indicator_query.iter_mut() {
        data.bar_props = bar_props;
        data.bar_layout = bar_layout;
        data.update(commands, &theme, entity);
        settings
            .layout
            .focus_bar(commands, theme, tab_bars_query, &data);
    }
}

fn on_tab_resized(
    mut evts: EventReader<TabBarsResizedEvent>,
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut settings: ResMut<NotationSettings>,
    mut query: Query<(Entity, &BarPlaying, &Arc<BarView>, &LayoutData)>,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &Arc<TabBars>,
        &LayoutData,
        &Arc<GridData>,
    )>,
) {
    let mut bars = None;
    for evt in evts.iter() {
        bars = Some(&evt.0);
    }
    if let Some(_bars) = bars {
        let mut first_playing_layout = None;
        let mut current_playing_layout = None;
        for (_entity, playing, view, layout) in query.iter_mut() {
            if view.bar_props.bar_ordinal == 1 {
                first_playing_layout = Some((playing, layout.clone()));
            }
            if playing.value == PlayingState::Current {
                current_playing_layout = Some((playing, layout.clone()));
                break;
            }
        }
        let playing_layout = if current_playing_layout.is_none() {
            first_playing_layout
        } else {
            current_playing_layout
        };
        if let Some((playing, layout)) = playing_layout {
            update_indicators(
                &mut commands,
                &theme,
                &mut settings,
                &mut bar_indicator_query,
                &mut pos_indicator_query,
                &mut tab_bars_query,
                playing.bar_props,
                layout.clone(),
            );
        }
    }
}

fn on_bar_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut settings: ResMut<NotationSettings>,
    mut query: Query<(Entity, &BarPlaying, &Arc<BarView>, &LayoutData), Changed<BarPlaying>>,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
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
                &mut settings,
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
    mut settings: ResMut<NotationSettings>,
    mut query: Query<(Entity, &TabState), Added<TabPlayStateChanged>>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    mut bar_playing_query: Query<(Entity, &mut BarPlaying), With<BarPlaying>>,
    mut entry_playing_query: Query<
        (Entity, &Arc<LaneEntry>, &mut EntryPlaying),
        With<EntryPlaying>,
    >,
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
        if let Some(pos_data) = PosIndicatorData::update_pos(
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
    settings: &mut NotationSettings,
    pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    bar_playing_query: &mut Query<(Entity, &mut BarPlaying), With<BarPlaying>>,
    entry_playing_query: &mut Query<
        (Entity, &Arc<LaneEntry>, &mut EntryPlaying),
        With<EntryPlaying>,
    >,
    chord_playing_query: &mut Query<(Entity, &mut ChordPlaying), With<ChordPlaying>>,
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
        changed: _changed,
        end_passed,
        stopped,
        jumped,
    } = tick_result;
    if *stopped {
        tab_state.set_play_state(commands, state_entity, PlayState::Stopped);
    } else {
        if let Some(pos_data) =
            PosIndicatorData::update_pos(commands, theme, pos_indicator_query, *new_position)
        {
            if settings.layout.mode == LayoutMode::Line && pos_data.is_synced() {
                settings
                    .layout
                    .focus_bar(commands, theme, tab_bars_query, &pos_data);
            }
        }
        let playing_bar_ordinal = new_position.bar.bar_ordinal;
        BarPlaying::update(bar_playing_query, tab_state, playing_bar_ordinal);
        ChordPlaying::update(chord_playing_query, tab_state, new_position);
        EntryPlaying::update_with_pos(
            entry_playing_query,
            tab_state,
            new_position,
            *end_passed,
            *jumped,
        );
    }
}

fn on_play_control_evt(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut settings: ResMut<NotationSettings>,
    mut evts: EventReader<PlayControlEvent>,
    mut tab_state_query: Query<(Entity, &mut TabState)>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    mut bar_playing_query: Query<(Entity, &mut BarPlaying), With<BarPlaying>>,
    mut entry_playing_query: Query<
        (Entity, &Arc<LaneEntry>, &mut EntryPlaying),
        With<EntryPlaying>,
    >,
    mut chord_playing_query: Query<(Entity, &mut ChordPlaying), With<ChordPlaying>>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &Arc<TabBars>,
        &LayoutData,
        &Arc<GridData>,
    )>,
    mut beat_query: Query<(Entity, &mut BarBeatData)>,
) {
    for evt in evts.iter() {
        for (state_entity, mut tab_state) in tab_state_query.iter_mut() {
            if !tab_state.under_control {
                continue;
            }
            match evt {
                PlayControlEvent::OnTick {
                    position,
                    tick_result,
                } => on_tick(
                    &mut commands,
                    &theme,
                    &mut settings,
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
                PlayControlEvent::OnPlayState(play_state) => {
                    tab_state.set_play_state(&mut commands, state_entity, *play_state);
                }
                PlayControlEvent::OnSpeedFactor(play_speed) => {
                    tab_state.set_speed_factor(*play_speed);
                }
                PlayControlEvent::OnBeginEnd(begin_bar_ordinal, end_bar_ordinal) => {
                    tab_state.set_begin_end(*begin_bar_ordinal, *end_bar_ordinal);
                    BarBeatData::update_all(&mut commands, &theme, &tab_state, &mut beat_query);
                }
            }
        }
    }
}
