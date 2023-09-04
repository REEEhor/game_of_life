use std::error::Error;
use std::fs;
use std::iter::repeat;

pub struct Screen {
    width: usize,
    height: usize,
    data: Vec<Vec<bool>>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Screen {
            width,
            height,
            data: vec![vec![false; width]; height],
        }
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }

    pub fn with_dimensions_of(other: &Self) -> Self {
        Screen::new(other.width, other.height)
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
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

        return Ok(Screen {
            width,
            height,
            data,
        });
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut bool {
        return &mut self.data[y][x];
    }

    pub fn at(&self, x: usize, y: usize) -> bool {
        return self.data[y][x];
    }

    pub fn neighbourhood_at(&self, x: usize, y: usize) -> i32 {
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

    pub fn print(&self) {
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
