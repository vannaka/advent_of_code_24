use std::{collections::{HashMap, HashSet}, error, fs::File, io::{self, BufRead}, cmp::*};


// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day5_1_example.txt";
const INPUT: &str = "data/day5_1.txt";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    let (pg2subs, print_lists) = parse_input(&in_file)?;

    /* Find correct print lists */
    let (correct_lists, incorrect_lists) =
        validate_print_lists(print_lists, &pg2subs);

    let mid_sum = sum_mids(&correct_lists);

    println!("1. Total: {}, Valid: {}, Sum: {}", correct_lists.len() + incorrect_lists.len(), correct_lists.len(), mid_sum);

    /* Fix incorrect lists */
    let corrected_lists =
        fix_lists(incorrect_lists, &pg2subs);

    let mid_sum = sum_mids(&corrected_lists);

    println!("2. Sum: {}", mid_sum);

    Ok(())
}

fn sum_mids(lists: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for list in lists {
        sum += list[list.len()/2];
    }

    sum
}

fn validate_print_lists(print_lists: Vec<Vec<u32>>, pg2subs: &HashMap<u32, HashSet<u32>>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut correct_lists = Vec::new();
    let mut incorrect_lists = Vec::new();

    for print_list in print_lists {
        if validate_print_list(&print_list, &pg2subs) {

            correct_lists.push(print_list);
        } else {
            incorrect_lists.push(print_list);
        }
    }

    (correct_lists, incorrect_lists)
}

fn validate_print_list(print_list: &Vec<u32>, pg2subs: &HashMap<u32, HashSet<u32>>) -> bool {

    for (i, pg_num) in print_list.iter().enumerate() {
        let subs =
        match pg2subs.get(pg_num) {
            Some(subs) => subs,
            None => continue,
        };

        /* Verify order */
        for idx in 0..i {
            /* Sub appears in print list before current page */
            if subs.contains(&print_list[idx]) {
                return false;
            }
        }
    }

    true
}

fn fix_lists(mut bad_lists: Vec<Vec<u32>>, pg2subs: &HashMap<u32, HashSet<u32>>) -> Vec<Vec<u32>> {
    for bad_list in &mut bad_lists {
        bad_list.sort_by(|l: &u32, r| {
            /* l < r: l must come before r */
            if let Some(subs) = pg2subs.get(l) {
                if subs.contains(r) {
                    return Ordering::Less
                }
            }

            /* l > r: l must come after r */
            if let Some(subs) = pg2subs.get(r) {
                if subs.contains(l) {
                    return Ordering::Greater
                }
            }

            /* r isn't a sub of l nor is l a sub of r. */
            Ordering::Equal
        });
    }

    bad_lists
}


fn parse_input(f: &File) -> Result<(HashMap<u32, HashSet<u32>>,Vec<Vec<u32>>)> {
    let mut relations: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut print_lists: Vec<Vec<u32>> = Vec::new();
    let reader = io::BufReader::new(f);
    let mut parsing_first = true;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            parsing_first = false;
            continue;
        }

        /* Parsing relations */
        if parsing_first {
            let tokens: Vec<&str> = line.split("|").filter(|k| !k.is_empty()).collect();
            assert_eq!(tokens.len(), 2);

            let high_pg: u32 = tokens[0].parse()?;
            let low_pg: u32 = tokens[1].parse()?;

            let low_pages =
                relations.entry(high_pg).or_insert_with(|| HashSet::new());

            low_pages.insert(low_pg);
        }
        /* Parsing print lists */
        else {
            let mut print_list: Vec<u32> = Vec::new();

            for n in line.split(",").filter(|k| !k.is_empty()) {
                print_list.push(n.parse()?);
            }

            print_lists.push(print_list);
        }
    }

    Ok((relations, print_lists))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {


    }
}
