use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingComponent, EasingType};
use bevy_utils::prelude::{GridData, LayoutData};
use float_eq::float_ne;
use std::sync::Arc;

use notation_model::prelude::{LaneKind, Position};
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::bar::bar_layout::BarLayoutData;
use crate::lane::lane_layout::LaneLayoutData;
use crate::play::pos_indicator::PosIndicatorData;
use crate::prelude::{NotationTheme, TabBars};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub enum LayoutMode {
    Grid,
    Line,
}
impl Default for LayoutMode {
    fn default() -> Self {
        Self::Grid
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LayoutSettings {
    pub mode: LayoutMode,
    pub focus_bar_ease_ms: u64,
    pub shapes_lane_order: usize,
    pub strings_lane_order: usize,
    pub lyrics_lane_order: usize,
    pub melody_lane_order: usize,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            mode: LayoutMode::default(),
            focus_bar_ease_ms: 250,
            shapes_lane_order: 1,
            strings_lane_order: 2,
            lyrics_lane_order: 3,
            melody_lane_order: 4,
        }
    }
}

impl LayoutSettings {
    pub fn calc_lane_order(&self, lane_layout: &LaneLayoutData) -> (usize, usize) {
        let (track_index, lane_kind) = lane_layout.order();
        (track_index, match lane_kind {
            LaneKind::Lyrics => self.lyrics_lane_order,
            LaneKind::Melody => self.melody_lane_order,
            LaneKind::Strings => self.strings_lane_order,
            LaneKind::Shapes => self.shapes_lane_order,
            _ => 0,
        })
    }
    pub fn sort_lane_layouts(&self, lanes: &Vec<LaneLayoutData>) -> Vec<LaneLayoutData> {
        let mut sorted: Vec<LaneLayoutData> = lanes.clone();
        sorted.sort_by(|a, b| self.calc_lane_order(a).cmp(&self.calc_lane_order(b)));
        sorted
    }
    pub fn bar_layout_of_pos(
        &self,
        //bar_layouts: &Arc<Vec<BarLayoutData>>,
        _pos: Position,
    ) -> Option<BarLayoutData> {
        //bar_layouts.get(pos.bar.bar_ordinal - 1).map(|x| x.clone())
        None
    }
    pub fn pan_tab_bars(
        &self,
        tab_bars_query: &mut Query<(Entity, &mut Transform, &Arc<TabBars>)>,
        delta_x: f32,
        delta_y: f32,
    ) {
        if let Ok((_, mut camera_transform, _)) = tab_bars_query.single_mut() {
            let trans = camera_transform.translation;
            let (x, y) = match self.mode {
                LayoutMode::Grid => (trans.x, trans.y + delta_y),
                LayoutMode::Line => (trans.x - delta_x, trans.y),
            };
            *camera_transform = Transform::from_xyz(x, y, trans.z);
        }
    }
    pub fn set_transform_xy(
        &self,
        transform: &mut Transform,
        x: Option<f32>,
        y: Option<f32>,
    ) {
        let trans = transform.translation;
        *transform =
            Transform::from_xyz(
                x.unwrap_or(trans.x),
                y.unwrap_or(trans.y),
                trans.z
            );
    }
    pub fn ease_transform_xy(
        &self,
        commands: &mut Commands,
        entity: Entity,
        transform: &mut Transform,
        x: Option<f32>,
        y: Option<f32>,
    ) {
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove::<EasingComponent<Transform>>();
        let from = transform.translation;
        let to = Vec3::new(
            x.unwrap_or(from.x),
            y.unwrap_or(from.y),
            from.z
        );
        if float_ne!(from.x, to.x, abs <= 0.01) || float_ne!(from.y, to.y, abs <= 0.01) {
            println!(
                "layout_settings.ease_transform_xy(): {}, {} -> {}, {}",
                from.x, from.y, to.x, to.y
            );
            let ease_function = EaseFunction::CubicOut;
            entity_commands.insert(transform.ease_to(
                Transform::from_translation(to),
                ease_function,
                EasingType::Once {
                    duration: std::time::Duration::from_millis(self.focus_bar_ease_ms),
                },
            ));
        }
    }
    fn calc_grid_focus_y(&self,
        theme: &NotationTheme,
        _bars: &TabBars,
        _layout: &LayoutData,
        grid_data: &GridData,
        pos_data: &PosIndicatorData,
    ) -> f32 {
        let (row, col) = grid_data.calc_row_col(pos_data.bar_position.bar_ordinal - 1);
        let y = if row == 0 {
            pos_data.bar_layout.offset.y
        } else {
            grid_data.calc_cell_offset(row - 1, col).y
        };
        y + theme.grid.margin + theme.grid.header_height
    }
    fn calc_line_focus_x(&self,
        _theme: &NotationTheme,
        _bars: &TabBars,
        layout: &LayoutData,
        grid_data: &GridData,
        pos_data: &PosIndicatorData,
    ) -> f32 {
        if pos_data.bar_position.bar_ordinal == 1 {
            - grid_data.offset.x - layout.offset.x
        } else {
            let size = grid_data.calc_cell_size(0, pos_data.bar_position.bar_ordinal - 1);
            pos_data.offset_x() - size.width - layout.offset.x - grid_data.offset.x
        }
        /*
        Units(if bar_layout.data.col > 0 {
            bar_layout.data.col as f32 - 1.0 + pos.bar.in_bar_pos.0 / pos.bar.bar_units.0
        } else {
            bar_layout.data.col as f32
        })
         */
    }
    pub fn focus_bar(
        &self,
        commands: &mut Commands,
        theme: &NotationTheme,
        tab_bars_query: &mut Query<(Entity, &mut Transform, &Arc<TabBars>, &LayoutData, &Arc<GridData>)>,
        pos_data: &PosIndicatorData,
    ) {
        if let Ok((bars_entity, mut bars_transform, bars, layout, grid_data)) = tab_bars_query.single_mut() {
            match self.mode {
                LayoutMode::Grid => {
                    let y = self.calc_grid_focus_y(theme, bars, layout, grid_data, pos_data);
                    self.ease_transform_xy(commands, bars_entity, &mut bars_transform, None, Some(-y));
                }
                LayoutMode::Line => {
                    let x = self.calc_line_focus_x(theme, bars, layout, grid_data, pos_data);
                    self.set_transform_xy(&mut bars_transform, Some(-x), None);
                }
            }
        } else {
            println!("layout_settings.focus_bar() Query Failed");
        }
    }
}
