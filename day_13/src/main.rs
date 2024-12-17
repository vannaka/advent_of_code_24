use std::{error, fs::File, io::{self, BufRead}};
use sscanf::sscanf;

use num_integer::{self, Integer};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day13_example.txt";
const INPUT: &str = "data/day13.txt";

struct Pair {
    x: i128,
    y: i128,
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
fn find_moves(game: &Game) -> Option<(i128, i128)> {
    /* shortcut vars */
    let prize = &game.prize;
    let a = &game.a;
    let b = &game.b;

    /* Solve system of linear equation */
    let (b_cnt, b_rem) = (prize.y*a.x - prize.x*a.y).div_rem(&(a.x*b.y - a.y*b.x));
    let (a_cnt, a_rem) = (prize.x - b_cnt*b.x).div_rem(&a.x);

    /* Check for invalid solutions */
    if a_cnt < 0
    // || a_cnt > 100
    || b_cnt < 0
    // || b_cnt > 100
    || a_rem != 0
    || b_rem != 0 {
        return None
    }

    Some((a_cnt, b_cnt))
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
        let (_, x, y) = sscanf!(line, "Button {}: X+{}, Y+{}", char, i128, i128)?;
        game.a.x = x;
        game.a.y = y;

        /* Get Button B data */
        let line = lines.next().expect("B data missing")?;
        let (_, x, y) = sscanf!(line, "Button {}: X+{}, Y+{}", char, i128, i128)?;
        game.b.x = x;
        game.b.y = y;

        /* Get Prize data */
        let line = lines.next().expect("Prize data missing")?;
        let (x, y) = sscanf!(line, "Prize: X={}, Y={}", i128, i128)?;
        game.prize.x = x + 10000000000000;
        game.prize.y = y + 10000000000000;


        /* Consume empty line */
        let _ = lines.next();

        games.push(game);
    }

    Ok(games)
}
