use std::{error, fs::File, io::{self, BufRead}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const INPUT: &str = "data/day8_example.txt";
// const INPUT: &str = "data/day8.txt";

struct Point {
    x: usize,
    y: usize,
}

struct Antenna {
    freq: char,
    point: Point,
}

struct Antinode {

    point: Point,
}

impl Antenna {
    fn new(x: usize, y: usize, freq: char) -> Self {
        Self {
            freq,
            point: Point {x, y},
        }
    }
}

impl Antinode {
    fn new(x: u32, y: u32) -> Self {
        Self {
            point: Point {x, y},
        }
    }
}

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    Ok(())
}


fn parse_input(f: &File) -> Result<Vec<Antenna>> {
    let mut antennas = Vec::new();
    let reader: io::BufReader<&File> = io::BufReader::new(f);

    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.push(Antenna::new(x, y, ch));
            }
        }
    }

    Ok(antennas)
}
