use core::panic;
use std::{ascii, cell::{BorrowError, Cell}, cmp::*, collections::{HashMap, HashSet}, error, fmt, fs::File, io::{self, BufRead, Write}};


// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Gaurd,
    Obstruction([bool; 4]),
    Visited,
    Empty,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           CellType::Gaurd => write!(f, "^"),
           CellType::Obstruction(_) => write!(f, "#"),
           CellType::Visited => write!(f, "X"),
           CellType::Empty => write!(f, "."),
       }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North = 0,
    East= 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn get_move(&self) -> (i32, i32) {
        match self {
            Self::North => (0,-1),
            Self::East => (1,0),
            Self::South => (0,1),
            Self::West => (-1,0),
        }
    }

    fn turn(&mut self) {
        match self {
            Self::North => *self = Self::East,
            Self::East  => *self = Self::South,
            Self::South => *self = Self::West,
            Self::West  => *self = Self::North,
        }
    }
}

#[derive(Clone)]
struct Board {
    area: Vec<Vec<CellType>>,
    gaurd_dir: Direction,
    initial_direction: Direction,
    unique_visits: u32,
    location: Option<(i32, i32)>,
    initial_location: (i32, i32),
}

impl Board {
    fn new() -> Self {
        Board {
            area: Vec::new(),
            gaurd_dir: Direction::North, // Assume always start facing north
            initial_direction: Direction::North,
            unique_visits: 1,
            location: None,     // None == walked out of `area`
            initial_location: (0,0),
        }
    }

    fn reset(&mut self) {
        self.gaurd_dir = self.initial_direction;
        self.location = Some(self.initial_location);
        self.unique_visits = 1;

        /* Reset Gaurd */
        let (x, y) = self.initial_location;
        let start_cell = self.get_cell(x, y).unwrap();
        *start_cell = CellType::Gaurd;

        /* Reset Obstacles */
        for row in &mut self.area {
            for cell in row.iter_mut().filter(|c| match c {CellType::Obstruction(_) => true, _ => false}) {
                *cell = CellType::Obstruction([false; 4]);
            }
        }
    }

    fn get_cell(&mut self, x: i32, y: i32) -> Option<&mut CellType> {
        if (x < 0 || x >= self.area[0].len() as i32)
        || (y < 0 || y >= self.area.len() as i32) {
            None
        } else {
            Some(&mut self.area[y as usize][x as usize])
        }
    }

    fn visit_cell(&mut self, x: i32, y: i32) {
        let curr_cell = self.get_cell(x, y).expect("Cell should exist");
        *curr_cell = CellType::Visited;
    }

    fn walk_guard(&mut self, print_board: bool) -> bool {
        while let Some(current_location) = self.location {
            let (x, y) = current_location;
            let (dx, dy) = self.gaurd_dir.get_move();

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if print_board {
                self.print_board();
            }

            let curr_dir = self.gaurd_dir;

            /* handle next step */
            match self.get_cell(nx, ny) {
                None => {
                    self.visit_cell(x, y);
                    break;
                },
                Some(new_cell) => {
                    match new_cell {
                        /* Move to empty cell */
                        CellType::Empty => {
                            *new_cell = CellType::Gaurd;
                            self.visit_cell(x, y);
                            self.location = Some((nx, ny));
                            self.unique_visits += 1;
                            }
                        CellType::Visited =>{
                            // *new_cell = CellType::Gaurd;    // Do this to help visualize loop search
                            self.visit_cell(x, y);
                            self.location = Some((nx, ny));
                        }
                        CellType::Obstruction(dirs) => {
                            /* If we've visited this before with the same Direction
                            *   we're in a cycle. */
                            if dirs[curr_dir as usize] {
                                return false;
                            }
                            else {
                                dirs[curr_dir as usize] = true;
                            }
                            self.gaurd_dir.turn();
                        }
                        CellType::Gaurd => {
                            panic!("Gaurd encountered another guard!?");
                        }
                    }
                }
            }
        // Step once and see what happens
        // self.print_board();
        // break;
        }


        true
    }

    fn print_board(&self) {
        println!("\n{:-<width$}",  "-", width = &self.area[0].len()+2);
        for row in &self.area {
            print!("|");
            for cell in row {
                print!("{}", cell);
            }
            print!("|");
            println!("");
        }
        println!("{:-<width$}",  "-", width = &self.area[0].len()+2);
    }
}


// const INPUT: &str = "data/day6_1_example.txt";
const INPUT: &str = "data/day6_1.txt";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    let mut board = parse_input(&in_file)?;

    board.print_board();

    // board.walk_guard();
    // println!("Visited {}", &board.unique_visits);

    let loop_cnt = find_loops(&mut board);
    println!("Loop count: {}", loop_cnt);

    Ok(())
}

fn find_loops(board: &mut Board) -> u32 {
    let mut loop_count = 0;

    print!("Finding loops:");

    for y in 0..board.area.len() {
        println!("");
        let _ = std::io::stdout().flush();

        for x in 0..board.area[y].len() {
            print!(".");

            // let mut scratch = board.clone(); // Clone becuase we trash the board area while walking it.
            board.reset();

            let cell = &mut board.area[y][x];
            let orig_cell_val = *cell;

            if matches!( *cell, CellType::Obstruction(..) ) || CellType::Gaurd == *cell {
                /* Skip */
                continue;
            } else {
                /* Place new Obstruction and test for loop */
                *cell = CellType::Obstruction([false; 4]);

                // println!("({}, {})", x, y);
                // board.print_board();

                // let print_board =  5 == x && 6 == y;
                let print_board =  false;

                /* Walk board looking for a loop */
                if !board.walk_guard(print_board) {
                    loop_count += 1;
                }
            }

            let cell = &mut board.area[y][x];
            *cell = orig_cell_val;
        }
    }

    println!("");

    loop_count
}

fn parse_input(file: &File) -> Result<Board> {
    let reader = io::BufReader::new(file);

    let mut board = Board::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            panic!("Unexpected empty line!");
        }

        let mut new_row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    new_row.push(CellType::Gaurd);
                    board.location = Some((x as i32, y as i32));
                    board.initial_location = board.location.unwrap();
                },
                '#' => {
                    new_row.push(CellType::Obstruction([false; 4]));
                },
                '.' => {
                    new_row.push(CellType::Empty);
                }
                _ => panic!("Unexpected input"),
            }
        }

        board.area.push(new_row);
    }

    Ok(board)

}
