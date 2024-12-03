use std::{collections::HashMap, fs::File, io::{self, BufRead}, path::Path};
use std::error;

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const INPUT: &str = "input.txt";

fn main() -> Result<()> {
    /* Open Input file */
    let in_file = File::open(Path::new(INPUT))?;

    /* Parse input file */
    let mut cols = parse_input(&in_file)?;

    // let mut cols = (vec![3,4,2,1,3,3], vec![4,3,5,3,9,3]);

    cols.0.sort();
    cols.1.sort();

    /* Compute distance between lists */
    let distance = compute_distance(&cols);
    println!("Distance: {}", distance);

    let similarity = compute_similarity(&cols);
    println!("Similarity: {}", similarity);

    Ok(())
}

fn parse_input(f: &File) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<_> = line.split_whitespace().filter(|k| !k.is_empty()).collect();
        assert_eq!(tokens.len(), 2);

        left.push(tokens[0].parse()?);
        right.push(tokens[1].parse()?);
    }

    Ok((left, right))
}

fn compute_distance(cols: &(Vec<u32>, Vec<u32>)) -> u32 {
    cols.0.iter()
        .zip(cols.1.iter())
        .map(|(l,r)| l.abs_diff(*r))
        .collect::<Vec<u32>>()
        .into_iter()
        .sum::<u32>()
}

fn compute_similarity(cols: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut r_occurances: HashMap<u32, u32> = HashMap::new();

    /* Count reocurring values of the right list */
    for elem in &cols.1 {
        r_occurances.entry(*elem).and_modify(|counter| *counter += 1).or_insert(1);
    }

    cols.0.iter()
        .map(|n| n * r_occurances.get(&n).unwrap_or(&0))
        .sum()
}
