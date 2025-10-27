use iced::{Element, widget};

use iced::widget::text_input;

#[derive(Debug, Default)]
pub struct BallInput {
    input: String,
}

#[derive(Debug, Clone)]
enum Message {
    Changed(String),
}

impl BallInput {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Changed(content) => {
                self.input = content;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        widget::container(text_input("", &self.input).on_input(Message::Changed)).into()
    }

    pub fn get_ball(&self) -> i32 {
        self.input.parse::<i32>().unwrap_or(0)
    }
}
