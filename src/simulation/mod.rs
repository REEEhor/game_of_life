pub mod screen;
use crate::simulation::screen::Screen;
use std::error::Error;

pub struct Simulation {
    buffers: [Screen; 2],
    current_buffer_id: bool,
}

impl Simulation {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            buffers: [Screen::new(width, height), Screen::new(width, height)],
            current_buffer_id: false,
        }
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let buffer1 = Screen::load_from_file(file_path)?;
        let buffer2 = Screen::with_dimensions_of(&buffer1);

        Ok(Simulation {
            buffers: [buffer2, buffer1],
            current_buffer_id: false,
        })
    }

    fn current_buffer_mut(&mut self) -> &mut Screen {
        &mut self.buffers[self.current_buffer_id as usize]
    }

    fn current_buffer(&self) -> &Screen {
        &self.buffers[self.current_buffer_id as usize]
    }

    fn previous_buffer(&self) -> &Screen {
        &self.buffers[!self.current_buffer_id as usize]
    }

    fn swap_buffers(&mut self) {
        self.current_buffer_id = !self.current_buffer_id;
    }

    pub fn update(&mut self) {
        let width = self.current_buffer_mut().width();
        let height = self.current_buffer_mut().height();

        for y in 0..height {
            for x in 0..width {
                let is_alive = self.previous_buffer().at(x, y);
                let neighbours = self.previous_buffer().neighbourhood_at(x, y);

                // rules from: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
                *self.current_buffer_mut().at_mut(x, y) = (is_alive
                    && (neighbours == 2 || neighbours == 3))
                    || (!is_alive && neighbours == 3);
            }
        }

        self.swap_buffers();
    }

    pub fn print(&self) {
        print!("\x1b[2J");
        print!("\x1b[H");
        self.current_buffer().print();
    }
}
