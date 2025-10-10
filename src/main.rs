use iced::widget::{Column, Row, Text, button, container};
use iced::{Center, Element, Length, Size, Task, window};

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::default::Default;
use std::vec::Vec;

pub fn main() -> iced::Result {
    iced::application(Lotto::title, Lotto::update, Lotto::view)
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            position: window::Position::Centered,
            ..Default::default()
        })
        .run_with(|| (Lotto::new(45, 7, 5), Task::none()))
}

#[derive(Debug)]
struct Lotto {
    n_balls: usize,
    n_draws: usize,
    n_simulations: usize,
    drawing_systems: Vec<DrawingSystem>,
    simulations: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Generate,
}

impl Lotto {
    fn new(n_balls: usize, n_draws: usize, n_simulations: usize) -> Self {
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

    fn title(&self) -> String {
        String::from("Lotto 6/45 Simulation")
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::Generate => self.renew(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut cols = Vec::with_capacity(self.n_simulations);

        for simulation in self.visualize_simulations() {
            cols.push(simulation);
        }

        let to_be_shown = container(
            Column::from_vec(cols)
                .push(button("Generate").on_press(Message::Generate))
                .align_x(Center),
        );

        to_be_shown.center(Length::Fill).into()
    }

    fn visualize_drawn_balls(&self, no_simulation: usize) -> Element<'_, Message> {
        let mut row = Vec::with_capacity(self.n_simulations);

        if self.drawing_systems.is_empty() {
            row.resize_with(self.n_draws, || Text::new(0).into());
        } else {
            for i in 0..self.n_draws {
                row.push(Text::new(self.simulations[no_simulation][i]).into());
            }
        }

        Row::from_vec(row).spacing(20).into()
    }

    fn visualize_simulations(&self) -> Vec<Element<'_, Message>> {
        let mut rows = Vec::with_capacity(self.n_simulations);

        for i in 0..self.n_simulations {
            rows.push(self.visualize_drawn_balls(i));
        }

        rows
    }
}

impl Default for Lotto {
    fn default() -> Self {
        Lotto::new(45, 7, 5)
    }
}

#[derive(Debug)]
struct DrawingSystem {
    rng: ThreadRng,
    balls: Vec<usize>,
}

impl DrawingSystem {
    fn new(n_balls: usize) -> Self {
        Self {
            rng: rand::rng(),
            balls: (1..=n_balls).collect(),
        }
    }

    fn draw(&mut self) -> usize {
        self.balls.pop().unwrap_or(0)
    }

    fn shuffle(&mut self) {
        self.balls.shuffle(&mut self.rng);
    }
}
