use std::{collections::HashMap, fs::File, io::{self, BufRead}, path::Path};

const INPUT: &str = "input.txt";

fn main() {
    /* Open Input file */
    let in_file = File::open(Path::new(INPUT)).expect("input.txt cannot be opened!");

    /* Parse input file */
    let (left, right) = parse_input(&in_file);

    // let left = vec![3,4,2,1,3,3];
    // let right = vec![4,3,5,3,9,3];

    /* Compute distance between lists */
    let distance = compute_distance(&left, &right);
    println!("The distance between the left and right lists is: {}", distance);

    let similarity = compute_similarity(&left, &right);
    println!("The similarity between the left and right lists is: {}", similarity);
}

fn parse_input(f: &File) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let reader = io::BufReader::new(f);

    for line in reader.lines().map(|l| l.expect("Error while reading line")) {
        let tokens: Vec<_> = line.split_whitespace().filter(|k| !k.is_empty()).collect();
        assert_eq!(tokens.len(), 2);

        left.push(tokens[0].parse().expect("parsing error"));
        right.push(tokens[1].parse().expect("parsing error"));
    }

    /* Sort, lowest to highest */
    left.sort();
    right.sort();

    (left, right)
}

fn compute_distance(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut distance = 0;
    for i in 0..v1.len() {
        distance += (v1[i] - v2[i]).abs();
    }

    distance
}


fn compute_similarity(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut similarity = 0;
    let mut r_occurances = HashMap::new();

    /* Count reocurring values of the right list */
    for elem in v2 {
        r_occurances.entry(*elem).and_modify(|counter| *counter += 1).or_insert(1);
    }

    for n in v1 {
        let count = r_occurances.get(n).unwrap_or(&0);

        similarity += n * count;
    }

    similarity
}
