pub const WINDOW_SIZE: (u32, u32) = (460, 270);
pub const WINDOW_TITLE: &'static str = "Steam Blocker";
pub const WINDOW_NOCONSOLE: bool = true;

pub const CONSOLE_WIDTH: u16 = 430;
pub const CONSOLE_HEIGHT: u16 = 150;

pub const GUI_SCALE: f64 = 1.0;


// pub fn console_get_font() -> iced::Font {
//     use std::fs::File;
//     use std::io::Read;
//
//     let def_font = iced::Font::Default;
//
//     let mut buff = Vec::<u8>::new();
//     let mut file = match File::open("bin/fonts/CONSOLAS.TTF") {
//         Ok(file) => file,
//         Err(_) => return def_font,
//     };
//
//     if let Err(result) = file.read(&mut buff){
//         return def_font
//     };
//
//     iced::Font::External {
//         name: "consolas",
//         bytes: buff.as_slice()
//     }
// }
