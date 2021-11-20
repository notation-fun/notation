use std::fmt::Display;
use std::ops::{Add, Sub};

use bevy::prelude::*;
use bevy_prototype_lyon::shapes;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::prelude::{LayoutAnchor, LayoutHAnchor, LayoutVAnchor};

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LayoutSize {
    pub width: f32,
    pub height: f32,
}
impl Display for LayoutSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
impl LayoutSize {
    pub const ZERO: Self = Self {
        width: 0.0,
        height: 0.0,
    };
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub fn calc_offset(&self, pivot: LayoutAnchor, anchor: LayoutAnchor) -> Vec2 {
        let factor = anchor.factor() - pivot.factor();
        Vec2::new(self.width * factor.x, self.height * factor.y)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LayoutConstraint {
    pub max: LayoutSize,
}
impl Display for LayoutConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<LayoutConstraint>(max: {})", self.max)
    }
}
impl LayoutConstraint {
    pub fn new(max: LayoutSize) -> Self {
        Self { max }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LayoutData {
    pub depth: usize,
    pub size: LayoutSize,
    pub pivot: LayoutAnchor,
    pub anchor: LayoutAnchor,
    pub offset: Vec2,
}
impl Display for LayoutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<LayoutData>({} {} {} {} {})",
            self.depth, self.size, self.pivot, self.anchor, self.offset
        )
    }
}

impl LayoutData {
    pub const ZERO: Self = Self {
        depth: 0,
        size: LayoutSize::ZERO,
        pivot: LayoutAnchor::CENTER,
        anchor: LayoutAnchor::CENTER,
        offset: Vec2::ZERO,
    };
    pub fn new(
        depth: usize,
        size: LayoutSize,
        pivot: LayoutAnchor,
        anchor: LayoutAnchor,
        offset: Vec2,
    ) -> Self {
        Self {
            depth,
            size,
            pivot,
            anchor,
            offset,
        }
    }
    pub fn _calc_offset(
        size: LayoutSize,
        pivot: LayoutAnchor,
        anchor: LayoutAnchor,
        offset: Vec2,
    ) -> Vec2 {
        size.calc_offset(pivot, anchor) + offset
    }
    pub fn calc_offset(&self, anchor: LayoutAnchor, offset: Vec2) -> Vec2 {
        Self::_calc_offset(self.size, self.pivot, anchor, offset)
    }
    pub fn new_child(&self, anchor: LayoutAnchor, offset: Vec2, child_size: LayoutSize) -> Self {
        let child_offset = self.calc_offset(anchor, offset);
        Self::new(self.depth + 1, child_size, anchor, anchor, child_offset)
    }
    pub fn change_pivot(&self, pivot: LayoutAnchor) -> Self {
        let offset = self.calc_offset(pivot, self.offset);
        Self {
            pivot,
            offset,
            ..*self
        }
    }
    pub fn transform(&self) -> Transform {
        Transform::from_xyz(self.offset.x, self.offset.y, 1.0)
    }
    pub fn is_inside(&self, offset: Vec2) -> bool {
        let top_left = self.calc_offset(LayoutAnchor::TOP_LEFT, Vec2::ZERO);
        let bottom_right = self.calc_offset(LayoutAnchor::BOTTOM_RIGHT, Vec2::ZERO);
        offset.x >= top_left.x
            && offset.x <= bottom_right.x
            && offset.y <= top_left.y
            && offset.y >= bottom_right.y
    }
    pub fn is_pos_inside(&self, pos: Vec2, global_transform: &GlobalTransform) -> bool {
        let offset = pos
            - Vec2::new(
                global_transform.translation.x,
                global_transform.translation.y,
            );
        self.is_inside(offset)
    }
}

impl From<LayoutData> for shapes::RectangleOrigin {
    fn from(v: LayoutData) -> Self {
        match (v.pivot.v, v.pivot.h) {
            (LayoutVAnchor::Top, LayoutHAnchor::Left) => Self::TopLeft,
            (LayoutVAnchor::Top, LayoutHAnchor::Center) => {
                Self::CustomCenter(Vec2::new(0.0, -0.5 * v.size.height))
            }
            (LayoutVAnchor::Top, LayoutHAnchor::Right) => Self::TopRight,
            (LayoutVAnchor::Center, LayoutHAnchor::Left) => {
                Self::CustomCenter(Vec2::new(-0.5 * v.size.height, 0.0))
            }
            (LayoutVAnchor::Center, LayoutHAnchor::Center) => Self::Center,
            (LayoutVAnchor::Center, LayoutHAnchor::Right) => {
                Self::CustomCenter(Vec2::new(0.5 * v.size.height, 0.0))
            }
            (LayoutVAnchor::Bottom, LayoutHAnchor::Left) => Self::BottomLeft,
            (LayoutVAnchor::Bottom, LayoutHAnchor::Center) => {
                Self::CustomCenter(Vec2::new(0.0, 0.5 * v.size.height))
            }
            (LayoutVAnchor::Bottom, LayoutHAnchor::Right) => Self::BottomRight,
        }
    }
}

impl From<(f32, f32)> for LayoutSize {
    fn from(v: (f32, f32)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<LayoutSize> for LayoutConstraint {
    fn from(v: LayoutSize) -> Self {
        Self::new(v)
    }
}

impl From<(f32, f32)> for LayoutConstraint {
    fn from(v: (f32, f32)) -> Self {
        LayoutSize::from(v).into()
    }
}

impl From<LayoutData> for LayoutConstraint {
    fn from(v: LayoutData) -> Self {
        Self::new(v.size)
    }
}

impl Add for LayoutSize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl Sub for LayoutSize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}
