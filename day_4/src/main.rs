use std::{error, fs::File, io::{self, BufRead, Read}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const INPUT: &str = "data/day4_1.txt";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let mut in_file = File::open(cwd.join(INPUT))?;

    let input = parse_input(&in_file)?;

    Ok(())
}

fn parse_input(f: &File) -> Result<Vec<String>> {
    let mut input = Vec::new();
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        input.push(line);
    }

    Ok(input)
}
