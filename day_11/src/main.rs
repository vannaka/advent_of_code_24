use std::{ collections::HashMap, error, fs::File, io::{self, BufRead}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day11_example.txt";
const INPUT: &str = "data/day11.txt";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    let mut numbers = parse_input(&in_file)?;

    // println!("{:?}", numbers);

    for _ in 0..75 {
        numbers = blink(numbers)?;
        // println!("{:?}", numbers);
    }

    println!("Count: {}", numbers.iter().map(|(_,v)| v ).sum::<u64>());

    Ok(())
}

fn blink(numbers: HashMap<u64, u64>) -> Result<HashMap<u64, u64>> {
    let mut new_numbers: HashMap<u64, u64> = HashMap::new();

    /* Iterrate over list */
    for (n, cnt) in numbers {

        /* Rule 1 - Number is 0 */
        /* 0 -> 1 */
        if n == 0 {
            let entry = new_numbers.entry(1).or_insert(0);
            *entry += cnt;
        }
        /* Rule 2 - Number has even digits */
        /* Split in two at midpoint of digits */
        else if let Some(n_str) = check_rule_2(n) {
            let mid = n_str.len() / 2;
            let n1: u64 = n_str[0..mid].parse()?;
            let n2: u64 = n_str[mid..].parse()?;

            let entry = new_numbers.entry(n1).or_insert(0);
            *entry += cnt;
            let entry = new_numbers.entry(n2).or_insert(0);
            *entry += cnt;
        }
        /* Rule 3 - Number has odd digits */
        /* Multiply by 2024 */
        else {
            let entry = new_numbers.entry(n*2024u64).or_insert(0);
            *entry += cnt;
        }
    }

    Ok(new_numbers)
}

fn check_rule_2(n: u64) -> Option<String> {
    let n_str = n.to_string();
    if n_str.len() % 2 == 0 {
        Some(n_str)
    } else {
        None
    }
}

fn parse_input(f: &File) -> Result<HashMap<u64, u64>> {
    let mut numbers = HashMap::new();
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split(" ").filter(|k| !k.is_empty()).collect();

        for token in tokens {
            let n = token.parse::<u64>()?;

            let entry = numbers.entry(n).or_insert(0);
            *entry += 1;
        }
    }

    Ok(numbers)
}
