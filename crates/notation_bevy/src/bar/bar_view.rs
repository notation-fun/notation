use std::sync::Arc;

use bevy::prelude::*;

use crate::lane::lane_view::LaneView;
use crate::prelude::{BarBundle, BarData, BarLayoutData, NotationAppState, NotationAssets, NotationSettings, NotationTheme};
use crate::tab::tab_events::{BarViewDoLayoutEvent, TabBarsResizedEvent};
use crate::ui::layout::NotationLayout;
use bevy_utils::prelude::{BevyUtil, GridCell, LayoutQuery, LyonShapeOp, VBoxView, View, ViewQuery};
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
        mut tab_resized_evts: EventWriter<TabBarsResizedEvent>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        let mut bars = Vec::new();
        for evt in evts.iter() {
            evt.view.do_layout(
                &mut commands,
                &engine,
                &mut layout_query,
                &cell_query,
                evt.entity,
                evt.layout,
            );
            bars.push((evt.view.clone(), evt.layout.clone()));
        }
        for (entity, mut data) in sep_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size;
                    BarSeparator::update(&mut commands, &theme, entity, &data);
                }
            }
        }
        for (entity, mut data) in beat_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size;
                    BarBeat::update(&mut commands, &theme, entity, &data);
                }
            }
        }
        tab_resized_evts.send(TabBarsResizedEvent(bars));
    }
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        bar: &Arc<TabBar>,
        bar_layout: &BarLayoutData,
    ) -> Entity {
        let bar_bundle = BarBundle::new(bar.clone(), bar_layout.clone());
        let bar_entity = BevyUtil::spawn_child_bundle(commands, entity, bar_bundle);
        for lane_layout in bar_layout.lane_layouts.iter() {
            LaneView::spawn(
                commands,
                assets,
                theme,
                settings,
                bar_entity,
                &bar,
                lane_layout,
            );
        }
        //TODO, create bar separator for the first one in row
        if false {
            BarSeparator::create(
                commands,
                theme,
                bar_entity,
                BarSeparatorData::new(bar, BarSeparatorValue::new(true)),
            );
        }
        BarSeparator::create(
            commands,
            theme,
            bar_entity,
            BarSeparatorData::new(bar, BarSeparatorValue::new(false)),
        );
        let signature = bar.signature();
        for beat in 0..signature.bar_beats {
            BarBeatValue::may_new(theme, bar, &signature, beat)
                .map(|value| BarBeatData::new(bar, value))
                .map(|data| BarBeat::create(commands, theme, bar_entity, data));
        }
        bar_entity
    }
}
