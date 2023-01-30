enum ParseError {
    InvalidWidth { arg: &'static str },
    InvalidHeight { arg: &'static str },
    InvalidPercentage { arg: &'static str },
    TooManyArguments,
    NotEnoughArguments,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl Cell {
    pub fn random_cell(percent_alive: u32) -> Self {
        let n = ftkit::random_number(0..100) as u32;

        if n < percent_alive {
            Self::Alive
        } else {
            Self::Dead
        }
    }

    fn is_alive(self) -> bool {
        self == Cell::Alive
    }

    #[allow(dead_code)]
    fn is_dead(self) -> bool {
        self == Cell::Dead
    }
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize, percentage: u32) -> Self {
        let mut cells = Vec::new();
        for _ in 0..width * height {
            cells.push(Cell::random_cell(percentage));
        }

        Self {
            width,
            height,
            cells,
        }
    }

    fn from_args() -> Result<Self, ParseError> {
        if ftkit::ARGS.len() < 4 {
            return Err(ParseError::NotEnoughArguments);
        }

        if ftkit::ARGS.len() > 4 {
            return Err(ParseError::TooManyArguments);
        }

        let width = match ftkit::ARGS[1].parse() {
            Ok(ok) => ok,
            Err(_) => {
                return Err(ParseError::InvalidWidth {
                    arg: &ftkit::ARGS[1],
                })
            }
        };

        let height = match ftkit::ARGS[2].parse() {
            Ok(ok) => ok,
            Err(_) => {
                return Err(ParseError::InvalidHeight {
                    arg: &ftkit::ARGS[2],
                })
            }
        };

        let percentage = match ftkit::ARGS[3].parse::<u32>() {
            Ok(ok) if ok <= 100 => ok,
            _ => {
                return Err(ParseError::InvalidPercentage {
                    arg: &ftkit::ARGS[3],
                })
            }
        };

        Ok(Self::new(width, height, percentage))
    }

    fn get(&self, mut x: isize, mut y: isize) -> Cell {
        while x < 0 {
            x += self.width as isize;
        }
        let x = x as usize % self.width;
        while y < 0 {
            y += self.height as isize;
        }
        let y = y as usize % self.height;

        self.cells[x + y * self.width]
    }

    fn step(&mut self) {
        let mut next_board = Vec::new();

        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                let neighbors = self.get(x - 1, y).is_alive() as u32
                    + self.get(x + 1, y).is_alive() as u32
                    + self.get(x, y - 1).is_alive() as u32
                    + self.get(x, y + 1).is_alive() as u32
                    + self.get(x + 1, y + 1).is_alive() as u32
                    + self.get(x + 1, y - 1).is_alive() as u32
                    + self.get(x - 1, y + 1).is_alive() as u32
                    + self.get(x - 1, y - 1).is_alive() as u32;

                let new_cell = match (self.get(x, y), neighbors) {
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };

                next_board.push(new_cell);
            }
        }

        self.cells = next_board;
    }

    #[allow(clippy::print_with_newline)]
    fn print(&self, clear: bool) {
        if clear {
            print!("\x1B[{}A\x1B[J", self.height + 2);
        }
        for _ in 0..self.width {
            print!("░░");
        }
        print!("░░░░\n");
        for y in 0..self.height {
            print!("░░");
            for x in 0..self.width {
                let r = ((x as f32 / self.width as f32) * 255.0) as u8;
                let g = ((y as f32 / self.height as f32) * 255.0) as u8;
                let b = 255;
                match self.get(x as isize, y as isize) {
                    Cell::Alive => print!("\x1B[38;2;{r};{g};{b}m█▓\x1B[0m"),
                    Cell::Dead => print!("  "),
                }
            }
            print!("░░\n");
        }
        for _ in 0..self.width {
            print!("░░");
        }
        println!("░░░░");
    }
}

fn main() {
    let mut board = match Board::from_args() {
        Ok(ok) => ok,
        Err(err) => {
            match err {
                ParseError::InvalidWidth { arg } => {
                    eprintln!("error: '{arg}' is not a valid width")
                }
                ParseError::InvalidHeight { arg } => {
                    eprintln!("error: '{arg}' is not a valid height")
                }
                ParseError::InvalidPercentage { arg } => {
                    eprintln!("error: '{arg}' is not a valid percentage")
                }
                ParseError::TooManyArguments => eprintln!("error: too many arguments"),
                ParseError::NotEnoughArguments => eprintln!("error: not enough arguments"),
            }
            return;
        }
    };

    board.print(false);
    loop {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
        board.step();
        board.print(true);
    }
}
