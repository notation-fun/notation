use std::fmt::Display;

use bevy::prelude::*;
use bevy_utils::prelude::{BevyUtil, GridCell, LayoutAnchor, LayoutChangedWithChildrenQuery, ShapeOp, FillPath, View, ViewBundle};

use crate::prelude::{NotationAssets, NotationTheme};
use crate::ui::layout::NotationLayout;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PlayButtonAction {
    PlayPause,
    Stop,
    Settings,
    SetBegin,
    SetEnd,
    LoopMode,
}
impl Display for PlayButtonAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl PlayButtonAction {
    pub fn order(&self) -> usize {
        match self {
            PlayButtonAction::PlayPause => 0,
            PlayButtonAction::Stop => 1,
            PlayButtonAction::Settings => 2,
            PlayButtonAction::SetBegin => 3,
            PlayButtonAction::SetEnd => 4,
            PlayButtonAction::LoopMode => 5,
        }
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(1792.0, 1792.0)
    }
    pub fn path(&self, playing: bool) -> String {
        // https://github.com/Rush/Font-Awesome-SVG-PNG/tree/master/black/svg
        match self {
            PlayButtonAction::PlayPause =>
                if playing {
                    "M832 1184v-576q0-14-9-23t-23-9h-256q-14 0-23 9t-9 23v576q0 14 9 23t23 9h256q14 0 23-9t9-23zm448 0v-576q0-14-9-23t-23-9h-256q-14 0-23 9t-9 23v576q0 14 9 23t23 9h256q14 0 23-9t9-23zm384-288q0 209-103 385.5t-279.5 279.5-385.5 103-385.5-103-279.5-279.5-103-385.5 103-385.5 279.5-279.5 385.5-103 385.5 103 279.5 279.5 103 385.5z"
                } else {
                    "M1312 896q0 37-32 55l-544 320q-15 9-32 9-16 0-32-8-32-19-32-56v-640q0-37 32-56 33-18 64 1l544 320q32 18 32 55zm128 0q0-148-73-273t-198-198-273-73-273 73-198 198-73 273 73 273 198 198 273 73 273-73 198-198 73-273zm224 0q0 209-103 385.5t-279.5 279.5-385.5 103-385.5-103-279.5-279.5-103-385.5 103-385.5 279.5-279.5 385.5-103 385.5 103 279.5 279.5 103 385.5z"
                }
            PlayButtonAction::Stop =>
                "M1216 1184v-576q0-14-9-23t-23-9h-576q-14 0-23 9t-9 23v576q0 14 9 23t23 9h576q14 0 23-9t9-23zm448-288q0 209-103 385.5t-279.5 279.5-385.5 103-385.5-103-279.5-279.5-103-385.5 103-385.5 279.5-279.5 385.5-103 385.5 103 279.5 279.5 103 385.5z",
            PlayButtonAction::Settings =>
                "M1152 896q0-106-75-181t-181-75-181 75-75 181 75 181 181 75 181-75 75-181zm512-109v222q0 12-8 23t-20 13l-185 28q-19 54-39 91 35 50 107 138 10 12 10 25t-9 23q-27 37-99 108t-94 71q-12 0-26-9l-138-108q-44 23-91 38-16 136-29 186-7 28-36 28h-222q-14 0-24.5-8.5t-11.5-21.5l-28-184q-49-16-90-37l-141 107q-10 9-25 9-14 0-25-11-126-114-165-168-7-10-7-23 0-12 8-23 15-21 51-66.5t54-70.5q-27-50-41-99l-183-27q-13-2-21-12.5t-8-23.5v-222q0-12 8-23t19-13l186-28q14-46 39-92-40-57-107-138-10-12-10-24 0-10 9-23 26-36 98.5-107.5t94.5-71.5q13 0 26 10l138 107q44-23 91-38 16-136 29-186 7-28 36-28h222q14 0 24.5 8.5t11.5 21.5l28 184q49 16 90 37l142-107q9-9 24-9 13 0 25 10 129 119 165 170 7 8 7 22 0 12-8 23-15 21-51 66.5t-54 70.5q26 50 41 98l183 28q13 2 21 12.5t8 23.5z",
            PlayButtonAction::SetBegin =>
                "M1203 544q0 13-10 23l-393 393 393 393q10 10 10 23t-10 23l-50 50q-10 10-23 10t-23-10l-466-466q-10-10-10-23t10-23l466-466q10-10 23-10t23 10l50 50q10 10 10 23z",
            PlayButtonAction::SetEnd =>
                "M1171 960q0 13-10 23l-466 466q-10 10-23 10t-23-10l-50-50q-10-10-10-23t10-23l393-393-393-393q-10-10-10-23t10-23l50-50q10-10 23-10t23 10l466 466q10 10 10 23z",
            PlayButtonAction::LoopMode =>
                "M1664 256v448q0 26-19 45t-45 19h-448q-42 0-59-40-17-39 14-69l138-138q-148-137-349-137-104 0-198.5 40.5t-163.5 109.5-109.5 163.5-40.5 198.5 40.5 198.5 109.5 163.5 163.5 109.5 198.5 40.5q119 0 225-52t179-147q7-10 23-12 15 0 25 9l137 138q9 8 9.5 20.5t-7.5 22.5q-109 132-264 204.5t-327 72.5q-156 0-298-61t-245-164-164-245-61-298 61-298 164-245 245-164 298-61q147 0 284.5 55.5t244.5 156.5l130-129q29-31 70-14 39 17 39 59z",
        }.to_owned()
    }
}

impl From<usize> for PlayButtonAction {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::PlayPause,
            1 => Self::Stop,
            2 => Self::Settings,
            3 => Self::SetBegin,
            4 => Self::SetEnd,
            _ => Self::LoopMode,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayButton  {
    pub action: PlayButtonAction,
}
impl Display for PlayButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct PlayButtonShape {
    pub action: PlayButtonAction,
    pub width: f32,
    pub height: f32,
}
impl Display for PlayButtonShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ShapeOp<NotationTheme, FillPath> for PlayButtonShape {
    fn get_shape(&self, theme: &NotationTheme) -> FillPath {
        let scale = self.height / 1792.0;
        FillPath {
            size: self.action.size(),
            path: self.action.path(true),
            color: theme.shapes.shape_color,
            //line_width: theme.shapes.shape_line_width,
            offset: Vec3::new(0.0, 0.0, theme.core.mini_map_z),
            scale: scale,
            angle: 0.0,
        }
    }
}

impl<'a> View<NotationLayout<'a>> for PlayButton {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::CENTER
    }
}
impl<'a> GridCell<NotationLayout<'a>> for PlayButton {
    fn order(&self) -> usize {
        self.action.order()
    }
}

impl PlayButton {
    pub fn spawn(
        commands: &mut Commands,
        _assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        action: PlayButtonAction,
    ) -> Entity {
        let button_entity = BevyUtil::spawn_child_bundle(
            commands,
            entity,
            ViewBundle::from(PlayButton{action}),
        );
        let button_shape = PlayButtonShape {
            action,
            width: 32.0,
            height: 32.0,
        };
        button_shape.create(commands, theme, button_entity);
        button_entity
    }
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedWithChildrenQuery<PlayButton>,
        mut shape_query: Query<(Entity, &mut PlayButtonShape)>,
    ) {
        for (_entity, _view, layout, children) in query.iter() {
            for child in children.iter() {
                if let Ok((entity, mut data)) = shape_query.get_mut(*child) {
                    data.width = layout.size.width;
                    data.height = layout.size.height;
                    data.update(&mut commands, &theme, entity);
                }
            }
        }
    }
}