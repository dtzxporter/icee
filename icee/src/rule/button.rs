use iced::widget::button::Appearance;
use iced::widget::button::StyleSheet;

use iced::theme::Button;

use iced::Border;
use iced::Theme;

use super::parse_color;
use super::Rules;
use crate::parse_background_color;

impl StyleSheet for Rules {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        let default: Appearance = style.active(&Default::default());

        let style = self.any();

        Appearance {
            text_color: style
                .color
                .as_deref()
                .and_then(parse_color)
                .unwrap_or(default.text_color),
            background: style
                .background_color
                .as_deref()
                .and_then(parse_background_color)
                .or(default.background),
            border: Border {
                radius: style.border_radius().unwrap_or(default.border.radius),
                ..default.border
            },
            ..default
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        let default: Appearance = style.pressed(&Default::default());

        if let Some(style) = self.active() {
            Appearance {
                text_color: style
                    .color
                    .as_deref()
                    .and_then(parse_color)
                    .unwrap_or(default.text_color),
                background: style
                    .background_color
                    .as_deref()
                    .and_then(parse_background_color)
                    .or(default.background),
                border: Border {
                    radius: style.border_radius().unwrap_or(default.border.radius),
                    ..default.border
                },
                ..default
            }
        } else {
            default
        }
    }
}

impl From<Rules> for Button {
    fn from(value: Rules) -> Self {
        Self::Custom(Box::new(value))
    }
}
