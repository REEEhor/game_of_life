pub mod screen;
use crate::simulation::screen::Screen;
use std::error::Error;

pub struct Simulation {
    buffers: Vec<Screen>,
    current_index: usize,
    width: usize,
    height: usize,
}

impl Simulation {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            buffers: vec![],
            current_index: 0,
            width,
            height,
        }
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let inital_state = Screen::load_from_file(file_path)?;
        let width = inital_state.width();
        let height = inital_state.height();

        Ok(Simulation {
            buffers: vec![inital_state],
            current_index: 0,
            width,
            height,
        })
    }

    fn last_buffer(&self) -> &Screen {
        &self.buffers[self.buffers.len() - 1]
    }

    fn current_buffer(&self) -> &Screen {
        &self.buffers[self.current_index]
    }

    fn calculate_next(&mut self) {
        let width = self.width;
        let height = self.height;
        let mut new_buffer = Screen::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let is_alive = self.last_buffer().at(x, y);
                let neighbours = self.last_buffer().neighbourhood_at(x, y);

                // rules from: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
                *new_buffer.at_mut(x, y) = (is_alive && (neighbours == 2 || neighbours == 3))
                    || (!is_alive && neighbours == 3);
            }
        }

        self.buffers.push(new_buffer);
    }

    pub fn precalculate_next_n(&mut self, n: usize) {
        self.buffers.reserve(n);
        for _ in 0..n {
            self.calculate_next();
        }
    }

    pub fn step_backwards(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub fn step_forwards(&mut self) {
        self.current_index += 1;

        let last_calculated_index = self.buffers.len() - 1;
        if self.current_index > last_calculated_index {
            self.calculate_next();
        }
    }

    pub fn print(&self) {
        print!("\x1b[2J");
        print!("\x1b[H");
        self.current_buffer().print();
    }
}
