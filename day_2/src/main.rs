use std::{error, fs::File, io::{self, BufRead}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day2_1_example.txt";
const INPUT: &str = "data/day2_1.txt";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    /* Parse input file */
    let input = parse_input(&in_file)?;

    /* Count valid reports */
    let mut valid_cnt = 0;

    for report in input {
        if is_report_valid2(&report) {
            valid_cnt += 1;
        }
    }

    println!("Valid reports: {}", valid_cnt);

    Ok(())
}

/// Check the validity of a report.
///
/// Each report must maintain the following invariants:
///     1. The levels are either all increasing or all
///         decreasing.
///     2. Any two adjacent levels differ by at least one
///         and at most three.
///
fn is_report_valid( report: &[u32]) -> bool {
    let is_increasing = report[0] < report[1];

    for i in 0..(report.len()-1) {
        /* Check invariant 2 */
        let diff = report[i].abs_diff(report[i+1]);
        if diff > 3 || diff < 1 {
            return false
        }

        /* Check invariant 1. */
        if is_increasing {
            if report[i] > report[i+1] {
                return false
            }
        } else {
            if report[i] < report[i+1] {
                return false
            }
        }
    }

    true
}

/// Check the validity of a report.
///
/// Each report must maintain the following invariants:
///     1. The levels are either all increasing or all
///         decreasing.
///     2. Any two adjacent levels differ by at least one
///         and at most three.
///     3. If removing one level makes the report valid,
///         consider the report valid.
///
fn is_report_valid2( report: &[u32]) -> bool {
    let mut copy = report.to_vec();

    let mut i: usize = 0;

    while !is_report_valid(&copy) {
        if i == report.len() {
            return false;
        }

        /* Get original report */
        copy = report.to_vec();
        /* See if removing next index yeilds a valid report */
        copy.remove(i);

        i += 1;
    }

    true
}


fn parse_input(f: &File) -> Result<Vec<Vec<u32>>> {
    let mut reports = Vec::new();
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        let mut levels = Vec::new();

        for ch in line.split_whitespace() {
            let lvl: u32 = ch.parse()?;
            levels.push(lvl);
        }
        reports.push(levels);
    }

    Ok(reports)
}
