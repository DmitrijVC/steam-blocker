pub mod style;
pub mod settings;

use crate::steam;
use crate::LOGS;
use crate::sbfe_enable;
use crate::sbfe::sbfe_is_enabled;
use settings::*;
use iced::{button, Element, Length, Container, Sandbox, Row, Align, Scrollable, scrollable, Space, Color};
use iced::{Button, Column, Text};
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Mutex;

lazy_static!{
    static ref LOGS_DATA: Mutex<String> = {
        Mutex::new(String::new())
    };
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Messages {
    Blocker_TogglePressed,
}

struct Blocker {
    is_enabled: bool,
    toggle_button: button::State,
}

impl Blocker {
    pub fn view(&mut self) -> Column<Messages> {

        let (mode, m_color) = if self.is_enabled {
            ("enabled", Color::from_rgb8(11, 186, 72))
        } else {
            ("disabled", Color::from_rgb8(186, 37, 11))
        };

        Column::new()
            .push(
                Row::new()
                    .push(
                        Text::new("Steam Blocker is: ")
                            .color(Color::from_rgb8(235, 233, 232))
                    )

                    .push(
                        Text::new(mode)
                            .color(m_color)
                    )
            )

            .push(
                Space::with_height(Length::Units(5))
            )

            .push(
                Button::new(&mut self.toggle_button, Text::new("Toggle"))
                    .on_press(Messages::Blocker_TogglePressed)
                    .style(style::Button::Primary)
            )

            .push(
                Space::with_height(Length::Units(30))
            )

            .align_items(Align::Center)
    }

    pub fn update(&mut self, message: Messages, app_path_str: &str) {
        match message {
            Messages::Blocker_TogglePressed => {

                let app_path = CString::new(app_path_str).unwrap();

                unsafe {
                    match sbfe_enable(app_path.as_ptr() as *const c_char) {
                        Ok(result) => {
                            match result {
                                0 => {
                                    self.is_enabled = false;
                                    LOGS.lock().unwrap().add("Blocker has been disabled!");
                                }
                                1 => {
                                    self.is_enabled = true;
                                    LOGS.lock().unwrap().add("Blocker has been enabled!");
                                }
                                2 => {
                                    LOGS.lock().unwrap().add("Critical error!");
                                }
                                3 => {
                                    LOGS.lock().unwrap().add("Missing admin privileges!");
                                }
                                4 => {
                                    LOGS.lock().unwrap().add("Unknown defined error!");
                                }
                                _ => {
                                    LOGS.lock().unwrap().add("Unknown unexpected error!");
                                }
                            }
                        }
                        Err(_) => {
                            LOGS.lock().unwrap().add("SBFwEditor.dll missing or damaged!");
                        }
                    }

                    *LOGS_DATA.lock().unwrap() = LOGS.lock().unwrap().to_string();
                }
            }
        }
    }
}

pub(crate) struct App {
    blocker: Blocker,
    scroll_stdout: scrollable::State,
    steam_path: String,
}

impl Sandbox for App {
    type Message = Messages;

    fn new() -> Self { unsafe {
        let mut logs = LOGS.lock().unwrap();
        logs.add("App initialization...");

        let enabled = match sbfe_is_enabled() {
            Ok(result) => {
                logs.add("Retrieved the Blocker status successfully!");
                result
            },
            Err(_) => {
                logs.add("SBFwEditor.dll missing or damaged!");
                logs.add("Restart the software to avoid potential issues!");
                false
            },
        };

        let steam = match steam::find_path() {
            Ok(path) => {
                logs.add("Retrieved the Steam path successfully!");
                path
            },
            Err(_) => {
                logs.add("Steam installation path not found!");
                logs.add("Please check this registry key:");
                logs.add(&*format!("[HKLM\\{}\\InstallPath]", steam::REG_PATH));
                String::new()
            }
        };

        logs.add("Done!");
        *LOGS_DATA.lock().unwrap() = logs.to_string();

        Self {
            blocker: Blocker {
                is_enabled: enabled,
                toggle_button: Default::default(),
            },
            scroll_stdout: Default::default(),
            steam_path: steam,
        }
    } }

    fn title(&self) -> String {
        String::from(WINDOW_TITLE)
    }

    fn update(&mut self, event: Self::Message) {
        match event {
            Messages::Blocker_TogglePressed => {
                if !self.steam_path.is_empty() {
                    self.blocker.update(event, self.steam_path.as_str());
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let App {
            ..
        } = self;

        let mut controls = Column::new();
        controls = controls.push(
            Row::new()
                .push(
                    self.blocker.view()
                )
        );

        controls = controls.push(
            Row::new()
                .push(
                    Container::new(
                        Scrollable::new(&mut self.scroll_stdout)
                            .width(Length::Units(CONSOLE_WIDTH))
                            .height(Length::Units(CONSOLE_HEIGHT))
                            .style(style::ScrollableStdout)
                            .push(
                                Text::new(format!("{}", LOGS_DATA.lock().unwrap()))
                                    .size(15)
                                    // .font(console_get_font())
                            )
                    )
                        .style(style::ContainerStdout)
                        .align_x(Align::Center)
                        .align_y(Align::Center)
                )
        );

        controls = controls.width(Length::Fill).align_items(Align::Center);

        Container::new(controls)
            .style(style::Container)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_y()
            .center_x()
            .into()
    }

    fn scale_factor(&self) -> f64 {
        GUI_SCALE
    }
}
