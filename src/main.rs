use std::error::Error;
use std::fs;
use std::iter::repeat;

fn wait_for_enter() {
    let _ = std::io::stdin().read_line(&mut String::new());
}

struct Buffer {
    width: usize,
    height: usize,
    data: Vec<Vec<bool>>,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            data: vec![vec![false; width]; height],
        }
    }

    fn with_dimensions_of(other: &Self) -> Self {
        Buffer::new(other.width, other.height)
    }

    fn load_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        let mut lines = contents.lines();
        let first_line = lines.next().ok_or("Could not read the first line")?;
        let mut values_on_first_line = first_line.split(' ');
        let width = values_on_first_line
            .next()
            .ok_or("No width in file found")?
            .parse::<usize>()?;

        let height = values_on_first_line
            .next()
            .ok_or("No height in file found")?
            .parse::<usize>()?;

        let mut data = Vec::with_capacity(height);
        //
        for line in lines.take(height) {
            let mut data_line = Vec::with_capacity(width);
            //
            for symbol in line.chars().take(width) {
                match symbol {
                    '@' => {
                        data_line.push(true);
                    }
                    ' ' => {
                        data_line.push(false);
                    }
                    invalid_symbol => {
                        return Err(format!("Invalid symbol '{}'", invalid_symbol))?;
                    }
                }
            }
            let to_add_count = usize::checked_sub(width, data_line.len()).unwrap_or(0usize);

            data_line.extend(repeat(false).take(to_add_count));
            data.push(data_line);
        }
        let to_add_count = usize::checked_sub(height, data.len()).unwrap_or(0usize);
        data.extend(repeat(vec![false; width]).take(to_add_count));

        return Ok(Buffer {
            width,
            height,
            data,
        });
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut bool {
        return &mut self.data[y][x];
    }

    fn at(&self, x: usize, y: usize) -> bool {
        return self.data[y][x];
    }

    fn neighbourhood_at(&self, x: usize, y: usize) -> i32 {
        let mut result = 0;
        let x = x as i32;
        let y = y as i32;
        let cords = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            //
            (x - 1, y),
            // (x, y), // left out
            (x + 1, y),
            //
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        for (x, y) in cords {
            if (x >= 0 && y >= 0) && (x < self.width as i32 && y < self.height as i32) {
                let x = x as usize;
                let y = y as usize;
                result += self.at(x, y) as i32;
            }
        }

        return result;
    }

    fn print(&self) {
        for line in &self.data {
            for &is_alive in line {
                print!("{}", {
                    if is_alive {
                        "\x1b[46m  \x1b[0m"
                    } else {
                        "\x1b[47m  \x1b[0m"
                    }
                });
            }
            println!();
        }
    }
}

struct Simulation {
    buffers: [Buffer; 2],
    current_buffer_id: bool,
}

impl Simulation {
    #[allow(dead_code)]
    fn new(width: usize, height: usize) -> Self {
        Simulation {
            buffers: [Buffer::new(width, height), Buffer::new(width, height)],
            current_buffer_id: false,
        }
    }

    fn load_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let buffer1 = Buffer::load_from_file(file_path)?;
        let buffer2 = Buffer::with_dimensions_of(&buffer1);

        Ok(Simulation {
            buffers: [buffer2, buffer1],
            current_buffer_id: false,
        })
    }

    fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_id as usize]
    }

    fn current_buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer_id as usize]
    }

    fn previous_buffer(&self) -> &Buffer {
        &self.buffers[!self.current_buffer_id as usize]
    }

    fn swap_buffers(&mut self) {
        self.current_buffer_id = !self.current_buffer_id;
    }

    fn update(&mut self) {
        let width = self.current_buffer_mut().width;
        let height = self.current_buffer_mut().height;

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

    fn print(&self) {
        self.current_buffer().print();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut simulation = Simulation::load_from_file("input.txt")?;

    loop {
        simulation.update();
        simulation.print();
        wait_for_enter();
    }
}
