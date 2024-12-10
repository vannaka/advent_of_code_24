use std::{collections::{HashMap, HashSet}, error, fs::File, io::{self, BufRead}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day8_example.txt";
const INPUT: &str = "data/day8.txt";

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

type Antenna = Point;
type Antinode = Point;


fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;
    let (map_sz, antenna_sets) = parse_input(&in_file)?;

    let mut antinodes = HashSet::new();

    for (_, antennas) in antenna_sets {

        /* Scan over every combination of antenna in this group */
        for i in 0..antennas.len() - 1 {
            let v1 = &antennas[i];
            for v2 in &antennas[i+1..] {
                /* Calculate slope */
                let rise = v2.y - v1.y;
                let run = v2.x - v1.x;

                /* Add the antenna as antinodes */
                antinodes.insert(Antinode::new(v1.x, v1.y));
                antinodes.insert(Antinode::new(v2.x, v2.y));

                /* Search backwards */
                let mut nx = v1.x;
                let mut ny = v1.y;
                loop {
                    nx -= run;
                    ny -= rise;

                    if nx < 0 || nx >= map_sz.x || ny < 0 || ny >= map_sz.y {
                        break;
                    }

                    antinodes.insert(Antinode::new(nx, ny));
                }

                /* Search Forwards */
                let mut nx = v2.x;
                let mut ny = v2.y;
                loop {
                    nx += run;
                    ny += rise;

                    if nx < 0 || nx >= map_sz.x || ny < 0 || ny >= map_sz.y {
                        break;
                    }

                    antinodes.insert(Antinode::new(nx, ny));
                }
            }
        }
    }

    println!("Antinode count: {}", antinodes.len());

    Ok(())
}


fn parse_input(f: &File) -> Result<(Point, HashMap<char, Vec<Antenna>>)> {

    let reader: io::BufReader<&File> = io::BufReader::new(f);
    let mut antenna_sets = HashMap::new();
    let mut map_size = Point {x: 0, y: 0};

    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        map_size.x = line.len() as i32;
        map_size.y += 1;

        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                let set = antenna_sets.entry(ch).or_insert_with(|| Vec::new());
                set.push(Antenna::new(x as i32, y as i32));
            }
        }
    }

    Ok((map_size, antenna_sets))
}
