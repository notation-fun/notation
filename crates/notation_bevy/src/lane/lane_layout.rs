use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LaneLayoutData {
    pub order: u8,
    pub offset: f32,
    pub height: f32,
    pub margin: f32,
}
impl LaneLayoutData {
    pub fn new(order: u8, offset: f32, height: f32, margin: f32) -> Self {
        Self {
            order,
            offset,
            height,
            margin,
        }
    }
    pub fn calc_transform(&self) -> Transform {
        Transform::from_xyz(0.0, self.offset, 0.0)
    }
}
