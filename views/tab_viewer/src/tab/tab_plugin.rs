use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::prelude::{GridData, LayoutData};
use notation_model::prelude::TabBarProps;
use notation_midi::prelude::{JumpToBarEvent, PlayControlEvent};

use crate::bar::bar_view::BarView;
use crate::chord::chord_view::ChordView;
use crate::mini::mini_bar::MiniBar;

#[cfg(feature = "with_egui")]
use crate::notation::egui_control_panel::EguiControlPanel;

use crate::play::play_button::PlayButton;
use crate::prelude::{
    AddTabEvent, MouseClickedEvent, MouseDraggedEvent, NotationState, NotationAssetsStates,
    NotationSettings, NotationTheme, TabAsset, TabBars, TabState,
};
use crate::rhythm::rhythm_bar::RhythmBarData;
use crate::rhythm::rhythm_view::RhythmView;

use super::tab_asset::TabAssetLoader;

use super::tab_chords::TabChords;
use super::tab_content::TabContent;
use super::tab_control::TabControl;
use super::tab_events::{
    RhythmViewDoLayoutEvent, TabBarsDoLayoutEvent, TabBarsResizedEvent, TabBarsResizedPreEvent,
    TabChordsDoLayoutEvent, TabContentDoLayoutEvent, TabControlDoLayoutEvent,
    TabHeaderDoLayoutEvent, TabViewDoLayoutEvent,
};
use super::tab_header::TabHeader;
use super::tab_view::TabView;

#[cfg(feature = "midi")]
use notation_midi::prelude::{MidiSettings, MidiState};

#[cfg(feature = "midi")]
use crate::midi::midi_control::MidiControl;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut App) {
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
        #[cfg(feature = "dsl")]
        app.init_asset_loader::<crate::dsl::get_tab_asset::GetTabAssetLoader>();
        app.add_systems(Update, (
            TabView::do_layout,
            TabContent::do_layout,
            TabHeader::do_layout,
            TabControl::do_layout,
            RhythmView::do_layout,
            RhythmBarData::update_rhythm,
            TabChords::do_layout,
            TabBars::on_resized_pre,
            TabBars::do_layout,
        ).run_if(in_state(NotationAssetsStates::Loaded)));
    }
}

impl TabPlugin {
    pub fn setup_mouse_input(app: &mut App) {
        app.add_systems(Update, (
            Self::on_mouse_clicked,
            Self::on_mouse_dragged,
        ).run_if(in_state(NotationAssetsStates::Loaded)));
    }
    pub fn jump_to_bar(jump_to_bar_evts: &mut EventWriter<JumpToBarEvent>, bar_props: TabBarProps) {
        jump_to_bar_evts.send(JumpToBarEvent::new(bar_props));
    }
    fn on_mouse_clicked(
        mut evts: EventReader<MouseClickedEvent>,
        theme: Res<NotationTheme>,
        mut app_state: ResMut<NotationState>,
        mut settings: ResMut<NotationSettings>,
        tab_state_query: Query<(Entity, &TabState), With<TabState>>,
        mini_bar_query: Query<(&MiniBar, &LayoutData, &GlobalTransform)>,
        button_query: Query<(&PlayButton, &LayoutData, &GlobalTransform)>,
        rhythm_query: Query<(&RhythmView, &LayoutData, &GlobalTransform)>,
        chord_query: Query<(&ChordView, &LayoutData, &GlobalTransform)>,
        bar_query: Query<(&BarView, &LayoutData, &GlobalTransform)>,
        tab_control_query: Query<(&TabControl, &LayoutData, &GlobalTransform)>,
        mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
        #[cfg(feature = "midi")]
        midi_settings: Res<MidiSettings>,
        #[cfg(feature = "midi")]
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
    ) {
        if theme._bypass_systems {
            return;
        }
        let mut pos = None;
        for evt in evts.iter() {
            pos = Some(app_state.convert_pos(evt.cursor_position));
        }
        if let Some(pos) = pos {
            if app_state.show_control {
                #[cfg(feature = "with_egui")]
                if !EguiControlPanel::is_pos_inside(app_state.window_width, pos) {
                    app_state.show_control = false;
                }
            } else if app_state.show_kb {
                //TODO: after #125 done, can pass click event in case of not inside help panel
            } else {
                println!("tab_plugin::on_mouse_clicked() -> {:?}", pos);
                for (mini_bar, layout, global_transform) in mini_bar_query.iter() {
                    if layout.is_pos_inside(pos, global_transform) {
                        Self::jump_to_bar(&mut jump_to_bar_evts, mini_bar.bar_props);
                        return;
                    }
                }
                for (button, layout, global_transform) in button_query.iter() {
                    if layout.is_pos_inside(pos, global_transform) {
                        #[cfg(feature = "midi")]
                        Self::on_play_button_clicked(&mut settings, &midi_settings, &mut midi_state, &mut play_control_evts, button);
                        return;
                    }
                }
                for (_rhythm_view, layout, global_transform) in rhythm_query.iter() {
                    if layout.is_pos_inside(pos, global_transform) {
                        if !app_state.show_control {
                            app_state.show_control = true;
                        }
                        return;
                    }
                }
                for (chord, layout, global_transform) in chord_query.iter() {
                    if layout.is_pos_inside(pos, global_transform) {
                        let position =
                            TabState::get_position(&tab_state_query, chord.chord.tab().map(|x| x.uuid));
                        if let Some(next_bar) = chord.chord.search_next(true, position) {
                            Self::jump_to_bar(&mut jump_to_bar_evts, next_bar.props);
                        }
                        return;
                    }
                }
                for (bar, layout, global_transform) in bar_query.iter() {
                    if layout.is_pos_inside(pos, global_transform) {
                        Self::jump_to_bar(&mut jump_to_bar_evts, bar.bar_props);
                        return;
                    }
                }
                // Not using GuitarView here, since it's y position been changed to adjust with capo position
                for (_tab_control, layout, global_transform) in tab_control_query.iter() {
                    if layout.is_pos_inside(pos, global_transform) {
                        #[cfg(feature = "midi")]
                        MidiControl::seek_forward(&midi_settings, &mut midi_state, &mut play_control_evts);
                        return;
                    }
                }
            }
        }
    }

    fn on_mouse_dragged(
        mut evts: EventReader<MouseDraggedEvent>,
        app_state: Res<NotationState>,
        theme: Res<NotationTheme>,
        settings: Res<NotationSettings>,
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
        for evt in evts.iter() {
            let pos = app_state.convert_pos(evt.cursor_position);
            #[cfg(feature = "with_egui")]
            if app_state.show_control && EguiControlPanel::is_pos_inside(app_state.window_width, pos) {
                return;
            }
            if app_state.show_kb {
                //TODO: after #125 done, can pass drag event in case of not inside help panel
                return;
            }
            if settings.allow_panning {
                settings
                    .layout
                    .pan_tab_bars(&theme, &mut tab_bars_query, -evt.delta.x, -evt.delta.y);
            }
        }
    }
}
