use std::{fmt::Display, ops::{Add, Sub}, sync::{Arc, RwLock}};

use bevy::prelude::*;
use bevy_prototype_lyon::shapes;

use crate::prelude::{LayoutAnchor, LayoutHAnchor, LayoutVAnchor};

#[derive(Copy, Clone, Debug, Default)]
pub struct LayoutSize {
    pub width: f32,
    pub height: f32,
}
impl Display for LayoutSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.width, self.height)
    }
}
impl LayoutSize {
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
pub struct LayoutData {
    pub depth: usize,
    pub pivot: LayoutAnchor,
    pub anchor: LayoutAnchor,
    pub offset: Vec2,
    pub size: LayoutSize,
}
impl Display for LayoutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<LayoutData>({} {} {} - {}, {} - {})",
            self.depth, self.pivot, self.anchor, self.offset.x, self.offset.y, self.size)
    }
}

impl LayoutData {
    pub fn new(
        depth: usize,
        pivot: LayoutAnchor,
        anchor: LayoutAnchor,
        offset: Vec2,
        size: LayoutSize,
    ) -> Self {
        Self {
            depth,
            pivot,
            anchor,
            offset,
            size,
        }
    }
    pub fn view_default() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(LayoutData::default()))
    }
    pub fn calc_offset(&self, pivot: LayoutAnchor, anchor: LayoutAnchor, offset: Vec2) -> Vec2 {
        self.size.calc_offset(pivot, anchor) + offset
    }
    pub fn new_child(&self, pivot: LayoutAnchor, anchor: LayoutAnchor, offset: Vec2, size: LayoutSize) -> Self {
        let offset = self.size.calc_offset(pivot, anchor) + offset;
        Self::new(self.depth + 1, anchor, anchor, offset, size)
    }
    pub fn change_pivot(&self, pivot: LayoutAnchor) -> Self {
        let offset = self.calc_offset(self.pivot, pivot, self.offset);
        Self {
            pivot,
            offset,
            ..*self
        }
    }
    pub fn transform(&self) -> Transform {
        Transform::from_xyz(self.offset.x, self.offset.y, 1.0)
    }
}

impl From<LayoutData> for shapes::RectangleOrigin {
    fn from(v: LayoutData) -> Self {
        match (v.pivot.v, v.pivot.h) {
            (LayoutVAnchor::Top, LayoutHAnchor::Left) =>
                Self::TopLeft,
            (LayoutVAnchor::Top, LayoutHAnchor::Center) =>
                Self::CustomCenter(Vec2::new(0.0, -0.5 * v.size.height)),
            (LayoutVAnchor::Top, LayoutHAnchor::Right) =>
                Self::TopRight,
            (LayoutVAnchor::Center, LayoutHAnchor::Left) =>
                Self::CustomCenter(Vec2::new(-0.5 * v.size.height, 0.0)),
            (LayoutVAnchor::Center, LayoutHAnchor::Center) =>
                Self::Center,
            (LayoutVAnchor::Center, LayoutHAnchor::Right) =>
                Self::CustomCenter(Vec2::new(0.5 * v.size.height, 0.0)),
            (LayoutVAnchor::Bottom, LayoutHAnchor::Left) =>
                Self::BottomLeft,
            (LayoutVAnchor::Bottom, LayoutHAnchor::Center) =>
                Self::CustomCenter(Vec2::new(0.0, 0.5 * v.size.height)),
            (LayoutVAnchor::Bottom, LayoutHAnchor::Right) =>
                Self::BottomRight,
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