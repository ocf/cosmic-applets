use cosmic::app::Core;
use cosmic::{Action, Element, Task};

use cosmic::iced::{Alignment, Length};
use cosmic::iced::widget::{row, vertical_space};
use cosmic::widget::{autosize, container, button};
use std::process::Command;
use std::fs;
use std::path::PathBuf;

const ID: &str = "io.ocf.theme-applet";

pub struct Window {
    core: Core,
    is_dark: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    ToggleTheme,
}

fn get_config_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| {
        let mut path = PathBuf::from(home);
        path.push(".config/cosmic/com.system76.CosmicTheme.Mode/v1/is_dark");
        path
    })
}

fn read_is_dark() -> bool {
    if let Some(path) = get_config_path() {
        if let Ok(content) = fs::read_to_string(path) {
            return content.trim() == "true";
        }
    }
    return false
}

fn set_theme(is_dark: bool) {
    let scheme = if is_dark { "prefer-dark" } else { "prefer-light" };

    // Set gsettings
    let _ = Command::new("gsettings")
        .args(["set", "org.gnome.desktop.interface", "color-scheme", scheme])
        .output();

    // Write to cosmic config
    if let Some(path) = get_config_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(path, if is_dark { "true" } else { "false" });
    }
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
        let is_dark = read_is_dark();
        let window = Window {
            core,
            is_dark,
        };

        (window, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Action<Self::Message>> {
        match message {
            Message::ToggleTheme => {
                self.is_dark = !self.is_dark;
                set_theme(self.is_dark);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let label = if self.is_dark { "Dark Mode" } else { "Light Mode" };
        
        let content = Element::from(
            row!(
                self.core.applet.text(label),
                container(vertical_space().height(Length::Fixed(
                    (self.core.applet.suggested_size(true).1
                        + 2 * self.core.applet.suggested_padding(true).1)
                        as f32
                )))
            ).align_y(Alignment::Center),
        );

        let button = button::custom(content)
            .on_press(Message::ToggleTheme)
            .padding([0, self.core.applet.suggested_padding(true).0])
            .class(cosmic::theme::Button::AppletIcon);

        autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }
}
