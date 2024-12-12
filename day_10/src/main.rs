use std::{collections::HashSet, error, fs::File, hash::Hash, io::{self, BufRead}, ops::Index};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const INPUT: &str = "data/day10_example.txt";
// const INPUT: &str = "data/day10.txt";

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x, y,
        }
    }
}

struct Map<T>(Vec<Vec<T>>);

impl<T> Map<T> {
    fn new() -> Self {
        Self {
            0: Vec::new()
        }
    }

    fn add_row(self: &mut Self, row: Vec<T>) {
        self.0.push(row);
    }

    fn width(self: &Self) -> usize {
        if self.0.len() > 0 {
            self.0[0].len()
        } else {
            0
        }
    }

    fn height(self: &Self) -> usize {
        self.0.len()
    }
}

impl<T> Index<usize> for Map<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;
    let (map, trailheads) = parse_input(&in_file)?;

    let mut total_score = 0;
    let mut total_rating = 0;
    for head in trailheads {
        let mut found_paths = HashSet::new();

        let rating = search(&map, head.x, head.y, &mut found_paths);

        total_score += found_paths.len();
        total_rating += rating;
    }

    println!("Score: {}, Rating: {}", total_score, total_rating);

    Ok(())
}

fn search(map: &Map<u32>, x: usize, y: usize, paths: &mut HashSet<(usize, usize)>) -> u32 {
    let dirs = vec![(-1,0), (1,0), (0,-1), (0,1)];
    let mut cnt_found = 0;

    /* current point's value */
    let n = map[y][x];

    /* Base Cases */
    /* We found a complete trail */
    if n == 9 {
        /* Add point to unique ending point set */
        paths.insert((x,y));

        return 1;
    }

    /* Nowhere to go */
    for dir in dirs {
        let x = x as i32 + dir.0;
        let y = y as i32 + dir.1;

        if x >= 0 && x < map.width() as i32
        && y >= 0 && y < map.height() as i32
        && map[y as usize][x as usize] == n+1 {
            cnt_found += search(map, x as usize, y as usize, paths );
        }
    }

    cnt_found
}


fn parse_input(f: &File) -> Result<(Map<u32>, Vec<Point>)> {
    let reader: io::BufReader<&File> = io::BufReader::new(f);
    let mut map = Map::new();
    let mut trailheads = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        let mut row = Vec::new();

        for (x, ch) in line.chars().enumerate() {
            let n = ch.to_digit(10).expect("char is not digit");

            row.push(n);

            /* This is a trailhead */
            if 0 == n {
                trailheads.push(Point::new(x,y));
            }
        }

        map.add_row(row);
    }

    Ok((map, trailheads))
}
