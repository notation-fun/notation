use std::sync::Arc;

use bevy::prelude::*;

use bevy_utils::prelude::{
    GridCell, LayoutAnchor, LayoutChangedWithChildrenQuery, View, ViewAddedQuery,
};
use notation_model::prelude::Chord;

use crate::prelude::{ModelEntryData, NotationTheme};
use crate::ui::layout::NotationLayout;

use super::chord_base::ChordBaseData;
use super::chord_diagram::{ChordDiagram, ChordDiagramData};
use super::chord_interval::ChordIntervalData;
use super::chord_playing::ChordPlaying;
use super::interval_dot::IntervalDotData;

pub type ChordView = ModelEntryData<Chord>;

impl<'a> View<NotationLayout<'a>> for ChordView {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::CENTER
    }
}
impl<'a> GridCell<NotationLayout<'a>> for ChordView {
    fn order(&self) -> usize {
        0
    }
}

impl ChordView {
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedWithChildrenQuery<ChordView>,
        mut diagram_query: Query<(Entity, &mut ChordDiagramData, &Children)>,
        mut interval_query: Query<(Entity, &mut ChordIntervalData, &Children)>,
        mut base_query: Query<(Entity, &mut ChordBaseData, &Children)>,
        mut dot_query: Query<(Entity, &mut IntervalDotData)>,
    ) {
        for (_entity, _view, layout, children) in query.iter() {
            let radius = layout.size.width * theme.sizes.chord.diagram_factor;
            for child in children.iter() {
                if let Ok((diagram_entity, mut diagram_data, diagram_children)) =
                    diagram_query.get_mut(*child)
                {
                    ChordDiagram::update_size(
                        &mut commands,
                        &theme,
                        &mut interval_query,
                        &mut base_query,
                        &mut dot_query,
                        diagram_entity,
                        &mut diagram_data,
                        diagram_children,
                        radius,
                    );
                }
            }
        }
    }
    pub fn on_added(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: ViewAddedQuery<ChordView>,
    ) {
        for (_parent, entity, view) in query.iter() {
            //TODO: handle initialization in a nicer way.
            let radius = 0.0;
            ChordDiagram::spawn(
                &mut commands,
                &theme,
                entity,
                view.entry_props,
                view.value,
                radius,
            );
            commands
                .entity(entity)
                .insert(ChordPlaying::from((view.entry_props, view.value)));
        }
    }
    pub fn on_chord_playing_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        mut query: Query<
            (Entity, &ChordPlaying, &Arc<ChordView>, &Children),
            Changed<ChordPlaying>,
        >,
        mut diagram_query: Query<(Entity, &mut ChordDiagramData)>,
    ) {
        for (_entity, playing, _view, children) in query.iter_mut() {
            for child in children.iter() {
                if let Ok((diagram_entity, mut diagram_data)) = diagram_query.get_mut(*child) {
                    ChordDiagram::update_playing_state(
                        &mut commands,
                        &theme,
                        diagram_entity,
                        &mut diagram_data,
                        playing.value.state,
                    );
                }
            }
        }
    }
}
