use bevy::prelude::*;

use crate::prelude::{DoLayoutEvent, LayoutChangedQuery, LayoutEnv, View};

pub struct BevyUtil();

impl BevyUtil {
    pub fn spawn_child_bundle<T: Bundle>(
        commands: &mut Commands,
        entity: Entity,
        bundle: T,
    ) -> Entity {
        let child_entity = commands.spawn_bundle(bundle).id();
        commands.entity(entity).push_children(&[child_entity]);
        child_entity
    }
    /*
    pub fn get_sibling(
        entity: Entity,
        depth: usize,
        lane_kind: LaneKind,
        lane_queries: (&Query<&Parent>, &Query<&Children>, &Query<&Arc<BarLane>>),
    ) -> Option<(Entity, Arc<BarLane>)> {
        let mut current_entity = entity;
        for i in 0..depth {
            if let Ok(parent) = lane_queries.0.get(current_entity) {
                current_entity = parent.0;
            } else {
                println!(
                    "BarPlugin::get_lane({:?}, {}, {}) Parent Not Found: {}",
                    entity, depth, lane_kind, i
                );
                return None;
            }
        }
        if let Ok(children) = lane_queries.1.get(current_entity) {
            if children.len() == 0 {
                println!(
                    "BarPlugin::get_lane({:?}, {}, {}) Children Is Empty: {:?}",
                    entity, depth, lane_kind, current_entity
                );
            }
            for &child in children.iter() {
                if let Ok(lane) = lane_queries.2.get(child) {
                    if lane.kind == lane_kind {
                        //println!("BarPlugin::get_lane({:?}, {}, {}) Found: {}", entity, depth, lane_kind, lane);
                        return Some((child, lane.clone()));
                    } else {
                        println!(
                            "BarPlugin::get_lane({:?}, {}, {}) BarLane Not Matched: {}",
                            entity, depth, lane_kind, lane
                        );
                    }
                } else {
                    println!(
                        "BarPlugin::get_lane({:?}, {}, {}) BarLane Not Found: {:?}",
                        entity, depth, lane_kind, child
                    );
                }
            }
        } else {
            println!(
                "BarPlugin::get_lane({:?}, {}, {}) Children Not Found: {:?}",
                entity, depth, lane_kind, current_entity
            );
        }
        None
    }
     */
}
