use std::sync::Arc;

use bevy::render::camera::OrthographicProjection;
#[cfg(target_arch = "wasm32")]
use instant::Duration as StdDuration;
#[cfg(target_arch = "wasm32")]
use instant::Instant as StdInstant;
use notation_midi::prelude::{PlayToneEvent, StopToneEvent};
use notation_model::prelude::Tone;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration as StdDuration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant as StdInstant;

use bevy::prelude::*;
use notation_model::prelude::{BarPosition, Duration, ModelEntry, Tab};

use crate::prelude::BarLayout;
use crate::prelude::{
    EntryState, LyonShapeOp, NotationSettings, NotationTheme, TabState, WindowResizedEvent,
};
use crate::tab::tab_state::TabPlayStateChanged;

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
        app.add_system(play_stop_tone.system());
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
    state_query: Query<(&Parent, Entity, &Arc<Vec<BarLayout>>), Added<TabState>>,
    tab_query: Query<&Arc<Tab>>,
) {
    for (parent, entity, bar_layouts) in state_query.iter() {
        if let Ok(tab) = tab_query.get(parent.0) {
            if let Some(bar_layout) = bar_layouts.get(0) {
                let data = PosIndicatorData::new(tab.bar_units(), bar_layout);
                PosIndicator::create(&mut commands, entity, &theme, data);
            }
        }
    }
}

fn on_stop(
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut query: Query<(Entity,&Arc<Vec<BarLayout>>, &TabState, &Children), Added<TabPlayStateChanged>>,
    mut pos_indicator_query: Query<&mut PosIndicatorData>,
    mut entry_query: Query<(Entity, &Arc<ModelEntry>, &BarPosition, &mut EntryState)>,
    mut camera_query: Query<(&mut Transform, &OrthographicProjection)>,
) {
    for (state_entity, bar_layouts, state, children) in query.iter_mut() {
        TabState::clear_play_state_changed(&mut commands, state_entity);
        if !state.play_state.is_playing() {
            PosIndicator::update_pos(&mut commands, &theme, children,
                &settings, &mut pos_indicator_query, bar_layouts, state.pos);
            settings.layout.focus_camera(&mut camera_query, bar_layouts, state.pos, theme.grid.bar_size);
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
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut time: ResMut<NotationTime>,
    mut query: Query<(Entity, &Arc<Vec<BarLayout>>, &mut TabState, &Children)>,
    mut pos_indicator_query: Query<&mut PosIndicatorData>,
    mut entry_query: Query<(
        Entity,
        &Arc<ModelEntry>,
        &Duration,
        &BarPosition,
        &mut EntryState,
    )>,
    camera_query: Query<(Entity, &Transform, &OrthographicProjection)>,
) {
    time.tick();
    for (state_entity, bar_layouts, mut state, children) in query.iter_mut() {
        let old_position = state.pos;
        let (changed, end_passed) = state.tick(&mut commands, state_entity, time.delta_seconds());
        if changed {
            PosIndicator::update_pos(&mut commands, &theme, children,
                &settings, &mut pos_indicator_query, bar_layouts, state.pos);
            //settings.layout.focus_camera(&mut camera_query, bar_layouts, state.pos, theme.grid.bar_size);
            if old_position.bar.bar_ordinal != state.pos.bar.bar_ordinal {
                settings.layout.focus_camera_by_ease(&mut commands, &camera_query, bar_layouts, state.pos, theme.grid.bar_size);
            }
            for (_entity, _entry, duration, position, mut entry_state) in entry_query.iter_mut() {
                if state.is_in_range(position) {
                    if entry_state.is_playing() && state.pos.is_passed_with(position, duration) {
                        *entry_state = EntryState::Played;
                    }
                    if entry_state.is_idle() && state.pos.is_passed(position) {
                        *entry_state = EntryState::Playing;
                    }
                    if end_passed {
                        if entry_state.is_played()
                            || position.bar_ordinal > state.pos.bar.bar_ordinal
                        {
                            *entry_state = EntryState::Idle;
                        }
                    }
                }
            }
        }
    }
}

fn play_stop_tone(
    mut _commands: Commands,
    _theme: Res<NotationTheme>,
    query: Query<(&Tone, &EntryState), Changed<EntryState>>,
    mut play_note_evts: EventWriter<PlayToneEvent>,
    mut stop_note_evts: EventWriter<StopToneEvent>,
) {
    for (tone, state) in query.iter() {
        if !tone.is_none() {
            if state.is_played() || state.is_idle() {
                stop_note_evts.send(StopToneEvent(*tone));
            } else if state.is_playing() {
                play_note_evts.send(PlayToneEvent(*tone));
            }
        }
    }
}
