use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{
    BevyUtil, ColorBackground, DockPanel, DockSide, LayoutAnchor, LayoutChangedQuery,
    LayoutConstraint, LayoutSize, View, ViewBundle,
};
use notation_model::prelude::{Syllable, Tab};

use crate::prelude::{NotationAssets, NotationTheme};
use crate::ui::layout::NotationLayout;

#[derive(Clone, Debug)]
pub struct GuitarView {
    pub tab: Arc<Tab>,
    pub syllable: Syllable,
}
impl GuitarView {
    pub fn new(tab: Arc<Tab>, syllable: Syllable) -> Self {
        Self { tab, syllable }
    }
}
impl Display for GuitarView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<GuitarView>({})", self.tab.bars.len())
    }
}

impl<'a> DockPanel<NotationLayout<'a>> for GuitarView {
    fn dock_side(&self) -> DockSide {
        DockSide::Left
    }
}

impl<'a> View<NotationLayout<'a>> for GuitarView {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::CENTER
    }
    fn calc_size(&self, _engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let width = constraint.max.height / 877.0 * 100.0;
        LayoutSize::new(width, constraint.max.height)
    }
}
impl GuitarView {
    pub fn spawn(
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let guitar_entity = BevyUtil::spawn_child_bundle(
            commands,
            entity,
            ViewBundle::from(GuitarView::new(tab.clone(), Syllable::default())),
        );
        ColorBackground::spawn(
            commands,
            guitar_entity,
            theme.core.mini_map_z,
            theme.core.background_color,
        );
        let sprite_bundle = SpriteBundle {
            sprite: Sprite::new(Vec2::new(100.0, 877.0)),
            transform: BevyUtil::offscreen_transform(),
            material: materials.add(assets.fretboard.clone().into()),
            ..Default::default()
        };
        BevyUtil::spawn_child_bundle(commands, guitar_entity, sprite_bundle);
        guitar_entity
    }
    pub fn on_layout_changed(
        // mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedQuery<GuitarView>,
        mut sprite_query: Query<(&Parent, &mut Transform), With<Sprite>>,
    ) {
        for (entity, _view, layout) in query.iter() {
            for (parent, mut transform) in sprite_query.iter_mut() {
                if parent.0 == entity {
                    let scale = layout.size.width / 100.0;
                    transform.translation = Vec3::new(0.0, 0.0, theme.core.mini_bar_z);
                    transform.scale = Vec3::new(scale, scale, 1.0);
                }
            }
        }
    }
}
