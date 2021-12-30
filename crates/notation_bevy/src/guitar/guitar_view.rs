use float_eq::float_ne;
use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use notation_bevy_utils::prelude::{
    BevyUtil, LayoutAnchor, LayoutChangedQuery, LayoutSize, ShapeOp, View, ViewBundle,
};
use notation_midi::prelude::MidiState;
use notation_model::prelude::{
    Duration, Entry, HandShape6, Interval, LaneEntry, LaneKind, ModelEntryProps, Pick, Syllable,
    Tab, TrackKind, Units,
};

use crate::prelude::{EntryPlaying, NotationAssets, NotationSettings, NotationTheme, TabState};
use crate::ui::layout::NotationLayout;

use super::fret_finger::FretFingerData;
use super::guitar_barre::GuitarBarreData;
use super::guitar_capo::GuitarCapoData;
use super::guitar_string::GuitarStringData;

#[derive(Clone, Debug)]
pub struct GuitarView {
    pub tab: Arc<Tab>,
}
impl GuitarView {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}
impl Display for GuitarView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<GuitarView>({})", self.tab.bars.len())
    }
}

impl<'a> View<NotationLayout<'a>> for GuitarView {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::TOP
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
            ViewBundle::from(GuitarView::new(tab.clone())),
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
        let fretboard = tab
            .get_track_of_kind(TrackKind::Guitar)
            .and_then(|x| x.get_fretboard6());

        BevyUtil::spawn_child_bundle(commands, guitar_entity, sprite_bundle);
        for string in 1..=6 {
            for upper in [true, false] {
                let string_data = GuitarStringData::new(string as u8, upper, fretboard);
                string_data.create(commands, theme, guitar_entity);
            }
        }
        let capo_data = GuitarCapoData::default();
        capo_data.create(commands, theme, guitar_entity);
        let barre_data = GuitarBarreData::default();
        barre_data.create(commands, theme, guitar_entity);
        if Self::CHECKING_FRETS {
            let mut string = 1;
            let mut fret = 0;
            for _index in 0..=22 {
                let mut finger_data = FretFingerData::new_data(
                    ModelEntryProps {
                        index: 0,
                        tied_units: Units(0.0),
                    },
                    Syllable::Do,
                    Interval::Unison,
                    string as u8,
                    false,
                    Some(fret as u8),
                    None,
                );
                finger_data.value.extra.visible = true;
                finger_data.spawn(commands, theme, guitar_entity);
                string = string + 1;
                if string > 6 {
                    string = 1;
                }
                fret = fret + 1;
            }
        } else {
            for index in 1..=6 {
                for pick in [true, false] {
                    let finger_data = FretFingerData::new_data(
                        ModelEntryProps {
                            index: 0,
                            tied_units: Units(0.0),
                        },
                        Syllable::Do,
                        Interval::Unison,
                        index as u8,
                        pick,
                        None,
                        None,
                    );
                    finger_data.spawn(commands, theme, guitar_entity);
                }
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
        mut barre_query: Query<(&Parent, Entity, &mut GuitarBarreData), With<GuitarBarreData>>,
        mut finger_query: Query<(&Parent, Entity, &mut FretFingerData), With<FretFingerData>>,
    ) {
        if theme._bypass_systems {
            return;
        }
        for (entity, _view, layout) in query.iter() {
            let guitar_height =
                layout.size.width * theme.guitar.image_size.1 / theme.guitar.image_size.0;
            let guitar_size = LayoutSize::new(layout.size.width, guitar_height);
            for (parent, mut transform) in sprite_query.iter_mut() {
                if parent.0 == entity {
                    let scale = layout.size.width / theme.guitar.image_size.0;
                    transform.translation = Vec3::new(0.0, 0.0, theme.z.guitar_view);
                    transform.scale = Vec3::new(scale, scale, 1.0);
                }
            }
            for (parent, string_entity, mut string_data) in string_query.iter_mut() {
                if parent.0 == entity {
                    string_data.guitar_size = guitar_size;
                    string_data.update(&mut commands, &theme, string_entity);
                }
            }
            for (parent, finger_entity, mut finger_data) in finger_query.iter_mut() {
                if parent.0 == entity {
                    finger_data.value.extra.guitar_size = guitar_size;
                    finger_data.update(&mut commands, &theme, finger_entity);
                }
            }
            for (parent, capo_entity, mut capo_data) in capo_query.iter_mut() {
                if parent.0 == entity {
                    capo_data.view_size = layout.size;
                    capo_data.guitar_size = guitar_size;
                    capo_data.update(&mut commands, &theme, capo_entity);
                }
            }
            for (parent, barre_entity, mut barre_data) in barre_query.iter_mut() {
                if parent.0 == entity {
                    barre_data.view_size = layout.size;
                    barre_data.guitar_size = guitar_size;
                    barre_data.update(&mut commands, &theme, barre_entity);
                }
            }
        }
    }
    pub fn update_string_state(
        mut commands: Commands,
        assets: Res<NotationAssets>,
        settings: Res<NotationSettings>,
        midi_state: Res<MidiState>,
        time: Res<Time>,
        theme: Res<NotationTheme>,
        query: Query<(&Arc<LaneEntry>, &Pick, &EntryPlaying), Changed<EntryPlaying>>,
        mut string_query: Query<(Entity, &mut GuitarStringData), With<GuitarStringData>>,
        mut finger_query: Query<(Entity, &mut FretFingerData), With<FretFingerData>>,
        mut barre_query: Query<(Entity, &mut GuitarBarreData), With<GuitarBarreData>>,
        dot_query: Query<&Children>,
    ) {
        if Self::CHECKING_FRETS {
            return;
        }
        if theme._bypass_systems {
            return;
        }
        let mut current_entry_pick = None;
        let mut string_states = [None; 6];
        let mut hit_strings = [(false, Duration::Zero); 6];
        for (entry, pick, playing) in query.iter() {
            if playing.value.is_current() {
                current_entry_pick = Some((entry, pick));
            }
            for pick_note in pick.get_notes() {
                if pick_note.string >= 1 && pick_note.string <= 6 {
                    string_states[(pick_note.string - 1) as usize] = Some(playing.value);
                    hit_strings[(pick_note.string - 1) as usize] =
                        (playing.value.is_current(), entry.duration());
                }
            }
        }
        let fretboard = current_entry_pick
            .and_then(|(entry, _)| entry.track().and_then(|x| x.get_fretboard6()));
        let meta = current_entry_pick.and_then(|(entry, _)| entry.bar().map(|x| x.tab_meta()));

        for (string_entity, mut string_data) in string_query.iter_mut() {
            if string_data.string >= 1 && string_data.string <= 6 {
                let (hit, hit_duration) = hit_strings[(string_data.string - 1) as usize];
                string_data.set_hit(
                    hit,
                    hit_duration,
                    &time,
                    theme.guitar.hit_string_seconds_range,
                    midi_state.play_control.play_speed,
                );
                if let Some(state) = string_states[(string_data.string - 1) as usize] {
                    string_data.state = state;
                }
                if let Some((_, pick)) = current_entry_pick {
                    string_data.update_pick(fretboard, *pick, meta.clone());
                }
                string_data.update(&mut commands, &theme, string_entity);
            }
        }
        if let Some((entry, pick)) = current_entry_pick {
            let chord = entry.bar().and_then(|x| x.get_chord_of_entry(&entry));
            for (finger_entity, mut finger_data) in finger_query.iter_mut() {
                let changed = finger_data.update_pick(fretboard, chord, *pick, meta.clone());
                if changed {
                    if finger_data.value.extra.pick {
                        finger_data.respawn_dots(
                            &mut commands,
                            &theme,
                            Some(&dot_query),
                            finger_entity,
                        );
                    }
                    finger_data.update_with_syllable(
                        &mut commands,
                        &assets,
                        &theme,
                        &settings,
                        finger_entity,
                    );
                }
            }
            for (_barre_entity, mut barre_data) in barre_query.iter_mut() {
                barre_data.update_pick(pick);
            }
        }
    }
    pub fn update_hand_shape6(
        mut commands: Commands,
        assets: Res<NotationAssets>,
        theme: Res<NotationTheme>,
        settings: Res<NotationSettings>,
        query: Query<(&Arc<LaneEntry>, &HandShape6, &EntryPlaying), Changed<EntryPlaying>>,
        mut finger_query: Query<(Entity, &mut FretFingerData), With<FretFingerData>>,
        mut string_query: Query<(Entity, &mut GuitarStringData), With<GuitarStringData>>,
        mut capo_query: Query<(Entity, &mut GuitarCapoData), With<GuitarCapoData>>,
        mut barre_query: Query<(Entity, &mut GuitarBarreData), With<GuitarBarreData>>,
        dot_query: Query<&Children>,
        tab_state_query: Query<(Entity, &TabState), With<TabState>>,
    ) {
        if Self::CHECKING_FRETS {
            return;
        }
        if theme._bypass_systems {
            return;
        }
        let mut current_shape = None;
        for (entry, shape, playing) in query.iter() {
            if playing.value.is_current() {
                //println!("GuitarView::update_hand_shape6(): found changed playing shape: {}", shape);
                current_shape = Some((entry, shape));
            }
        }
        if let Some((entry, shape)) = current_shape {
            let fretboard = entry.track().and_then(|x| x.get_fretboard6());
            let chord = entry.bar().and_then(|x| x.get_chord_of_entry(&entry));
            let pick = entry.bar().and_then(|x| {
                x.get_entry_in_other_lane(
                    LaneKind::Strings,
                    entry.track_index(),
                    Some(entry.in_bar_pos()),
                    &|x: &LaneEntry| {
                        x.proto()
                            .as_fretted6()
                            .and_then(|y| y.as_pick())
                            .map(|z| z.to_owned())
                    },
                )
            });
            let meta = entry.bar().map(|x| x.tab_meta());
            //println!("GuitarView::update_hand_shape6(): {}, {:#?}, {:#?}", shape, fretboard, chord);
            for (finger_entity, mut finger_data) in finger_query.iter_mut() {
                finger_data.update_value(shape, fretboard, chord, pick, meta.clone());
                finger_data.respawn_dots(&mut commands, &theme, Some(&dot_query), finger_entity);
                finger_data.update_with_syllable(
                    &mut commands,
                    &assets,
                    &theme,
                    &settings,
                    finger_entity,
                );
            }
            for (string_entity, mut string_data) in string_query.iter_mut() {
                string_data.update_value(shape, fretboard, pick, meta.clone());
                string_data.update(&mut commands, &theme, string_entity);
            }
            if let Some(fretboard) = fretboard {
                for (capo_entity, mut capo_data) in capo_query.iter_mut() {
                    if fretboard.capo != capo_data.capo {
                        capo_data.capo = fretboard.capo;
                        capo_data.update(&mut commands, &theme, capo_entity);
                    }
                }
                for (barre_entity, mut barre_data) in barre_query.iter_mut() {
                    barre_data.capo = fretboard.capo;
                    barre_data.shape = Some(shape.clone());
                    barre_data.pick = None;
                    barre_data.update(&mut commands, &theme, barre_entity);
                }
            }
        } else {
            let position = TabState::get_position(&tab_state_query, None);
            if position.is_some() && position.unwrap().bar.bar_ordinal == 0 {
                for (finger_entity, mut finger_data) in finger_query.iter_mut() {
                    finger_data.reset();
                    finger_data.update(&mut commands, &theme, finger_entity);
                }
                for (string_entity, mut string_data) in string_query.iter_mut() {
                    string_data.reset();
                    string_data.update(&mut commands, &theme, string_entity);
                }
                for (barre_entity, mut barre_data) in barre_query.iter_mut() {
                    if barre_data.shape.is_some() {
                        barre_data.shape = None;
                        barre_data.pick = None;
                        barre_data.update(&mut commands, &theme, barre_entity);
                    }
                }
            }
        }
    }
    pub fn update_y(guitar_view_query: &mut Query<&mut Transform, With<Arc<GuitarView>>>, y: f32) {
        if let Ok(mut transform) = guitar_view_query.single_mut() {
            let trans = transform.translation;
            if float_ne!(trans.y, y, abs <= 0.01) {
                println!("GuitarView::update_y {} -> {}", trans.y, y);
                *transform = Transform::from_xyz(trans.x, y, trans.z);
            }
        }
    }
    pub fn adjust_y_by_frets(
        theme: &NotationTheme,
        guitar_view_query: &mut Query<&mut Transform, With<Arc<GuitarView>>>,
        view_size: LayoutSize,
        guitar_size: LayoutSize,
        min_fret: u8,
        max_fret: u8,
    ) {
        let calc_y = |fret: u8| {
            let fret_y = theme.guitar.calc_fret_y(fret, guitar_size.height);
            -(fret_y + guitar_size.height * theme.guitar.capo_height_factor)
        };
        let top_y = calc_y(min_fret);
        let bottom_y = calc_y(max_fret + 1);
        let y = if bottom_y - top_y > view_size.height {
            bottom_y - view_size.height
        } else {
            top_y
        };
        //println!("GuitarView::adjust_y_by_frets {} {} [{} - {}] -> {} {} -> {}", view_size, guitar_size, min_fret, max_fret, top_y, bottom_y, y);
        Self::update_y(guitar_view_query, y);
    }
    pub fn adjust_y_by_barre(
        theme: Res<NotationTheme>,
        settings: Res<NotationSettings>,
        barre_query: Query<&GuitarBarreData, Changed<GuitarBarreData>>,
        mut guitar_view_query: Query<&mut Transform, With<Arc<GuitarView>>>,
    ) {
        if Self::CHECKING_FRETS {
            return;
        }
        if theme._bypass_systems {
            return;
        }
        if settings.override_guitar_y.is_some() {
            return;
        }
        for barre_data in barre_query.iter() {
            if barre_data.view_size.height > barre_data.guitar_size.height {
                Self::update_y(&mut guitar_view_query, -barre_data.guitar_size.height / 2.0);
            } else {
                let min_fret = barre_data.capo;
                let max_fret = barre_data.max_fret();
                Self::adjust_y_by_frets(
                    &theme,
                    &mut guitar_view_query,
                    barre_data.view_size,
                    barre_data.guitar_size,
                    min_fret,
                    max_fret,
                );
            }
        }
    }
}
