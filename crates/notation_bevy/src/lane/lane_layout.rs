use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LaneLayoutData {
    pub order: u8,
    pub height: f32,
    pub margin: f32,
}
impl LaneLayoutData {
    pub fn new(order: u8, height: f32, margin: f32) -> Self {
        Self {
            order,
            height,
            margin,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LaneLayout {
    pub data: LaneLayoutData,
    pub offset: f32,
}
impl LaneLayout {
    pub fn new(data: LaneLayoutData, offset: f32) -> Self {
        Self {
            data,
            offset,
        }
    }
    pub fn calc_transform(&self) -> Transform {
        Transform::from_xyz(0.0, self.offset, 0.0)
    }
}
