use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{BevyUtil, ColorBackground, DockPanel, DockSide, LayoutAnchor, LayoutChangedQuery, LayoutConstraint, LayoutSize, LyonShapeOp, View, ViewBundle};
use notation_model::prelude::{LaneEntry, Pick, PlayingState, Syllable, Tab};

use crate::prelude::{EntryPlaying, NotationAssets, NotationTheme};
use crate::strings::pick_note::PickNoteData;
use crate::ui::layout::NotationLayout;

use super::guitar_string::{GuitarString, GuitarStringData};

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
            sprite: Sprite::new(Vec2::new(theme.guitar.image_size.0, theme.guitar.image_size.1)),
            transform: BevyUtil::offscreen_transform(),
            material: materials.add(assets.fretboard.clone().into()),
            ..Default::default()
        };
        BevyUtil::spawn_child_bundle(commands, guitar_entity, sprite_bundle);
        for string in 1..=6 {
            GuitarString::create(
                commands,
                theme,
                guitar_entity,
                GuitarStringData::new(string as u8),
            );
        }
        guitar_entity
    }
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedQuery<GuitarView>,
        mut sprite_query: Query<(&Parent, &mut Transform), With<Sprite>>,
        mut string_query: Query<(&Parent, Entity, &mut GuitarStringData), With<GuitarStringData>>,
    ) {
        for (entity, _view, layout) in query.iter() {
            for (parent, mut transform) in sprite_query.iter_mut() {
                if parent.0 == entity {
                    let scale = layout.size.width / theme.guitar.image_size.0;
                    transform.translation = Vec3::new(0.0, 0.0, theme.core.mini_bar_z);
                    transform.scale = Vec3::new(scale, scale, 1.0);
                }
            }
            for (parent, string_entity, mut string_data) in string_query.iter_mut() {
                if parent.0 == entity {
                    string_data.guitar_size = layout.size;
                    GuitarString::update(&mut commands, &theme, string_entity, &string_data);
                }
            }
        }
    }
    pub fn update_string_state(
        mut commands: Commands,
        time: Res<Time>,
        theme: Res<NotationTheme>,
        query: Query<(&Arc<LaneEntry>, &Pick, &EntryPlaying), Changed<EntryPlaying>>,
        mut string_query: Query<(Entity, &mut GuitarStringData), With<GuitarStringData>>,
    ) {
        let mut current_notes = Vec::new();
        let mut string_states = [None; 6];
        let mut hit_strings = [false; 6];
        for (entry, pick, playing) in query.iter() {
            for pick_note in pick.get_notes() {
                if playing.value.is_current() {
                    current_notes.push(pick_note);
                }
                if pick_note.string >= 1 && pick_note.string <= 6 {
                    string_states[(pick_note.string - 1) as usize] = Some(playing.value);
                    hit_strings[(pick_note.string - 1) as usize] = playing.value.is_current();
                }
            }
        }
        for (string_entity, mut string_data) in string_query.iter_mut() {
            if string_data.string >= 1 && string_data.string <= 6 {
                let hit = hit_strings[(string_data.string - 1) as usize];
                string_data.set_hit(hit, &time, theme.strings.hit_string_seconds);
                if let Some(state) = string_states[(string_data.string - 1) as usize] {
                    string_data.state = state;
                }
                GuitarString::update(&mut commands, &theme, string_entity, &string_data);
            }
        }
    }
}
