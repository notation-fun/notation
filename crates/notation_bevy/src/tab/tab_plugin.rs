use std::sync::Arc;

use bevy::prelude::*;
use notation_bevy_utils::prelude::{LayoutData, GridData};
use notation_midi::prelude::{JumpToBarEvent, MidiState, PlayControlEvent};
use notation_model::prelude::TabBarProps;

use crate::bar::bar_view::BarView;
use crate::chord::chord_view::ChordView;
use crate::mini::mini_bar::MiniBar;

use crate::play::play_button::PlayButton;
use crate::prelude::{
    AddTabEvent, MouseClickedEvent, MouseDraggedEvent, NotationAppState, NotationAssetsStates,
    NotationSettings, NotationTheme, TabAsset, TabBars, TabState,
};
use crate::rhythm::rhythm_bar::{RhythmBarData};
use crate::rhythm::rhythm_view::RhythmView;
use crate::viewer::control::ControlView;

use super::tab_asset::TabAssetLoader;

use super::tab_chords::TabChords;
use super::tab_content::TabContent;
use super::tab_control::TabControl;
use super::tab_events::{RhythmViewDoLayoutEvent, TabBarsDoLayoutEvent, TabBarsResizedEvent, TabBarsResizedPreEvent, TabChordsDoLayoutEvent, TabContentDoLayoutEvent, TabControlDoLayoutEvent, TabHeaderDoLayoutEvent, TabViewDoLayoutEvent};
use super::tab_header::TabHeader;
use super::tab_view::TabView;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        TabViewDoLayoutEvent::setup(app);
        TabContentDoLayoutEvent::setup(app);
        TabHeaderDoLayoutEvent::setup(app);
        TabControlDoLayoutEvent::setup(app);
        TabChordsDoLayoutEvent::setup(app);
        TabBarsDoLayoutEvent::setup(app);
        RhythmViewDoLayoutEvent::setup(app);
        app.add_event::<AddTabEvent>();
        app.add_event::<TabBarsResizedEvent>();
        app.add_event::<TabBarsResizedPreEvent>();
        app.add_asset::<TabAsset>();
        app.init_asset_loader::<TabAssetLoader>();
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(on_mouse_clicked.system())
                .with_system(on_mouse_dragged.system())
                .with_system(TabView::do_layout.system())
                .with_system(TabContent::do_layout.system())
                .with_system(TabHeader::do_layout.system())
                .with_system(TabControl::do_layout.system())
                .with_system(RhythmView::do_layout.system())
                .with_system(RhythmBarData::update_rhythm.system())
                .with_system(TabChords::do_layout.system())
                .with_system(TabBars::on_resized_pre.system())
                .with_system(TabBars::do_layout.system()),
        );
    }
}

fn jump_to_bar(
    jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>,
    bar_props: TabBarProps,
) {
    jump_to_bar_evts.send(JumpToBarEvent::new(bar_props));
}

fn on_mouse_clicked(
    mut evts: EventReader<MouseClickedEvent>,
    _theme: Res<NotationTheme>,
    mut app_state: ResMut<NotationAppState>,
    mut settings: ResMut<NotationSettings>,
    tab_state_query: Query<(Entity, &TabState), With<TabState>>,
    mini_bar_query: Query<(&Arc<MiniBar>, &LayoutData, &GlobalTransform)>,
    button_query: Query<(&Arc<PlayButton>, &LayoutData, &GlobalTransform)>,
    rhythm_query: Query<(&Arc<RhythmView>, &LayoutData, &GlobalTransform)>,
    chord_query: Query<(&Arc<ChordView>, &LayoutData, &GlobalTransform)>,
    bar_query: Query<(&Arc<BarView>, &LayoutData, &GlobalTransform)>,
    mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
    mut midi_state: ResMut<MidiState>,
    mut play_control_evts: EventWriter<PlayControlEvent>,
) {
    let mut pos = None;
    for evt in evts.iter() {
        pos = Some(app_state.convert_pos(evt.cursor_position));
    }
    if let Some(pos) = pos {
        if !app_state.hide_control {
            if app_state.window_width / 2.0 - pos.x > ControlView::calc_width(app_state.window_width) {
                app_state.hide_control = true;
            }
        } else {
            println!("tab_plugin::on_mouse_clicked() -> {:?}", pos);
            for (mini_bar, layout, global_transform) in mini_bar_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    jump_to_bar( &mut jump_to_bar_evts, mini_bar.bar_props);
                    return;
                }
            }
            for (button, layout, global_transform) in button_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    match button.action {
                        crate::play::play_button::PlayButtonAction::PlayPause =>
                            crate::viewer::control::ControlView::play_or_pause(&mut midi_state, &mut play_control_evts),
                        crate::play::play_button::PlayButtonAction::Stop =>
                            crate::viewer::control::ControlView::stop(&mut midi_state, &mut play_control_evts),
                        crate::play::play_button::PlayButtonAction::LoopMode => {
                            settings.should_loop = !settings.should_loop;
                            crate::viewer::control::ControlView::sync_should_loop(&settings, &mut midi_state, &mut play_control_evts)
                        }
                        crate::play::play_button::PlayButtonAction::SetBegin =>
                            crate::viewer::control::ControlView::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts),
                        crate::play::play_button::PlayButtonAction::SetEnd =>
                            crate::viewer::control::ControlView::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts),
                        crate::play::play_button::PlayButtonAction::Clear =>
                            crate::viewer::control::ControlView::clear_begin_end(&mut midi_state, &mut play_control_evts),
                    }
                    return;
                }
            }
            for (_rhythm_view, layout, global_transform) in rhythm_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    if app_state.hide_control {
                        app_state.hide_control = false;
                    }
                    return;
                }
            }
            for (chord, layout, global_transform) in chord_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    let position =
                        TabState::get_position(&tab_state_query, chord.chord.tab().map(|x| x.uuid));
                    if let Some(next_bar) = chord.chord.search_next(true, position) {
                        jump_to_bar(&mut jump_to_bar_evts, next_bar.props);
                    }
                    return;
                }
            }
            for (bar, layout, global_transform) in bar_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    jump_to_bar(&mut jump_to_bar_evts, bar.bar_props);
                    return;
                }
            }
        }
    }
}

fn on_mouse_dragged(
    mut evts: EventReader<MouseDraggedEvent>,
    theme: Res<NotationTheme>,
    settings: Res<NotationSettings>,
    mut tab_bars_query: Query<(
            Entity,
            &mut Transform,
            &Arc<TabBars>,
            &LayoutData,
            &Arc<GridData>,
        )>,
) {
    for evt in evts.iter() {
        if settings.allow_panning {
            settings
                .layout
                .pan_tab_bars(&theme, &mut tab_bars_query, -evt.delta.x, -evt.delta.y);
        }
    }
}
