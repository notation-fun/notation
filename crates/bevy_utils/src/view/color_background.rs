use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::{BevyUtil, FillRectangle, LayoutAnchor, LayoutData, ShapeOp, SingleBundle};

#[derive(Clone, Debug)]
pub struct HasColorBackground;

#[derive(Clone, Debug)]
pub struct ColorBackground {
    layout: LayoutData,
    z: f32,
    color: Color,
}
impl Display for ColorBackground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ColorBackground>({:?})", self.color)
    }
}
impl ColorBackground {
    fn new(z: f32, color: Color) -> Self {
        Self {
            z,
            color,
            layout: LayoutData::ZERO,
        }
    }
}

impl ShapeOp<(), shapes::Rectangle, FillRectangle> for ColorBackground {
    fn get_shape(&self, _theme: &()) -> FillRectangle {
        let offset = self.layout.calc_offset(LayoutAnchor::CENTER, Vec2::ZERO);
        FillRectangle {
            width: self.layout.size.width,
            height: self.layout.size.height,
            origin: shapes::RectangleOrigin::Center,
            color: self.color,
            offset: Vec3::new(offset.x, offset.y, self.z),
        }
    }
}

impl ColorBackground {
    pub fn setup(app: &mut AppBuilder) {
        app.add_system(Self::on_layout_changed.system());
    }
    pub fn on_layout_changed(
        mut commands: Commands,
        query: Query<(&LayoutData, &HasColorBackground, &Children), Changed<LayoutData>>,
        mut background_query: Query<(Entity, &mut ColorBackground)>,
    ) {
        for (layout, _, children) in query.iter() {
            for child in children.iter() {
                if let Ok((background_entity, mut background)) = background_query.get_mut(*child) {
                    background.layout = *layout;
                    background.update(&mut commands, &(), background_entity);
                }
            }
        }
    }
    pub fn spawn(commands: &mut Commands, entity: Entity, z: f32, color: Color) -> Entity {
        commands.entity(entity).insert(HasColorBackground);
        BevyUtil::spawn_child_bundle(
            commands,
            entity,
            SingleBundle::<ColorBackground>::from(ColorBackground::new(z, color)),
        )
    }
}
