use std::mem::swap;

fn wait_for_enter() {
    let mut tmp = String::new();
    let _ = std::io::stdin().read_line(&mut tmp);
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
            data: vec![vec![false; height]; width],
        }
    }

    fn at(&mut self, x: usize, y: usize) -> &mut bool {
        return &mut self.data[x][y];
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
                result += self.data[x][y] as i32;
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

fn update<'a>(current_buffer: &'a mut Buffer, prev_buffer: &'a mut Buffer) {
    let width = current_buffer.width;
    let height = current_buffer.height;

    for y in 0..height {
        for x in 0..width {
            let is_alive = *prev_buffer.at(x, y);
            let neighbours = prev_buffer.neighbourhood_at(x, y);

            // rules from: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
            *current_buffer.at(x, y) = (is_alive && (neighbours == 2 || neighbours == 3))
                || (!is_alive && neighbours == 3);
        }
    }

    swap(current_buffer,  prev_buffer);
}

fn main() {
    let width = 20;
    let height = 20;
    let mut buffer1: Buffer = Buffer::new(width, height);
    let mut buffer2: Buffer = Buffer::new(width, height);
    let mut current_buffer = &mut buffer1;
    let mut prev_buffer = &mut buffer2;

    let data = &mut prev_buffer.data;

    // generate glider
    data[0][1] = true;
    data[1][2] = true;
    data[2][0] = true;
    data[2][1] = true;
    data[2][2] = true;

    loop {
        update(&mut current_buffer, &mut prev_buffer);
        current_buffer.print();
        wait_for_enter();
    }
}
