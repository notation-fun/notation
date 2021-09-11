use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{
    BevyUtil, ColorBackground, DockPanel, DockSide, LayoutAnchor, LayoutChangedQuery,
    LayoutConstraint, LayoutSize, LyonShapeOp, View, ViewBundle,
};
use notation_midi::prelude::MidiState;
use notation_model::prelude::{
    Duration, Entry, HandShape6, Interval, LaneEntry, ModelEntryProps, Pick, Syllable, Tab, Units,
};

use crate::prelude::{EntryPlaying, NotationAssets, NotationTheme};
use crate::ui::layout::NotationLayout;

use super::fret_finger::{FretFinger, FretFingerData};
use super::guitar_capo::{GuitarCapo, GuitarCapoData};
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
    fn dock_side(&self, _engine: &NotationLayout<'a>, _size: LayoutSize) -> DockSide {
        DockSide::Left
    }
}

impl<'a> View<NotationLayout<'a>> for GuitarView {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::CENTER
    }
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let width = constraint.max.height / engine.theme.guitar.image_size.1
            * engine.theme.guitar.image_size.0;
        LayoutSize::new(width, constraint.max.height)
    }
}

impl GuitarView {
    pub const CHECKING_FRETS: bool = false;
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
            sprite: Sprite::new(Vec2::new(
                theme.guitar.image_size.0,
                theme.guitar.image_size.1,
            )),
            transform: BevyUtil::offscreen_transform(),
            material: materials.add(assets.fretboard.clone().into()),
            ..Default::default()
        };
        BevyUtil::spawn_child_bundle(commands, guitar_entity, sprite_bundle);
        for string in 1..=6 {
            for upper in [true, false] {
                GuitarString::create(
                    commands,
                    theme,
                    guitar_entity,
                    GuitarStringData::new(string as u8, upper),
                );
            }
        }
        GuitarCapo::create(commands, theme, guitar_entity, GuitarCapoData::new(0));
        if Self::CHECKING_FRETS {
            let mut string = 1;
            let mut fret = 0;
            for _index in 0..=22 {
                let finger_data = FretFingerData::new_data(
                    ModelEntryProps {
                        index: 0,
                        tied_units: Units(0.0),
                    },
                    Syllable::Do,
                    Interval::Unison,
                    string as u8,
                    Some(fret as u8),
                    None,
                );
                FretFinger::spawn(commands, theme, guitar_entity, finger_data);
                string = string + 1;
                if string > 6 {
                    string = 1;
                }
                fret = fret + 1;
            }
        } else {
            for index in 1..=6 {
                let finger_data = FretFingerData::new_data(
                    ModelEntryProps {
                        index: 0,
                        tied_units: Units(0.0),
                    },
                    Syllable::Do,
                    Interval::Unison,
                    index as u8,
                    None,
                    None,
                );
                FretFinger::spawn(commands, theme, guitar_entity, finger_data);
            }
        }
        guitar_entity
    }
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedQuery<GuitarView>,
        mut sprite_query: Query<(&Parent, &mut Transform), With<Sprite>>,
        mut string_query: Query<(&Parent, Entity, &mut GuitarStringData), With<GuitarStringData>>,
        mut capo_query: Query<(&Parent, Entity, &mut GuitarCapoData), With<GuitarCapoData>>,
        mut finger_query: Query<(&Parent, Entity, &mut FretFingerData), With<FretFingerData>>,
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
            for (parent, capo_entity, mut capo_data) in capo_query.iter_mut() {
                if parent.0 == entity {
                    capo_data.guitar_size = layout.size;
                    GuitarCapo::update(&mut commands, &theme, capo_entity, &capo_data);
                }
            }
            for (parent, finger_entity, mut finger_data) in finger_query.iter_mut() {
                if parent.0 == entity {
                    finger_data.value.extra.guitar_size = layout.size;
                    FretFinger::update(&mut commands, &theme, finger_entity, &finger_data);
                }
            }
        }
    }
    pub fn update_string_state(
        mut commands: Commands,
        midi_state: Res<MidiState>,
        time: Res<Time>,
        theme: Res<NotationTheme>,
        query: Query<(&Arc<LaneEntry>, &Pick, &EntryPlaying), Changed<EntryPlaying>>,
        mut string_query: Query<(Entity, &mut GuitarStringData), With<GuitarStringData>>,
    ) {
        let mut current_notes = Vec::new();
        let mut string_states = [None; 6];
        let mut hit_strings = [(false, Duration::Zero); 6];
        for (entry, pick, playing) in query.iter() {
            for pick_note in pick.get_notes() {
                if playing.value.is_current() {
                    current_notes.push(pick_note);
                }
                if pick_note.string >= 1 && pick_note.string <= 6 {
                    string_states[(pick_note.string - 1) as usize] = Some(playing.value);
                    hit_strings[(pick_note.string - 1) as usize] =
                        (playing.value.is_current(), entry.duration());
                }
            }
        }
        for (string_entity, mut string_data) in string_query.iter_mut() {
            if string_data.string >= 1 && string_data.string <= 6 {
                let (hit, hit_duration) = hit_strings[(string_data.string - 1) as usize];
                string_data.set_hit(
                    hit,
                    hit_duration,
                    &time,
                    theme.strings.hit_string_seconds_range,
                    midi_state.play_control.play_speed,
                );
                if let Some(state) = string_states[(string_data.string - 1) as usize] {
                    string_data.state = state;
                }
                GuitarString::update(&mut commands, &theme, string_entity, &string_data);
            }
        }
    }
    pub fn update_hand_shape6(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: Query<(&Arc<LaneEntry>, &HandShape6, &EntryPlaying), Changed<EntryPlaying>>,
        dot_query: Query<&Children>,
        mut finger_query: Query<(Entity, &mut FretFingerData), With<FretFingerData>>,
        mut string_query: Query<(Entity, &mut GuitarStringData), With<GuitarStringData>>,
        mut capo_query: Query<(Entity, &mut GuitarCapoData), With<GuitarCapoData>>,
    ) {
        if Self::CHECKING_FRETS {
            return;
        }
        let mut current_shape = None;
        for (entry, shape, playing) in query.iter() {
            if playing.value.is_current() {
                current_shape = Some((entry, shape));
            }
        }
        if let Some((entry, shape)) = current_shape {
            let fretboard = entry.track().and_then(|x| x.get_fretboard6());
            let chord = entry.bar().and_then(|x| x.get_chord_of_entry(&entry));
            let meta = entry.bar().map(|x| x.tab_meta());
            //println!("GuitarView::update_hand_shape6(): {}, {:#?}, {:#?}", shape, fretboard, chord);
            for (finger_entity, mut finger_data) in finger_query.iter_mut() {
                finger_data.update(shape, fretboard, chord, meta.clone());
                FretFinger::respawn_dots(
                    &mut commands,
                    &theme,
                    Some(&dot_query),
                    finger_entity,
                    &finger_data,
                );
                FretFinger::update(&mut commands, &theme, finger_entity, &finger_data);
            }
            for (string_entity, mut string_data) in string_query.iter_mut() {
                string_data.update(shape, fretboard);
                GuitarString::update(&mut commands, &theme, string_entity, &string_data);
            }
            if let Some(fretboard) = fretboard {
                for (capo_entity, mut capo_data) in capo_query.iter_mut() {
                    if fretboard.capo != capo_data.capo {
                        capo_data.capo = fretboard.capo;
                        GuitarCapo::update(&mut commands, &theme, capo_entity, &capo_data);
                    }
                }
            }
        }
    }
}
