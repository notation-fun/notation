use std::sync::Arc;

use edger_bevy::prelude::{ColorBackground, DoLayoutEvent, GridData, LayoutData, ShapeOp, SingleData};
use notation_model::prelude::{LaneEntry, Position, Tab};
use notation_midi::prelude::{PlayControlEvent, PlayState, PlayingState, SwitchTabEvent, JumpToBarEvent, TickResult};

use edger_bevy::bevy_prelude::*;
use edger_bevy::prelude::AssetsStates;

use crate::bar::bar_beat::BarBeatData;
use crate::bar::bar_view::BarView;
use crate::chord::chord_color_background::ChordColorBackground;
use crate::chord::chord_playing::ChordPlaying;
use crate::prelude::{
    BarPlaying, EntryPlaying, NotationSettings, NotationTheme, TabBars,
    TabState,
};

use crate::settings::layout_settings::LayoutMode;
use crate::tab::tab_events::TabBarsResizedEvent;
use crate::tab::tab_state::TabPlayStateChanged;
use crate::prelude::NotationLayout;

use super::bar_indicator::BarIndicatorData;
use super::play_button::PlayButton;
use super::play_panel::PlayPanel;
use super::pos_indicator::PosIndicatorData;

pub type PlayPanelDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, PlayPanel>;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwitchTabEvent>();
        app.add_event::<JumpToBarEvent>();
        app.add_event::<PlayControlEvent>();
        PlayPanelDoLayoutEvent::setup(app);
        app.add_systems(Update, (
            PlayPanel::do_layout,
            PlayPanel::on_play_control_evt,
            PlayButton::on_layout_changed,
            on_bar_playing_changed,
            on_tab_play_state_changed,
            on_play_control_evt,
            on_tab_resized,
        ).run_if(in_state(AssetsStates::Loaded)));
    }
}

impl PlayPlugin {
    pub fn spawn_indicators(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Arc<Tab>,
    ) {
        let bar_data = BarIndicatorData::new(tab.clone());
        bar_data.create(commands, &theme, entity);
        let pos_data = PosIndicatorData::new(tab.bar_units());
        pos_data.create(commands, &theme, entity);
    }
}

fn update_indicators(
    commands: &mut Commands,
    theme: &NotationTheme,
    settings: &mut NotationSettings,
    chord_color_background_query: &mut Query<
        (Entity, &mut ColorBackground),
        With<ChordColorBackground>,
    >,
    bar_indicator_query: &mut Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
    pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    tab_bars_query: &mut Query<(
        Entity,
        &mut Transform,
        &TabBars,
        &LayoutData,
        &GridData,
    )>,
    bar_playing: &BarPlaying,
    bar_layout: LayoutData,
) {
    let bar_props = bar_playing.bar_props;
    let mut in_bar_pos = None;
    for (entity, mut data) in pos_indicator_query.iter_mut() {
        data.hidden = settings.hide_indicators;
        data.bar_props = bar_props;
        data.bar_layout = bar_layout;
        data.update(commands, &theme, entity);
        settings
            .layout
            .focus_bar(commands, theme, tab_bars_query, &data);
        in_bar_pos = Some(data.bar_position.in_bar_pos);
    }
    if settings.hide_indicators {
        return;
    }
    for (entity, mut data) in bar_indicator_query.iter_mut() {
        data.bar_props = bar_props;
        data.bar_layout = bar_layout;
        data.update_data(commands, theme, entity, bar_props, bar_layout, in_bar_pos);
        ChordColorBackground::update_color(
            commands,
            theme,
            chord_color_background_query,
            data.chord,
        );
    }
}

fn on_tab_resized(
    mut evts: EventReader<TabBarsResizedEvent>,
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut settings: ResMut<NotationSettings>,
    mut query: Query<(Entity, &BarPlaying, &BarView, &LayoutData)>,
    mut chord_color_background_query: Query<
        (Entity, &mut ColorBackground),
        With<ChordColorBackground>,
    >,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &TabBars,
        &LayoutData,
        &GridData,
    )>,
) {
    if theme._bypass_systems {
        return;
    }
    let mut bars = None;
    for evt in evts.read() {
        bars = Some(&evt.0);
    }
    if let Some(_bars) = bars {
        let mut first_playing_layout = None;
        let mut current_playing_layout = None;
        for (_entity, playing, view, layout) in query.iter_mut() {
            if view.bar_props.bar_ordinal == 0 {
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
                &mut chord_color_background_query,
                &mut bar_indicator_query,
                &mut pos_indicator_query,
                &mut tab_bars_query,
                playing,
                layout.clone(),
            );
        }
    }
}

fn on_bar_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut settings: ResMut<NotationSettings>,
    mut query: Query<(Entity, &BarPlaying, &BarView, &LayoutData), Changed<BarPlaying>>,
    mut chord_color_background_query: Query<
        (Entity, &mut ColorBackground),
        With<ChordColorBackground>,
    >,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &TabBars,
        &LayoutData,
        &GridData,
    )>,
) {
    if theme._bypass_systems {
        return;
    }
    for (_entity, playing, _view, layout) in query.iter_mut() {
        if playing.value == PlayingState::Current {
            update_indicators(
                &mut commands,
                &theme,
                &mut settings,
                &mut chord_color_background_query,
                &mut bar_indicator_query,
                &mut pos_indicator_query,
                &mut tab_bars_query,
                playing,
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
        (Entity, &SingleData<LaneEntry>, &mut EntryPlaying),
        With<EntryPlaying>,
    >,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &TabBars,
        &LayoutData,
        &GridData,
    )>,
) {
    if theme._bypass_systems {
        return;
    }
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
    chord_color_background_query: &mut Query<
        (Entity, &mut ColorBackground),
        With<ChordColorBackground>,
    >,
    bar_indicator_query: &mut Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
    pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    bar_playing_query: &mut Query<(Entity, &mut BarPlaying), With<BarPlaying>>,
    entry_playing_query: &mut Query<
        (Entity, &SingleData<LaneEntry>, &mut EntryPlaying),
        With<EntryPlaying>,
    >,
    chord_playing_query: &mut Query<(Entity, &mut ChordPlaying), With<ChordPlaying>>,
    tab_bars_query: &mut Query<(
        Entity,
        &mut Transform,
        &TabBars,
        &LayoutData,
        &GridData,
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
    }
    let playing_bar_ordinal = new_position.bar.bar_ordinal;
    BarPlaying::update(bar_playing_query, tab_state, playing_bar_ordinal);
    EntryPlaying::update_with_pos(
        entry_playing_query,
        tab_state,
        new_position,
        *end_passed,
        *jumped,
    );
    let chord_changed = ChordPlaying::update(chord_playing_query, tab_state, new_position);
    if let Some(pos_data) =
        PosIndicatorData::update_pos(commands, theme, pos_indicator_query, *new_position)
    {
        if settings.layout.mode == LayoutMode::Line && pos_data.is_synced() {
            settings
                .layout
                .focus_bar(commands, theme, tab_bars_query, &pos_data);
        }
        if chord_changed > 0 {
            if let Some(bar_data) = BarIndicatorData::update_pos(
                commands,
                theme,
                bar_indicator_query,
                pos_data.bar_props,
                pos_data.bar_position.in_bar_pos,
            ) {
                ChordColorBackground::update_color(
                    commands,
                    theme,
                    chord_color_background_query,
                    bar_data.chord,
                );
            }
        }
    }
}

fn on_play_control_evt(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut settings: ResMut<NotationSettings>,
    mut evts: EventReader<PlayControlEvent>,
    mut tab_state_query: Query<(Entity, &mut TabState)>,
    mut chord_color_background_query: Query<
        (Entity, &mut ColorBackground),
        With<ChordColorBackground>,
    >,
    mut bar_indicator_query: Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
    mut pos_indicator_query: Query<(Entity, &mut PosIndicatorData), With<PosIndicatorData>>,
    mut bar_playing_query: Query<(Entity, &mut BarPlaying), With<BarPlaying>>,
    mut entry_playing_query: Query<
        (Entity, &SingleData<LaneEntry>, &mut EntryPlaying),
        With<EntryPlaying>,
    >,
    mut chord_playing_query: Query<(Entity, &mut ChordPlaying), With<ChordPlaying>>,
    mut tab_bars_query: Query<(
        Entity,
        &mut Transform,
        &TabBars,
        &LayoutData,
        &GridData,
    )>,
    mut beat_query: Query<(Entity, &mut BarBeatData)>,
) {
    if theme._bypass_systems {
        return;
    }
    for evt in evts.read() {
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
                    &mut chord_color_background_query,
                    &mut bar_indicator_query,
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
                PlayControlEvent::OnShouldLoop(should_loop) => {
                    tab_state.set_should_loop(*should_loop);
                }
            }
        }
    }
}
