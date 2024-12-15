use std::{collections::HashMap, error, fs::File, io::{self, BufRead}, ops::{Index, IndexMut}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day12_example.txt";
const INPUT: &str = "data/day12.txt";

const DIRS: &[(i32, i32)] = &[(1,0), (0,-1), (-1,0), (0,1)];

struct Node {
    value: char,
    visited: bool,
}

impl Node {
    fn new(value: char) -> Self {
        Self {
            value,
            visited: false,
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

impl<T> IndexMut<usize> for Map<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;
    let mut map = parse_input(&in_file)?;

    let plots = search(&mut map);

    println!("Plot count: {}", plots.len());
    let price: u32 = plots.iter().map(|(_, v)| v.1 * v.2).sum();

    println!("Price: {}", price);

    // plots.iter().for_each(|(_,v)| println!("{} {}", v.1, v.2));

    Ok(())
}

fn search(map: &mut Map<Node>) -> HashMap<(usize, usize), (u32, u32, u32)> {
    let mut found_plots = HashMap::new();

    /* Iterate over map looking for unvisited Nodes */
    for y in 0..map.height() {
        for x in 0..map.width() {

            /* Found unvisited node */
            if !map[y][x].visited {
                let (fence_cnt, area, corners) = search_plot(map, (x as i32, y as i32));
                found_plots.insert((x,y), (fence_cnt, area, corners));
            }
        }
    }

    found_plots
}

/// DFS from given point looking for all adjacent points of the same type.
///
/// * `map` - Map to search within.
/// * `point` - Point to search off of.
fn search_plot( map: &mut Map<Node>, point: (i32, i32)) -> (u32, u32, u32) {
    let x = point.0 as i32;
    let y = point.1 as i32;
    let plot_type = map[y as usize][x as usize].value;
    let mut fence_cnt = 0;
    let mut area = 0;
    let mut adj_cnt = 0;
    let mut corner_cnt = 0;

    /* Mark as visited */
    map[y as usize][x as usize].visited = true;

    /* Continue search in each direction where the node is:
    *   - Not yet visited
    *   - The same type
    *
    * Number of perimeters for current node is equal to four minus
    * number of same typed adjacent nodes.
    */
    for dir in DIRS {
        let x = x as i32 + dir.0;
        let y = y as i32 + dir.1;

        if x >= 0 && x < map.width() as i32
        && y >= 0 && y < map.height() as i32
        && map[y as usize][x as usize].value == plot_type {

            if !map[y as usize][x as usize].visited {
                let (f, a, c) = search_plot(map, (x, y));

                /* Sum fence count and area of sub tree */
                fence_cnt += f;
                area += a;
                corner_cnt += c;
            }

            /* Sum adjacents of same type */
            adj_cnt += 1;
        }
    }

    /* Count this node's corners */
    corner_cnt += count_corners(map, point);

    /* Add in fence count and area of this node */
    fence_cnt += 4 - adj_cnt;
    area += 1;

    (fence_cnt, area, corner_cnt)
}


fn count_corners(map: &mut Map<Node>, point: (i32, i32)) -> u32 {
    /*                       x,           y */
    let right = (point.0 + 1, point.1    );
    let down  = (point.0,     point.1 + 1);
    let left  = (point.0 - 1, point.1    );
    let up    = (point.0,     point.1 - 1);

    let up_right   = (point.0 + 1, point.1 - 1);
    let down_right = (point.0 + 1, point.1 + 1);
    let down_left  = (point.0 - 1, point.1 + 1);
    let up_left    = (point.0 - 1, point.1 - 1);

    let n_val = map[point.1 as usize][point.0 as usize].value;

    let mut corner_cnt: u32 = 0;

    /* Outside top-left */
    if (up.1   < 0 || map[up.1 as usize][up.0 as usize].value     != n_val)
    && (left.0 < 0 || map[left.1 as usize][left.0 as usize].value != n_val) {
        corner_cnt += 1;
    }

    /* Outside top-right */
    if (up.1    <  0                 || map[up.1 as usize][up.0 as usize].value       != n_val)
    && (right.0 >= map.width() as i32|| map[right.1 as usize][right.0 as usize].value != n_val) {
        corner_cnt += 1;
    }

    /* Outside bottom-right */
    if (down.1  >= map.height() as i32|| map[down.1 as usize][down.0 as usize].value   != n_val)
    && (right.0 >= map.width() as i32 || map[right.1 as usize][right.0 as usize].value != n_val) {
        corner_cnt += 1;
    }

    /* Outside bottom-left */
    if (down.1 >= map.height() as i32|| map[down.1 as usize][down.0 as usize].value != n_val)
    && (left.0 < 0                   || map[left.1 as usize][left.0 as usize].value != n_val) {
        corner_cnt += 1;
    }

    /* Inside top-left */
    if (up.1   >= 0 && map[up.1 as usize][up.0 as usize].value     == n_val)
    && (left.0 >= 0 && map[left.1 as usize][left.0 as usize].value == n_val)
    && (map[up_left.1 as usize][up_left.0 as usize].value          != n_val) {
        corner_cnt += 1;
    }

    /* Inside top-right */
    if (up.1    >= 0                  && map[up.1 as usize][up.0 as usize].value       == n_val)
    && (right.0 <  map.width() as i32 && map[right.1 as usize][right.0 as usize].value == n_val)
    && (map[up_right.1 as usize][up_right.0 as usize].value                            != n_val) {
        corner_cnt += 1;
    }

    /* Inside bottom-right */
    if (down.1  < map.height() as i32 && map[down.1 as usize][down.0 as usize].value   == n_val)
    && (right.0 < map.width() as i32  && map[right.1 as usize][right.0 as usize].value == n_val)
    && (map[down_right.1 as usize][down_right.0 as usize].value                         != n_val) {
        corner_cnt += 1;
    }

    /* Inside bottom-left */
    if (down.1 < map.height() as i32 && map[down.1 as usize][down.0 as usize].value == n_val)
    && (left.0 >= 0                  && map[left.1 as usize][left.0 as usize].value == n_val)
    && (map[down_left.1 as usize][down_left.0 as usize].value                        != n_val) {
        corner_cnt += 1;
    }

    corner_cnt
}

fn parse_input(f: &File) -> Result<Map<Node>> {
    let reader: io::BufReader<&File> = io::BufReader::new(f);
    let mut map = Map::new();

    for line in reader.lines() {
        let line = line?;

        let mut row = Vec::new();

        for ch in line.chars() {

            row.push(Node::new(ch));
        }

        map.add_row(row);
    }

    Ok(map)
}
