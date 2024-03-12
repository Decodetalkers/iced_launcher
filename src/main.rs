//use iced::widget::{button, column, row, text, text_input};
use iced::widget::text_input;
use iced::{Command, Element, Theme};
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::Application;
//use iced_runtime::command::Action;

pub fn main() -> Result<(), iced_layershell::Error> {
    Launcher::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((1000, 1000)),
            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right | Anchor::Top,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Launcher {
    text: String,
}

#[derive(Debug, Clone)]
enum Message {
    SearchEditChanged(String),
    SearchSubmit,
}

impl Application for Launcher {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                text: "".to_string(),
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("iced_launcer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SearchSubmit => {}
            Message::SearchEditChanged(edit) => {
                self.text = edit;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        text_input("put the launcher name", &self.text)
            .padding(10)
            .on_input(Message::SearchEditChanged)
            .on_submit(Message::SearchSubmit)
            .into()
    }
}
