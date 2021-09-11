use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::LayoutData;
use notation_midi::prelude::JumpToBarEvent;

use crate::bar::bar_view::BarView;
use crate::chord::chord_view::ChordView;
use crate::mini::mini_bar::MiniBar;

use crate::prelude::{AddTabEvent, MouseClickedEvent, MouseDraggedEvent, NotationAppState, NotationAssetsStates, NotationSettings, NotationTheme, TabAsset, TabBars, TabState};
use crate::rhythm::rhythm_bar::RhythmBar;
use crate::viewer::control::ControlView;

use super::tab_asset::TabAssetLoader;

use super::tab_chords::TabChords;
use super::tab_content::TabContent;
use super::tab_control::TabControl;
use super::tab_events::{TabBarsDoLayoutEvent, TabBarsResizedEvent, TabBarsResizedPreEvent, TabChordsDoLayoutEvent, TabContentDoLayoutEvent, TabHeaderDoLayoutEvent, TabViewDoLayoutEvent};
use super::tab_header::TabHeader;
use super::tab_view::TabView;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        TabViewDoLayoutEvent::setup(app);
        TabContentDoLayoutEvent::setup(app);
        TabHeaderDoLayoutEvent::setup(app);
        TabChordsDoLayoutEvent::setup(app);
        TabBarsDoLayoutEvent::setup(app);
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
                .with_system(TabControl::on_layout_changed.system())
                .with_system(RhythmBar::update_rhythm.system())
                .with_system(TabChords::do_layout.system())
                .with_system(TabBars::on_resized_pre.system())
                .with_system(TabBars::do_layout.system()),
        );
    }
}

fn on_mouse_clicked(
    mut evts: EventReader<MouseClickedEvent>,
    _theme: Res<NotationTheme>,
    mut app_state: ResMut<NotationAppState>,
    settings: Res<NotationSettings>,
    tab_state_query: Query<(Entity, &TabState), With<TabState>>,
    mini_bar_query: Query<(&Arc<MiniBar>, &LayoutData, &GlobalTransform)>,
    control_query: Query<(&Arc<TabControl>, &LayoutData, &GlobalTransform)>,
    chord_query: Query<(&Arc<ChordView>, &LayoutData, &GlobalTransform)>,
    bar_query: Query<(&Arc<BarView>, &LayoutData, &GlobalTransform)>,
    mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
) {
    let mut pos = None;
    for evt in evts.iter() {
        pos = Some(app_state.convert_pos(evt.cursor_position));
    }
    if let Some(pos) = pos {
        if !app_state.hide_control {
            if app_state.window_width / 2.0 - pos.x > ControlView::WIDTH {
                app_state.hide_control = true;
            }
        } else if !settings.mouse_dragged_panning {
            println!("tab_plugin::on_mouse_clicked() -> {:?}", pos);
            for (mini_bar, layout, global_transform) in mini_bar_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    jump_to_bar_evts.send(JumpToBarEvent::new(mini_bar.bar_props));
                    return;
                }
            }
            for (_control, layout, global_transform) in control_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    if app_state.hide_control {
                        app_state.hide_control = false;
                    }
                    return;
                }
            }
            for (chord, layout, global_transform) in chord_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    let position = TabState::get_position(&tab_state_query, chord.entry.tab().map(|x| x.uuid));
                    if let Some(next_bar) = chord.search_next(true, position) {
                        jump_to_bar_evts.send(JumpToBarEvent::new(next_bar.props));
                    }
                    return;
                }
            }
            for (bar, layout, global_transform) in bar_query.iter() {
                if layout.is_pos_inside(pos, global_transform) {
                    jump_to_bar_evts.send(JumpToBarEvent::new(bar.bar_props));
                    return;
                }
            }
        }
    }
}

fn on_mouse_dragged(
    mut evts: EventReader<MouseDraggedEvent>,
    settings: Res<NotationSettings>,
    mut tab_bars_query: Query<(Entity, &mut Transform, &Arc<TabBars>)>,
) {
    for evt in evts.iter() {
        if settings.mouse_dragged_panning {
            settings
                .layout
                .pan_tab_bars(&mut tab_bars_query, -evt.delta.x, -evt.delta.y);
        }
    }
}
