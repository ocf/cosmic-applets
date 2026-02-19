use cosmic::app::Core;
use cosmic::{Action, Element, Task};

use cosmic::widget::autosize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
    return false;
}

fn set_theme(is_dark: bool) {
    let scheme = if is_dark {
        "prefer-dark"
    } else {
        "prefer-light"
    };

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
        let window = Window { core, is_dark };

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
        let sun_icon =
            cosmic::widget::icon::from_svg_bytes(include_bytes!("./assets/light_mode.svg"));
        let moon_icon =
            cosmic::widget::icon::from_svg_bytes(include_bytes!("./assets/dark_mode.svg"));

        let selected_icon = if self.is_dark { sun_icon } else { moon_icon };

        let button = self
            .core
            .applet
            .icon_button_from_handle(selected_icon)
            .on_press(Message::ToggleTheme)
            .padding([0, self.core.applet.suggested_padding(true).0])
            .class(cosmic::theme::Button::AppletIcon);

        autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }
}
