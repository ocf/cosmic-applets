use cosmic::app::Core;
use cosmic::{Action, Element, Task};

use cosmic::iced::{Alignment, Length};
use cosmic::iced::widget::{row, vertical_space};
use cosmic::widget::{autosize, container, button};
use std::time::Duration;

const ID: &str = "io.ocf.paper-genmon-applet";

#[derive(Default)]
pub struct Window {
    core: Core,
    panel_text: String, // Field stores paper genmon output
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick,                // Changed by timer
    UpdateText(String),  // Called with result of paper genmon command
}

impl cosmic::Application for Window {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        let window = Window {
            core,
            panel_text: String::from("Loading page count..."), // Initial text
            ..Default::default()
        };

        // Run the command immediately on startup
        (window, Task::done(Message::Tick).map(Action::from))
    }

    // Background timer for refreshing every 5 seconds
    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        cosmic::iced::time::every(Duration::from_secs(5))
            .map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<Action<Self::Message>> {
        match message {
            Message::Tick => {
                return Task::perform(
                    async {
                        let output = std::process::Command::new("paper-genmon")
                            .output()
                            .ok();

                        if let Some(out) = output {
                            String::from_utf8_lossy(&out.stdout).trim().to_string()
                        } else {
                            "Error".to_string()
                        }
                    },
                    Message::UpdateText,
                )
                .map(Action::from);
            }
            Message::UpdateText(new_text) => {
                self.panel_text = new_text;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = Element::from(
            row!(
                self.core.applet.text(&self.panel_text),
                container(vertical_space().height(Length::Fixed(
                    (self.core.applet.suggested_size(true).1
                        + 2 * self.core.applet.suggested_padding(true).1)
                        as f32
                )))
            ).align_y(Alignment::Center),
        );

        let button = button::custom(content)
            .padding([0, self.core.applet.suggested_padding(true).0])
            .class(cosmic::theme::Button::AppletIcon);

        autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }
}
