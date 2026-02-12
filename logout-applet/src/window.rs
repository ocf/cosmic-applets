use cosmic::app::Core;
use cosmic::{Action, Element, Task};

use cosmic::widget::{text, button};

const ID: &str = "io.ocf.logout-applet";

#[derive(Default)]
pub struct Window {
    core: Core,
}

#[derive(Clone, Debug)]
pub enum Message {
    Logout,
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
            ..Default::default()
        };

        (window, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Action<Self::Message>> {
        match message {
            Message::Logout => {
                std::process::Command::new("cosmic-osd")
                    .args(["log-out"])
                    .output()
                    .ok();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = text("Log Out")
            .size(18)
            .width(cosmic::iced::Length::Shrink);

        let button = button::custom(content)
            .class(cosmic::theme::Button::AppletIcon)
            .on_press(Message::Logout)
            .padding([0, 12]);

        cosmic::widget::autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }
}
