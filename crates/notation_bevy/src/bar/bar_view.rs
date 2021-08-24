use std::sync::Arc;

use bevy::prelude::*;

use crate::lane::lane_view::LaneView;
use crate::prelude::{
    AddEntryEvent, BarData, BarLayoutData, NotationAppState, NotationSettings, NotationTheme,
};
use crate::strings::pick_note::{PickNoteData, PickNoteShape};
use crate::strings::single_string::{SingleString, SingleStringData};
use crate::tab::tab_events::BarViewDoLayoutEvent;
use crate::tone::tone_note::{ToneNoteData, ToneNoteShape};
use crate::ui::layout::NotationLayout;
use crate::word::word_text::{WordTextData, WordTextShape};
use bevy_utils::prelude::{GridCell, LayoutQuery, LyonShapeOp, VBoxView, View, ViewQuery};
use notation_model::prelude::TabBar;

use super::bar_beat::{BarBeat, BarBeatData, BarBeatValue};
use super::bar_separator::{BarSeparator, BarSeparatorData, BarSeparatorValue};

pub type BarView = BarData<BarLayoutData>;

impl<'a> View<NotationLayout<'a>> for BarView {}
impl<'a> GridCell<NotationLayout<'a>> for BarView {
    fn order(&self) -> usize {
        self.bar_props.bar_ordinal
    }
}
impl<'a> VBoxView<NotationLayout<'a>, LaneView> for BarView {
    fn sort_cells(&self) -> bool {
        true
    }
}

impl BarView {
    pub fn do_layout(
        mut evts: EventReader<BarViewDoLayoutEvent>,
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<LaneView>,
        mut sep_query: Query<(Entity, &mut BarSeparatorData)>,
        mut beat_query: Query<(Entity, &mut BarBeatData)>,
        mut tone_note_query: Query<(Entity, &mut ToneNoteData)>,
        mut pick_note_query: Query<(Entity, &mut PickNoteData)>,
        mut single_string_query: Query<(Entity, &mut SingleStringData)>,
        mut word_text_query: Query<(Entity, &mut WordTextData)>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &mut commands,
                &engine,
                &mut layout_query,
                &cell_query,
                evt.entity,
                evt.layout,
            );
            for (entity, mut data) in sep_query.iter_mut() {
                if data.bar_props.bar_ordinal == evt.view.bar_props.bar_ordinal {
                    data.value.bar_size = evt.layout.size;
                    BarSeparator::update(&mut commands, &theme, entity, &data);
                }
            }
            for (entity, mut data) in beat_query.iter_mut() {
                if data.bar_props.bar_ordinal == evt.view.bar_props.bar_ordinal {
                    data.value.bar_size = evt.layout.size;
                    BarBeat::update(&mut commands, &theme, entity, &data);
                }
            }
            for (entity, mut data) in tone_note_query.iter_mut() {
                if data.bar_props.bar_ordinal == evt.view.bar_props.bar_ordinal {
                    data.value.bar_size = evt.layout.size.width;
                    ToneNoteShape::update(&mut commands, &theme, entity, &data);
                }
            }
            for (entity, mut data) in pick_note_query.iter_mut() {
                if data.bar_props.bar_ordinal == evt.view.bar_props.bar_ordinal {
                    data.value.bar_size = evt.layout.size.width;
                    PickNoteShape::update(&mut commands, &theme, entity, &data);
                }
            }
            for (entity, mut data) in single_string_query.iter_mut() {
                if data.bar_props.bar_ordinal == evt.view.bar_props.bar_ordinal {
                    data.value.bar_size = evt.layout.size.width;
                    SingleString::update(&mut commands, &theme, entity, &data);
                }
            }
            for (entity, mut data) in word_text_query.iter_mut() {
                if data.bar_props.bar_ordinal == evt.view.bar_props.bar_ordinal {
                    data.value.bar_size = evt.layout.size.width;
                    WordTextShape::update(&mut commands, &theme, entity, &data);
                }
            }
        }
    }
    pub fn on_added(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: Query<(Entity, &Arc<BarView>, &Arc<TabBar>, &BarLayoutData), Added<Arc<BarView>>>,
        mut add_entry_evts: EventWriter<AddEntryEvent>,
    ) {
        for (entity, _view, bar, bar_layout) in query.iter() {
            for lane_layout in bar_layout.lane_layouts.iter() {
                LaneView::spawn(
                    &mut commands,
                    entity,
                    &bar,
                    &mut add_entry_evts,
                    lane_layout,
                );
            }
            if false {
                //TODO
                BarSeparator::create(
                    &mut commands,
                    &theme,
                    entity,
                    BarSeparatorData::new(bar, BarSeparatorValue::new(true)),
                );
            }
            BarSeparator::create(
                &mut commands,
                &theme,
                entity,
                BarSeparatorData::new(bar, BarSeparatorValue::new(false)),
            );
            let signature = bar.signature();
            for beat in 0..signature.bar_beats {
                BarBeatValue::may_new(&theme, bar, &signature, beat)
                    .map(|value| BarBeatData::new(bar, value))
                    .map(|data| BarBeat::create(&mut commands, &theme, entity, data));
            }
        }
    }
}
