use iced::widget::{button, column, text};
use iced::{Alignment, Application, Element, Settings};

use icee::command::load_stylesheet;
use icee::subscription::hot_reload;

use icee::StyleSheet;
use icee::WithStyleSheet;

#[derive(Debug, Clone)]
enum Message {
    OnReload(StyleSheet),
    IncrementPressed,
    DecrementPressed,
}

struct MyApp {
    value: i32,
    stylesheet: StyleSheet,
}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = Self {
            value: 0,
            stylesheet: Default::default(),
        };

        // Load the stylesheet once from disk.
        let load = load_stylesheet("../demo/style/main.ron", Message::OnReload);

        (app, load)
    }

    fn title(&self) -> String {
        String::from("icee demo!")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::OnReload(stylesheet) => {
                self.stylesheet = stylesheet;
            }
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }

        iced::Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // Loads the stylesheet every time it changes from disk.
        hot_reload("../demo/style/main.ron", Message::OnReload)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            button("Increment")
                .on_press(Message::IncrementPressed)
                .with_stylesheet_id(&self.stylesheet, "increment"),
            text(self.value).size(50),
            button("Decrement")
                .on_press(Message::DecrementPressed)
                .with_stylesheet_id(&self.stylesheet, "decrement")
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

fn main() {
    MyApp::run(Settings::default()).unwrap()
}
