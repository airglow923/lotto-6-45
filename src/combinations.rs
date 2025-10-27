use crate::math;

use iced::widget::{Column, Row, Text, container};
use iced::{Center, Element, Length};

#[derive(Debug, Clone)]
pub enum MessageCombinations {
    GoBack,
}

#[derive(Debug)]
pub struct Combinations {
    fixed_inputs: [String; 6],
}

impl Combinations {
    pub fn new(ball_inputs: &[String; 6]) -> Self {
        Self {
            fixed_inputs: ball_inputs.clone(),
        }
    }

    pub fn update(&mut self, message: MessageCombinations) {
        match message {
            MessageCombinations::GoBack => (),
        }
    }

    pub fn view(&self) -> Element<'_, MessageCombinations> {
        let combinations = self.visualize_combinations();

        let element: Element<'_, MessageCombinations> = container(combinations)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        element
    }

    fn get_fixed_balls(&self) -> Vec<i32> {
        let mut res = Vec::new();

        for input in self.fixed_inputs.iter() {
            if let Ok(ball) = input.parse::<i32>() {
                res.push(ball);
            }
        }

        res
    }

    fn get_combinations(&self) -> Vec<Vec<i32>> {
        let fixed_balls = self.get_fixed_balls();
        let combinations = math::combinations(&(1..45).collect::<Vec<_>>(), Some(&fixed_balls), 7);

        combinations.unwrap()
    }

    fn visualize_ball<'a>(
        content: &'a str,
        style: container::Style,
    ) -> Element<'a, MessageCombinations> {
        container(content).style(move |_| style).into()
    }

    fn visualize_balls(&self, balls: Vec<i32>) -> Element<'_, MessageCombinations> {
        let mut rows = Vec::with_capacity(balls.len());

        for ball in balls {
            rows.push(Text::new(ball).into());
        }

        Row::from_vec(rows).spacing(10).align_y(Center).into()
    }

    fn visualize_combinations(&self) -> Element<'_, MessageCombinations> {
        let combinations = self.get_combinations();
        let mut cols = Vec::with_capacity(combinations.len());

        for combination in combinations {
            cols.push(self.visualize_balls(combination));
        }

        Column::from_vec(cols).into()
    }
}
