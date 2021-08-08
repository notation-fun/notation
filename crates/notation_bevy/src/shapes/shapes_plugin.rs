use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use notation_model::prelude::{BarLane, BarPosition};
use std::sync::Arc;

use super::hand_bundles::HandShapeBundle;
use super::shape_diagram::{ShapeDiagramData, ShapeDiagramShape};
use crate::prelude::{LyonShapeOp, NotationTheme, WindowResizedEvent};
use notation_model::prelude::{Fretboard, FrettedEntry, HandShape, TabBar};

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed::<6>.system());
        app.add_system(on_config_changed::<4>.system());
        app.add_system_set(super::hand_systems::new_system_set());
    }
}

fn on_config_changed<const S: usize>(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    shapes_query: Query<(Entity, &ShapeDiagramData<S>)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in shapes_query.iter() {
            ShapeDiagramShape::<S>::update(&mut commands, &theme, entity, data);
        }
    }
}

impl ShapesPlugin {
    pub fn insert_entry_extra<const S: usize>(
        commands: &mut EntityCommands,
        entry: &FrettedEntry<S>,
    ) {
        match entry {
            FrettedEntry::Shape(shape, _) => {
                commands.insert_bundle(HandShapeBundle::<S>::from(*shape));
            }
            _ => (),
        }
    }
    pub fn get_fretted_shape<const S: usize>(
        lane_entity: Entity,
        _position: &BarPosition,
        shape_queries: (
            &Query<(&Arc<TabBar>, &Arc<BarLane>, &Fretboard<S>, &Children)>,
            &Query<&HandShape<S>>,
        ),
    ) -> Option<(Arc<TabBar>, Fretboard<S>, HandShape<S>)> {
        if let Ok((bar, _lane, fretboard, children)) = shape_queries.0.get(lane_entity) {
            let mut matched_shape = None;
            for &child in children.iter() {
                if let Ok(shape) = shape_queries.1.get(child) {
                    //TODO: check shape duration aganst position
                    matched_shape = Some(shape);
                    break;
                }
            }
            if let Some(shape) = matched_shape {
                return Some((bar.clone(), *fretboard, *shape));
            }
        }
        None
    }
}
