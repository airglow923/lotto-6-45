use crate::combinations;
use crate::lotto;

use iced::Element;
use iced::widget::Text;

use combinations::Combinations;
use combinations::MessageCombinations;
use lotto::Lotto;
use lotto::MessageLotto;

#[derive(Debug, Clone)]
pub enum Message {
    Lotto(MessageLotto),
    Combinations(MessageCombinations),
}

#[derive(Debug)]
enum View {
    Lotto(Lotto),
    Combinations(Combinations),
}

pub struct App {
    active: View,
}

impl App {
    pub fn new() -> Self {
        Self {
            active: View::Lotto(Lotto::default()),
        }
    }

    pub fn title(&self) -> String {
        String::from("Lotto 6/45 Simulation")
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.active {
            View::Lotto(view) => view.view().map(Message::Lotto),
            View::Combinations(view) => view.view().map(Message::Combinations),
        }
    }

    pub fn update(&mut self, message: Message) {
        match &mut self.active {
            View::Lotto(view) => {
                if let Message::Lotto(msg) = message {
                    match msg {
                        MessageLotto::Generate => {
                            self.active =
                                View::Combinations(Combinations::new(&view.get_ball_inputs()));
                        }
                        _ => {
                            view.update(msg);
                        }
                    }
                }
            }
            View::Combinations(_view) => if let Message::Combinations(_msg) = message {},
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
