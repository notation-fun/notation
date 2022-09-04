use bevy::prelude::*;

use super::BevyUtil;

impl BevyUtil {
    pub fn offscreen_offset_2() -> Vec2 {
        Vec2::new(-999999.0, -999999.0)
    }
    pub fn offscreen_offset() -> Vec3 {
        Vec3::new(-999999.0, -999999.0, -999999.0)
    }
    pub fn offscreen_transform() -> Transform {
        Transform {
            translation: Vec3::new(-999999.0, -999999.0, -999999.0),
            ..Default::default()
        }
    }
    pub fn cap_str(name: String, max: usize) -> String {
        if name.len() <= max {
            name.as_str().into()
        } else {
            name.as_str()[..max].into()
        }
    }
    pub fn calc_name(name: String) -> Name {
        if name.len() <= 32 {
            name.as_str().into()
        } else {
            name.as_str()[..32].into()
        }
    }
    pub fn spawn_child_bundle<T: Bundle>(
        commands: &mut Commands,
        entity: Entity,
        bundle: T,
    ) -> Entity {
        let child_entity = commands.spawn_bundle(bundle).id();
        commands.entity(entity).push_children(&[child_entity]);
        child_entity
    }
    pub fn in_range(v: f32, range: (f32, f32)) -> f32 {
        if v < range.0 {
            range.0
        } else if v > range.1 {
            range.1
        } else {
            v
        }
    }
    pub fn in_range_with_margin(v: f32, range: (f32, f32), margin: f32) -> f32 {
        Self::in_range(v, (range.0 + margin, range.1 + margin))
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
                current_entity = parent.get();
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
