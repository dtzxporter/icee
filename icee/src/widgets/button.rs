use iced::Element;

use crate::StyleSheet;
use crate::WithStyleSheet;

impl<'a, Message, Theme, Renderer> WithStyleSheet<'a>
    for iced::widget::Button<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::button::StyleSheet,
    Renderer: iced::advanced::Renderer,
{
    type Output = IceeButton<'a, Message, Theme, Renderer>;

    fn with_stylesheet_id(self, stylesheet: &'a StyleSheet, id: &'static str) -> Self::Output {
        IceeButton {
            widget: self,
            stylesheet,
            id: Some(id),
        }
    }

    fn with_stylesheet(self, stylesheet: &'a StyleSheet) -> Self::Output {
        IceeButton {
            widget: self,
            stylesheet,
            id: None,
        }
    }
}

pub struct IceeButton<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::button::StyleSheet,
    Renderer: iced::advanced::Renderer,
{
    widget: iced::widget::Button<'a, Message, Theme, Renderer>,
    stylesheet: &'a StyleSheet,
    id: Option<&'static str>,
}

impl<'a, Message, Theme, Renderer> From<IceeButton<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'static,
    Theme: iced::widget::button::StyleSheet + 'a,
    <Theme as iced::widget::button::StyleSheet>::Style: From<crate::Rules>,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(
        button: IceeButton<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        let style = button.stylesheet.rules("button", button.id);

        Element::new(
            button
                .widget
                .width(style.any().width())
                .height(style.any().height())
                .clip(false)
                .style(style),
        )
    }
}
