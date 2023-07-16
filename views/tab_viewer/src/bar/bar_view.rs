use std::sync::Arc;

use edger_bevy_app::bevy_prelude::*;

use crate::lane::lane_view::LaneView;
use crate::prelude::{
    BarBundle, BarData, BarLayoutData, NotationState, NotationAssets, NotationSettings,
    NotationTheme,
};
use crate::tab::tab_events::BarViewDoLayoutEvent;
use crate::prelude::NotationLayout;
use edger_bevy_app::prelude::{
    entity, GridCell, LayoutQuery, ShapeOp, VBoxView, View, ViewQuery,
};
use notation_model::prelude::TabBar;

use super::bar_beat::{BarBeatData, BarBeatValue};
use super::bar_separator::{BarSeparatorData, BarSeparatorValue};

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
        state: Res<NotationState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<LaneView>,
        mut sep_query: Query<(Entity, &mut BarSeparatorData)>,
        mut beat_query: Query<(Entity, &mut BarBeatData)>,
    ) {
        if theme._bypass_systems {
            return;
        }
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
                    data.update(&mut commands, &theme, entity);
                }
            }
        }
        for (entity, mut data) in beat_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size;
                    data.update(&mut commands, &theme, entity);
                }
            }
        }
    }
    pub fn update_number_text(
        theme: Res<NotationTheme>,
        settings: Res<NotationSettings>,
        mut evts: EventReader<BarViewDoLayoutEvent>,
        mut text_query: Query<(&Parent, &mut Transform), With<Text>>,
    ) {
        if theme._bypass_systems {
            return;
        }
        if !settings.hide_bar_number {
            for evt in evts.iter() {
                for (parent, mut transform) in text_query.iter_mut() {
                    if parent.get() == evt.entity {
                        theme
                            .texts
                            .tab
                            .update_bar_number_x(&mut transform, evt.layout.size.width);
                    }
                }
            }
        }
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
        let bar_entity = entity::spawn_child_bundle(commands, entity, bar_bundle);
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
            let data = BarSeparatorData::new(bar, BarSeparatorValue::new(true));
            data.create(commands, theme, bar_entity);
        }
        let data = BarSeparatorData::new(bar, BarSeparatorValue::new(false));
        data.create(commands, theme, bar_entity);
        let signature = bar.signature();
        for beat in 0..signature.bar_beats {
            let data = BarBeatData::new(bar, BarBeatValue::new(bar, &signature, beat));
            data.create(commands, theme, bar_entity);
        }
        if !settings.hide_bar_number {
            theme.texts.tab.spawn_bar_number(
                commands,
                assets,
                bar_entity,
                bar.props.bar_number.to_string().as_str(),
            );
        }
        bar_entity
    }
}
