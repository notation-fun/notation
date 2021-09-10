use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::LayoutData;
use notation_midi::prelude::JumpToBarEvent;

use crate::bar::bar_view::BarView;
use crate::mini::mini_bar::MiniBar;

use crate::prelude::{
    AddTabEvent, MouseClickedEvent, MouseDraggedEvent, NotationAppState, NotationAssetsStates,
    NotationSettings, NotationTheme, TabAsset, TabBars,
};
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
                .with_system(RhythmBar::update_chord.system())
                .with_system(TabChords::do_layout.system())
                .with_system(TabBars::on_resized_pre.system())
                .with_system(TabBars::do_layout.system()),
        );
    }
}

fn on_mouse_clicked(
    mut evts: EventReader<MouseClickedEvent>,
    _theme: Res<NotationTheme>,
    mut state: ResMut<NotationAppState>,
    settings: Res<NotationSettings>,
    mini_bar_query: Query<(&Arc<MiniBar>, &LayoutData, &GlobalTransform)>,
    bar_query: Query<(&Arc<BarView>, &LayoutData, &GlobalTransform)>,
    mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
) {
    let mut pos = None;
    for evt in evts.iter() {
        pos = Some(state.convert_pos(evt.cursor_position));
    }
    if let Some(pos) = pos {
        if !state.hide_control {
            if state.window_width / 2.0 - pos.x > ControlView::WIDTH {
                state.hide_control = true;
            }
        } else if !settings.mouse_dragged_panning {
            println!("tab_plugin::on_mouse_clicked() -> {:?}", pos);
            for (mini_bar, layout, global_transform) in mini_bar_query.iter() {
                let offset = pos
                    - Vec2::new(
                        global_transform.translation.x,
                        global_transform.translation.y,
                    );
                if layout.is_inside(offset) {
                    jump_to_bar_evts.send(JumpToBarEvent::new(mini_bar.bar_props));
                    return;
                }
            }
            for (bar, layout, global_transform) in bar_query.iter() {
                let offset = pos
                    - Vec2::new(
                        global_transform.translation.x,
                        global_transform.translation.y,
                    );
                if layout.is_inside(offset) {
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
