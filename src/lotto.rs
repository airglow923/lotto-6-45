use crate::drawing_system;

use iced::widget::{Column, Row, Text, button, container};
use iced::{Center, Color, Element, Length, widget};

use std::default::Default;

use drawing_system::DrawingSystem;

#[derive(Debug)]
pub struct Lotto {
    n_balls: usize,
    n_draws: usize,
    n_simulations: usize,
    drawing_systems: Vec<DrawingSystem>,
    simulations: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Generate,
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
            n_balls,
            n_draws,
            n_simulations,
            drawing_systems,
            simulations,
        }
    }

    fn renew(&mut self) {
        for i in 0..self.n_simulations {
            self.drawing_systems[i] = DrawingSystem::new(self.n_balls);
            self.drawing_systems[i].shuffle();
        }

        for i in 0..self.n_simulations {
            for j in 0..self.n_draws {
                self.simulations[i][j] = self.drawing_systems[i].draw();
            }

            self.simulations[i].sort();
        }
    }

    pub fn title(&self) -> String {
        String::from("Lotto 6/45 Simulation")
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Generate => self.renew(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let cols = widget::column![
            self.visualize_simulations(),
            button("Generate").on_press(Message::Generate)
        ]
        .spacing(10)
        .align_x(Center);

        let element: Element<'_, Message> = container(cols)
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

    fn visualize_drawn_balls(&self, no_simulation: usize) -> Element<'_, Message> {
        let mut rows = Vec::with_capacity(self.n_simulations);

        if self.drawing_systems.is_empty() {
            rows.resize_with(self.n_draws, || Text::new(0).into());
        } else {
            for i in 0..self.n_draws {
                rows.push(Text::new(self.simulations[no_simulation][i]).into());
            }
        }

        Row::from_vec(rows).spacing(10).align_y(Center).into()
    }

    fn visualize_simulations(&self) -> Element<'_, Message> {
        let mut cols = Vec::with_capacity(self.n_simulations);

        for i in 0..self.n_simulations {
            cols.push(self.visualize_drawn_balls(i));
        }

        Column::from_vec(cols).into()
    }
}

impl Default for Lotto {
    fn default() -> Self {
        Lotto::new(45, 7, 5)
    }
}
