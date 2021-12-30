use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use notation_bevy_utils::prelude::{
    BevyUtil, FillPath, GridCell, LayoutAnchor, LayoutChangedWithChildrenQuery, ShapeOp, View,
    ViewBundle,
};
use notation_model::prelude::{PlayState, Tab};

use crate::prelude::{NotationAssets, NotationSettings, NotationTheme};
use crate::ui::layout::NotationLayout;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PlayButtonAction {
    PlayPause,
    Stop,
    LoopMode,
    SetBegin,
    SetEnd,
    Clear,
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
            PlayButtonAction::LoopMode => 2,
            PlayButtonAction::SetBegin => 3,
            PlayButtonAction::SetEnd => 4,
            PlayButtonAction::Clear => 5,
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
                    "M896 128q209 0 385.5 103t279.5 279.5 103 385.5-103 385.5-279.5 279.5-385.5 103-385.5-103-279.5-279.5-103-385.5 103-385.5 279.5-279.5 385.5-103zm384 823q32-18 32-55t-32-55l-544-320q-31-19-64-1-32 19-32 56v640q0 37 32 56 16 8 32 8 17 0 32-9z"
                }
            PlayButtonAction::Stop =>
                "M1216 1184v-576q0-14-9-23t-23-9h-576q-14 0-23 9t-9 23v576q0 14 9 23t23 9h576q14 0 23-9t9-23zm448-288q0 209-103 385.5t-279.5 279.5-385.5 103-385.5-103-279.5-279.5-103-385.5 103-385.5 279.5-279.5 385.5-103 385.5 103 279.5 279.5 103 385.5z",
            PlayButtonAction::LoopMode =>
                "M1664 256v448q0 26-19 45t-45 19h-448q-42 0-59-40-17-39 14-69l138-138q-148-137-349-137-104 0-198.5 40.5t-163.5 109.5-109.5 163.5-40.5 198.5 40.5 198.5 109.5 163.5 163.5 109.5 198.5 40.5q119 0 225-52t179-147q7-10 23-12 15 0 25 9l137 138q9 8 9.5 20.5t-7.5 22.5q-109 132-264 204.5t-327 72.5q-156 0-298-61t-245-164-164-245-61-298 61-298 164-245 245-164 298-61q147 0 284.5 55.5t244.5 156.5l130-129q29-31 70-14 39 17 39 59z",
            PlayButtonAction::SetBegin =>
                "M1203 544q0 13-10 23l-393 393 393 393q10 10 10 23t-10 23l-50 50q-10 10-23 10t-23-10l-466-466q-10-10-10-23t10-23l466-466q10-10 23-10t23 10l50 50q10 10 10 23z",
            PlayButtonAction::SetEnd =>
                "M1171 960q0 13-10 23l-466 466q-10 10-23 10t-23-10l-50-50q-10-10-10-23t10-23l393-393-393-393q-10-10-10-23t10-23l50-50q10-10 23-10t23 10l466 466q10 10 10 23z",
            PlayButtonAction::Clear =>
                "M1792 896q0 26-19 45l-256 256q-19 19-45 19t-45-19-19-45v-128h-1024v128q0 26-19 45t-45 19-45-19l-256-256q-19-19-19-45t19-45l256-256q19-19 45-19t45 19 19 45v128h1024v-128q0-26 19-45t45-19 45 19l256 256q19 19 19 45z",
        }.to_owned()
    }
}

impl From<usize> for PlayButtonAction {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::PlayPause,
            1 => Self::Stop,
            2 => Self::LoopMode,
            3 => Self::SetBegin,
            4 => Self::SetEnd,
            _ => Self::Clear,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayButton {
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
    pub play_state: PlayState,
    pub should_loop: bool,
    pub bars: usize,
    pub begin_bar_ordinal: usize,
    pub end_bar_ordinal: usize,
}
impl Display for PlayButtonShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl PlayButtonShape {
    pub fn get_color(&self, theme: &NotationTheme) -> Color {
        match self.action {
            PlayButtonAction::PlayPause | PlayButtonAction::Stop => theme.colors.ui.button_on,
            PlayButtonAction::LoopMode => theme.colors.ui.of_button(self.should_loop),
            PlayButtonAction::SetBegin | PlayButtonAction::SetEnd | PlayButtonAction::Clear => {
                theme
                    .colors
                    .ui
                    .of_button(self.begin_bar_ordinal != 0 || self.end_bar_ordinal + 1 != self.bars)
            }
        }
    }
}

impl ShapeOp<NotationTheme, FillPath> for PlayButtonShape {
    fn get_shape(&self, theme: &NotationTheme) -> FillPath {
        let scale = self.height / 1792.0 * theme.sizes.tab_control.button_scale_factor;
        FillPath {
            size: self.action.size(),
            path: self.action.path(self.play_state.is_playing()),
            color: self.get_color(theme),
            //line_width: theme.shapes.shape_line_width,
            offset: Vec3::new(0.0, 0.0, theme.z.play_button),
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
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
        action: PlayButtonAction,
    ) -> Entity {
        let button_entity =
            BevyUtil::spawn_child_bundle(commands, entity, ViewBundle::from(PlayButton { action }));
        let button_shape = PlayButtonShape {
            action,
            width: 32.0,
            height: 32.0,
            play_state: PlayState::Stopped,
            should_loop: settings.should_loop,
            bars: tab.bars.len(),
            begin_bar_ordinal: 0,
            end_bar_ordinal: if tab.bars.len() > 0 {
                tab.bars.len() - 1
            } else {
                0
            },
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
        if theme._bypass_systems {
            return;
        }
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
    pub fn on_play_state(
        commands: &mut Commands,
        theme: &NotationTheme,
        shape_query: &mut Query<(Entity, &mut PlayButtonShape)>,
        play_state: &PlayState,
    ) {
        for (entity, mut shape) in shape_query.iter_mut() {
            shape.play_state = play_state.clone();
            if shape.action == PlayButtonAction::PlayPause {
                shape.update(commands, theme, entity);
            }
        }
    }
    pub fn on_should_loop(
        commands: &mut Commands,
        theme: &NotationTheme,
        shape_query: &mut Query<(Entity, &mut PlayButtonShape)>,
        should_loop: bool,
    ) {
        for (entity, mut shape) in shape_query.iter_mut() {
            shape.should_loop = should_loop;
            if shape.action == PlayButtonAction::LoopMode {
                shape.update(commands, theme, entity);
            }
        }
    }
    pub fn on_begin_end(
        commands: &mut Commands,
        theme: &NotationTheme,
        shape_query: &mut Query<(Entity, &mut PlayButtonShape)>,
        begin_bar_ordinal: usize,
        end_bar_ordinal: usize,
    ) {
        for (entity, mut shape) in shape_query.iter_mut() {
            shape.begin_bar_ordinal = begin_bar_ordinal;
            shape.end_bar_ordinal = end_bar_ordinal;
            if shape.action == PlayButtonAction::SetBegin
                || shape.action == PlayButtonAction::SetEnd
                || shape.action == PlayButtonAction::Clear
            {
                shape.update(commands, theme, entity);
            }
        }
    }
}
