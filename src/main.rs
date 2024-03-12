use std::process::exit;

use applications::{all_apps, App};
use iced::widget::{column, scrollable, text_input};
use iced::{event, Command, Element, Event, Length, Theme};
mod applications;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::Application;

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
    apps: Vec<App>,
}

#[derive(Debug, Clone)]
enum Message {
    SearchEditChanged(String),
    SearchSubmit,
    Launch(usize),
    IcedEvent(Event),
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
                apps: all_apps(),
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("iced_launcer")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SearchSubmit => Command::none(),
            Message::SearchEditChanged(edit) => {
                self.text = edit;
                Command::none()
            }
            Message::Launch(index) => {
                self.apps[index].launch();
                std::thread::sleep(std::time::Duration::from_millis(1));
                exit(0)
                // FIXME: Command::single(Action::Window(WindowAction::Close(Id::MAIN))),
                // this will cause coredump
            }
            Message::IcedEvent(event) => {
                println!("{event:?}");
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let re = regex::Regex::new(&self.text).ok();
        let text_ip: Element<Message> = text_input("put the launcher name", &self.text)
            .padding(10)
            .on_input(Message::SearchEditChanged)
            .on_submit(Message::SearchSubmit)
            .into();
        let buttom_vec: Vec<Element<Message>> = self
            .apps
            .iter()
            .enumerate()
            .filter(|(_, app)| {
                if re.is_none() {
                    return true;
                }
                let re = re.as_ref().unwrap();

                re.is_match(app.title().to_lowercase().as_str())
                    || re.is_match(app.description().to_lowercase().as_str())
            })
            .map(|(index, app)| app.view(index))
            .collect();
        let buttom: Element<Message> = scrollable(column(buttom_vec).width(Length::Fill)).into();
        column![text_ip, buttom].into()
    }
}
