use std::{error, fs::File, io::{self, BufRead}};
use sscanf::sscanf;

extern crate nalgebra as na;
use na::{Matrix2, Matrix2x1};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day13_example.txt";
const INPUT: &str = "data/day13.txt";

struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    fn new() -> Self {
        Pair { x: 0, y: 0 }
    }
}

struct Game {
    a: Pair,
    b: Pair,
    prize: Pair,
}

impl Game {
    fn new() -> Self {
        Self {
            a: Pair::new(),
            b: Pair::new(),
            prize: Pair::new(),
        }
    }

    // fn print(self: &Self) {
    //     println!("Button A: X+{}, Y+{}", self.a.x, self.a.y);
    //     println!("Button B: X+{}, Y+{}", self.b.x, self.b.y);
    //     println!("Prize: X={}, Y={}", self.prize.x, self.prize.y);
    // }
}

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    let games = parse_input(&in_file)?;

    let mut cost = 0;

    for game in &games {
        if let Some((a_cnt, b_cnt)) = find_moves(game) {
            // println!("A: {}, B: {}", a_cnt, b_cnt);
            let icost = a_cnt * 3 + b_cnt;
            cost += icost;
        } else {
            // println!("No solution");
        }
    }

    // print_games(&games);

    println!("Cost: {}", cost);

    Ok(())
}

/// Find the number of presses of the A and B buttons
/// that are needed to reach the prize.
///
fn find_moves(game: &Game) -> Option<(u32, u32)> {
    /* shortcut vars */
    let prize = &game.prize;
    let a = &game.a;
    let b = &game.b;

    /* solve system of linear equation */
    let a_mtx = Matrix2::new(a.x as f64, b.x as f64, a.y as f64, b.y as f64);
    let b_mtx = Matrix2x1::new(prize.x as f64, prize.y as f64);

    let mut a_mtx_inv: na::Matrix<f64, na::Const<2>, na::Const<2>, na::ArrayStorage<f64, 2, 2>> = Default::default();
    let _ = na::try_invert_to(a_mtx, &mut a_mtx_inv);

    let res = a_mtx_inv * b_mtx;

    /* Verify results */
    let a_cnt = res[0];
    let b_cnt = res[1];

    /* Number is very close to a whole number */
    if f64::abs(a_cnt - ((a_cnt as u32) as f64)) < 0.0001 {

        let a_cnt = a_cnt.round();
        let b_cnt = b_cnt.round();

        /* One of them is negative */
        if a_cnt < 0. || b_cnt < 0. {
            return None;
        }

        return Some((a_cnt as u32, b_cnt as u32));
    }

    None
}

// fn print_games( games: &Vec<Game>) {
//     for game in games {
//         game.print();
//         println!("");
//     }
// }

fn parse_input(f: &File) -> Result<Vec<Game>> {
    let mut games = Vec::new();
    let reader: io::BufReader<&File> = io::BufReader::new(f);

    let mut lines = reader.lines();
    loop {
        /* Get A data and check for end of stream */
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.expect("A data missing")?;

        let mut game = Game::new();

        /* Get Button A data */
        let (_, x, y) = sscanf!(line, "Button {}: X+{}, Y+{}", char, i32, i32)?;
        game.a.x = x;
        game.a.y = y;

        /* Get Button B data */
        let line = lines.next().expect("B data missing")?;
        let (_, x, y) = sscanf!(line, "Button {}: X+{}, Y+{}", char, i32, i32)?;
        game.b.x = x;
        game.b.y = y;

        /* Get Prize data */
        let line = lines.next().expect("Prize data missing")?;
        let (x, y) = sscanf!(line, "Prize: X={}, Y={}", i32, i32)?;
        game.prize.x = x;
        game.prize.y = y;


        /* Consume empty line */
        let _ = lines.next();

        games.push(game);
    }

    Ok(games)
}
