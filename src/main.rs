use iced::{Size, Task, window};

use std::default::Default;

use app::App;

mod app;
mod ball_input;
mod color;
mod combinations;
mod drawing_system;
mod lotto;
mod math;

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            position: window::Position::Centered,
            ..Default::default()
        })
        .run_with(|| (App::default(), Task::none()))
}
