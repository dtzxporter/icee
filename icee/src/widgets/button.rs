use super::IceeWidget;

use crate::StyleSheet;
use crate::WithStyleSheet;

impl<'a, Message, Theme, Renderer> WithStyleSheet<'a>
    for iced::widget::Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'static,
    Theme: iced::widget::button::StyleSheet + 'static,
    <Theme as iced::widget::button::StyleSheet>::Style: From<crate::Rules>,
    Renderer: iced::advanced::text::Renderer + 'static,
{
    type Output = IceeWidget<'a, Message, Theme, Renderer>;

    fn with_stylesheet_id(self, stylesheet: &'a StyleSheet, id: &'static str) -> Self::Output {
        let rules = stylesheet.rules("button", Some(id));
        let any = rules.any();

        let mut element = self.style(rules.clone());

        if let Some(width) = any.width() {
            element = element.width(width);
        }

        if let Some(height) = any.height() {
            element = element.height(height);
        }

        IceeWidget {
            element: element.into(),
            rules,
        }
    }

    fn with_stylesheet(self, stylesheet: &'a StyleSheet) -> Self::Output {
        let rules = stylesheet.rules("button", None);

        IceeWidget {
            element: self.into(),
            rules,
        }
    }
}
