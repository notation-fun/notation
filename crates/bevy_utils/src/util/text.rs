use bevy::prelude::*;

use super::BevyUtil;

impl BevyUtil {
    pub fn spawn_text(
        commands: &mut Commands,
        entity: Entity,
        text: &str,
        font: Handle<Font>,
        font_size: f32,
        color: Color,
        horizontal: HorizontalAlign,
        vertical: VerticalAlign,
        x: f32,
        y: f32,
        z: f32,
    ) -> Entity {
        let style = TextStyle {
            font,
            font_size,
            color,
        };
        let alignment = TextAlignment {
            vertical,
            horizontal,
        };
        let text_entity = commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(text, style, alignment),
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            })
            .id();
        commands.entity(entity).push_children(&[text_entity]);
        text_entity
    }
    pub fn set_text_color(text: &mut Text, color: Color) {
        for section in text.sections.iter_mut() {
            section.style.color = color;
        }
    }
}
