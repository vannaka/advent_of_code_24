use std::{error, fs::File, io::Read};

use regex::Regex;

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const INPUT: &str = "data/day3_1.txt";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let mut in_file = File::open(cwd.join(INPUT))?;

    let mut input = String::new();
    in_file.read_to_string(&mut input)?;

    let total = run_calculations(&input)?;

    println!("Result: {}", total);

    Ok(())
}

fn run_calculations(input: &str) -> Result<u32> {
    let mut total = 0;
    let mut should_do = true;

    // let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
    let re = Regex::new(r"(?:mul\(([0-9]{1,3}),([0-9]{1,3})\))|(?:do\(\))|(?:don't\(\))")?;
    for caps in re.captures_iter(&input) {
        match &caps[0] {
            "do()" => should_do = true,
            "don't()" =>  should_do = false,
            _ => {
                if should_do {
                    let l: u32 = caps[1].parse()?;
                    let r: u32 = caps[2].parse()?;

                    total += l * r;
                }
            }
        }
    }

    Ok(total)
}
