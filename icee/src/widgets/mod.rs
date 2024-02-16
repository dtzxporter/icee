mod button;

use iced::{advanced::Widget, Element};

use crate::Rules;

pub struct IceeWidget<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer,
{
    element: Element<'a, Message, Theme, Renderer>,
    rules: Rules,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for IceeWidget<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer,
{
    fn size(&self) -> iced::Size<iced::Length> {
        let default = self.element.as_widget().size();
        let any = self.rules.any();

        iced::Size::new(
            any.width().unwrap_or(default.width),
            any.height().unwrap_or(default.height),
        )
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.element
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        vec![iced::advanced::widget::Tree::new(&self.element)]
    }

    fn on_event(
        &mut self,
        state: &mut iced::advanced::widget::Tree,
        event: iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        self.element.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.element.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        self.element.as_widget().tag()
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        self.element.as_widget().diff(&mut tree.children[0])
    }

    fn mouse_interaction(
        &self,
        state: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.element.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &self,
        state: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation<Message>,
    ) {
        self.element
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation)
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.element
            .as_widget_mut()
            .overlay(&mut state.children[0], layout, renderer, translation)
    }

    fn size_hint(&self) -> iced::Size<iced::Length> {
        self.element.as_widget().size_hint()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        self.element.as_widget().state()
    }
}

impl<'a, Message, Theme, Renderer> From<IceeWidget<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'static,
    Theme: iced::widget::button::StyleSheet + 'a,
    <Theme as iced::widget::button::StyleSheet>::Style: From<crate::Rules>,
    Renderer: iced::advanced::text::Renderer + 'a,
{
    fn from(
        widget: IceeWidget<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(widget)
    }
}
