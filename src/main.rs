mod drawing_system;
mod lotto;

use iced::{Size, Task, window};

use std::default::Default;

use lotto::Lotto;

pub fn main() -> iced::Result {
    iced::application(Lotto::title, Lotto::update, Lotto::view)
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            position: window::Position::Centered,
            ..Default::default()
        })
        .run_with(|| (Lotto::default(), Task::none()))
}
