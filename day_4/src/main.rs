use std::{error, fs::File, io::{self, BufRead}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const INPUT: &str = "data/day4_1.txt";
// const INPUT: &str = "data/day4_1_example.txt";
const SEARCH_WORD: &str = "XMAS";
const SEARCH_WORD2: &str = "MAS";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    let input = parse_input(&in_file)?;

    let count = count_matches(&input, SEARCH_WORD);
    let count2 = count_matches2(&input, SEARCH_WORD2);

    println!("Count: {}", count);
    println!("Count2: {}", count2);

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

fn count_matches( input: &Vec<String>, word: &str) -> u32 {
    let directions: Vec<(i32, i32)> = vec![(1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1)];  // (x, y) CCW in 45deg increments, starting in the east position.

    let mut count = 0u32;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for dir in &directions {

                if find_word(&input, word, x, y, dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

// Search for `word` within `input` at the given rotation (`dir`) around `x`,`y`. With the
//  first letter of `word` used as the rotation point.
fn find_word( input: &Vec<String>, word: &str, x: usize, y: usize, dir: &(i32, i32) ) -> bool {
    let mut matches = true;

    for i in 0..word.len() {
        let x = x as i32;
        let y = y as i32;

        let dx = i as i32 * dir.0;
        let dy = i as i32 * dir.1;

        /* Is index within input? */
        if let Some((nx, ny)) = is_inside(&input, x + dx, y + dy) {
            /* Check char for match */
            if input[ny].as_bytes()[nx] != word.as_bytes()[i] {
                matches = false;
                break;
            }
        }
        else {
            matches = false;
            break;
        }
    }

    matches
}

fn count_matches2( input: &Vec<String>, word: &str) -> u32 {
    let directions: Vec<(i32, i32)> = vec![(1,1),(-1,1),(-1,-1),(1,-1)];  // north-east, north-west, south-west, south-east

    let mut count = 0u32;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let mut dirs_matched = 0;
            for dir in &directions {
                /* Search all 4 angled dirs, but only 2 can ever match within one kernel */
                if find_word2(&input, word, x, y, dir) {
                    dirs_matched += 1;
                }
            }

            if dirs_matched >= 2 {
                count += 1;
            }
        }
    }

    count
}

// Search for `word` within `input` at the given rotation (`dir`) around `x`,`y`. With the
//  center of `word` used as the rotation point.
fn find_word2( input: &Vec<String>, word: &str, x: usize, y: usize, dir: &(i32, i32) ) -> bool {
    let mut matches = true;

    let x = x as i32;
    let y = y as i32;

    let half = (word.len() / 2) as i32;

    /* Shift x,y to a corner of the kernel in the opposite of dir. */
    let x  = x - (half * dir.0);
    let y = y - (half * dir.1);

    for i in 0..word.len() {

        let dx = i as i32 * dir.0;
        let dy = i as i32 * dir.1;

        /* Is index within input? */
        if let Some((nx, ny)) = is_inside(&input, x + dx, y + dy) {
            /* Check char for match */
            if input[ny].as_bytes()[nx] != word.as_bytes()[i] {
                matches = false;
                break;
            }
        }
        else {
            matches = false;
            break;
        }
    }

    matches
}

fn is_inside( input: &Vec<String>, x: i32, y: i32 ) -> Option<(usize, usize)> {
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    if x >= 0 && x < width && y >= 0 && y < height {
        Some((x as usize, y as usize))
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_input() {
        let input = vec![
            String::from(".M.S......"),
            String::from("..A..MSMS."),
            String::from(".M.S.MAA.."),
            String::from("..A.ASMSM."),
            String::from(".M.S.M...."),
            String::from(".........."),
            String::from("S.S.S.S.S."),
            String::from(".A.A.A.A.."),
            String::from("M.M.M.M.M."),
            String::from(".........."),
        ];

        assert_eq!(9, count_matches2(&input, "MAS"));
    }

    #[test]
    fn simple_input() {
        let input = vec![
            String::from("M.S"),
            String::from(".A."),
            String::from("M.S"),
        ];

        assert_eq!(1, count_matches2(&input, "MAS"));
    }
}
