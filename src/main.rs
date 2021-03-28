#![cfg(target_os = "windows")]
#[macro_use] extern crate lazy_static;

mod sbfe;
mod gui;
mod logs;
mod steam;

use sbfe::*;
use gui::*;
use gui::settings::*;
use logs::*;
use iced::{Settings, Sandbox};
use winapi::um::wincon::FreeConsole;
use std::sync::Mutex;

lazy_static!{
    static ref LOGS: Mutex<Logs> = {
        Mutex::new(Default::default())
    };
}


fn main() -> iced::Result {
    if WINDOW_NOCONSOLE { unsafe {
        FreeConsole();
    } }

    let settings = Settings {
        window: iced::window::Settings {
            size: WINDOW_SIZE,
            min_size: Some(WINDOW_SIZE),
            max_size: Some(WINDOW_SIZE),
            resizable: false,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None
        },
        flags: (),
        default_font: None,
        default_text_size: 20,
        antialiasing: true
    };

    App::run(settings)
}
