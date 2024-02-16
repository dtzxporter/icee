use iced::widget::container::Appearance;
use iced::widget::container::StyleSheet;

use iced::theme::Container;

use iced::Border;
use iced::Theme;

use super::Rules;
use crate::parse_background_color;

impl StyleSheet for Rules {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let default: Appearance = style.appearance(&Default::default());

        let style = self.any();

        Appearance {
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
}

impl From<Rules> for Container {
    fn from(value: Rules) -> Self {
        Self::Custom(Box::new(value))
    }
}