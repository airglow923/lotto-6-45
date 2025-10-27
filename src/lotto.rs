use crate::drawing_system;

use iced::widget::text_input;
use iced::widget::{button, container};
use iced::{Center, Color, Element, Length, widget};

use std::default::Default;

use drawing_system::DrawingSystem;

#[derive(Debug, Clone)]
pub enum MessageLotto {
    Generate,
    ChangedBallFirst(String),
    ChangedBallSecond(String),
    ChangedBallThird(String),
    ChangedBallFourth(String),
    ChangedBallFifth(String),
    ChangedBallSixth(String),
}

#[derive(Debug)]
pub struct Lotto {
    ball_inputs: [String; 6],
}

impl Lotto {
    pub fn new(n_balls: usize, n_draws: usize, n_simulations: usize) -> Self {
        let mut drawing_systems = Vec::with_capacity(n_simulations);
        let mut simulations = Vec::with_capacity(n_simulations);

        drawing_systems.resize_with(n_simulations, || DrawingSystem::new(n_balls));

        simulations.resize_with(n_simulations, || {
            let mut simulation = Vec::with_capacity(n_draws);
            simulation.resize_with(n_draws, || 0);
            simulation
        });

        Self {
            ball_inputs: std::array::from_fn(|_| String::new()),
        }
    }

    pub fn update(&mut self, message: MessageLotto) {
        match message {
            MessageLotto::Generate => (),
            MessageLotto::ChangedBallFirst(content) => self.ball_inputs[0] = content,
            MessageLotto::ChangedBallSecond(content) => self.ball_inputs[1] = content,
            MessageLotto::ChangedBallThird(content) => self.ball_inputs[2] = content,
            MessageLotto::ChangedBallFourth(content) => self.ball_inputs[3] = content,
            MessageLotto::ChangedBallFifth(content) => self.ball_inputs[4] = content,
            MessageLotto::ChangedBallSixth(content) => self.ball_inputs[5] = content,
        }
    }

    pub fn view(&self) -> Element<'_, MessageLotto> {
        let rows = widget::row![
            text_input("", &self.ball_inputs[0]).on_input(MessageLotto::ChangedBallFirst),
            text_input("", &self.ball_inputs[1]).on_input(MessageLotto::ChangedBallSecond),
            text_input("", &self.ball_inputs[2]).on_input(MessageLotto::ChangedBallThird),
            text_input("", &self.ball_inputs[3]).on_input(MessageLotto::ChangedBallFourth),
            text_input("", &self.ball_inputs[4]).on_input(MessageLotto::ChangedBallFifth),
            text_input("", &self.ball_inputs[5]).on_input(MessageLotto::ChangedBallSixth),
        ];

        let cols = widget::column![rows, button("Generate").on_press(MessageLotto::Generate),]
            .spacing(10)
            .align_x(Center);

        let element: Element<'_, MessageLotto> = container(cols)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        #[cfg(debug_assertions)]
        {
            return element.explain(Color::BLACK);
        }

        #[allow(unreachable_code)]
        return element;
    }

    pub fn get_ball_inputs(&self) -> &[String; 6] {
        &self.ball_inputs
    }
}

impl Default for Lotto {
    fn default() -> Self {
        Lotto::new(45, 7, 5)
    }
}
