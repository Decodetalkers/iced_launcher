use std::process::exit;

use applications::{all_apps, App};
use iced::widget::{column, scrollable, text_input};
use iced::{event, Command, Element, Event, Length, Theme};
mod applications;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity};
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::Application;

use once_cell::sync::Lazy;

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

pub fn main() -> Result<(), iced_layershell::Error> {
    Launcher::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((1000, 1000)),
            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right | Anchor::Top,
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Launcher {
    text: String,
    apps: Vec<App>,
    scrollpos: usize,
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
                scrollpos: 0,
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
        use iced_runtime::keyboard;
        use keyboard::key::Named;
        match message {
            Message::SearchSubmit => {
                self.apps[self.scrollpos].launch();
                std::thread::sleep(std::time::Duration::from_millis(1));
                exit(0)
            }
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
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
                    let singal_offset = 1. / self.apps.len() as f32;
                    match key {
                        keyboard::Key::Named(Named::ArrowUp) => {
                            if self.scrollpos == 0 {
                                return Command::none();
                            }
                            self.scrollpos -= 1;
                        }
                        keyboard::Key::Named(Named::ArrowDown) => {
                            if self.scrollpos >= self.apps.len() {
                                return Command::none();
                            }
                            self.scrollpos += 1;
                        }
                        keyboard::Key::Named(Named::Escape) => {
                            exit(0);
                        }
                        _ => {}
                    }
                    return scrollable::snap_to(
                        SCROLLABLE_ID.clone(),
                        scrollable::RelativeOffset {
                            x: 0.,
                            y: singal_offset * self.scrollpos as f32,
                        },
                    );
                }
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
            .map(|(index, app)| app.view(index, index == self.scrollpos))
            .collect();
        let buttom: Element<Message> = scrollable(column(buttom_vec).width(Length::Fill))
            .id(SCROLLABLE_ID.clone())
            .into();
        column![text_ip, buttom].into()
    }
}
