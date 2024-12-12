use std::{ error, fs::File, io::{self, BufRead}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day11_example.txt";
const INPUT: &str = "data/day11.txt";

struct Entry {
    value: u64,
    cnt: u64,
}

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    let mut numbers = Box::new(parse_input(&in_file)?);

    for _ in 0..75 {
        blink(&mut numbers)?;
    }

    println!("Count: {}", numbers.len());

    Ok(())
}

fn blink(numbers: &mut Vec<u64>) -> Result<()> {

    /* Iterrate over ORIGINAL list */
    for idx in 0..numbers.len() {
        let n = numbers[idx];

        /* Rule 1 - Number is 0 */
        if n == 0 {
            numbers[idx] = 1;
        }
        /* Rule 2 - Number has even digits */
        else if let Some(n_str) = check_rule_2(n) {
            let mid = n_str.len() / 2;
            let n1: u64 = n_str[0..mid].parse()?;
            let n2: u64 = n_str[mid..].parse()?;

            numbers[idx] = n1;
            numbers.push(n2);
        }
        /* Rule 3 - Number has odd digits */
        else {
            numbers[idx] = n * 2024;
        }
    }

    Ok(())
}

fn check_rule_2(n: u64) -> Option<String> {
    let n_str = n.to_string();
    if n_str.len() % 2 == 0 {
        Some(n_str)
    } else {
        None
    }
}

fn parse_input(f: &File) -> Result<Vec<u64>> {
    let mut numbers = Vec::new();
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split(" ").filter(|k| !k.is_empty()).collect();

        for token in tokens {
            let n = token.parse()?;
            numbers.push(n);
        }
    }

    Ok(numbers)
}
