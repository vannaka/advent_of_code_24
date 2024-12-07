use std::{collections::HashMap, error, fs::File, io::{self, BufRead}};

use petgraph::{algo::toposort, prelude::*};
// use petgraph::dot::{Dot, Config};

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

    let (relations, print_lists) = parse_input(&in_file)?;

    let g: GraphMap<u32, (), Directed> = DiGraphMap::from_edges(&relations);

    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    // return Ok(());

    let sorted = toposort(&g, None).expect("Unexpected cycles");

    println!("{:?}", sorted);
    // return Ok(());

    let pg2prior = sorted.iter().enumerate().map(|(i,n)| (*n, i as u32)).collect::<HashMap<u32, u32>>(); // K: page#, V: toposort index

    // Find page#'s in sorted, compare indecies. If always increasing, we're good.
    let mut valid_cnt = 0;
    let mut mid_sum = 0;

    for print_list in &print_lists {

        if validate_print_order(&print_list, &pg2prior) {
            valid_cnt += 1;
            mid_sum += print_list[&print_list.len()/2];
        }
    }

    println!("Cnt: {}, Sum: {}", valid_cnt, mid_sum);

    Ok(())
}


fn validate_print_order(print_list: &Vec<u32>, pg2prior: &HashMap<u32, u32>) -> bool {
    let mut last_idx = 0u32;
    let mut pass = true;

    for pg_num in print_list {
        let idx = match pg2prior.get(pg_num) {
            Some(idx) => *idx + 1, /* start at 1 */
            None => {
                panic!("pg_num ({}) not in pg2prior!", pg_num);
            }
        };

        /* idx should always increase */
        if idx <= last_idx {
            pass = false;
            break;
        }

        last_idx = idx;
    }

    pass
}


fn parse_input(f: &File) -> Result<(Vec<(u32, u32)>,Vec<Vec<u32>>)> {
    let mut relations: Vec<(u32, u32)> = Vec::new();
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
            let tokens: Vec<_> = line.split("|").filter(|k| !k.is_empty()).collect();
            assert_eq!(tokens.len(), 2);

            relations.push((tokens[0].parse()?, tokens[1].parse()?));
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
        let pri_list = vec![97, 75, 47, 61, 53, 29, 13];
        let pg2prior = pri_list.iter().enumerate().map(|(i,n)| (*n, i as u32)).collect::<HashMap<u32, u32>>(); // K: page#, V: toposort index

        let print_list = vec![75,47,61,53,29];
        assert!(validate_print_order(&print_list, &pg2prior));

        let print_list = vec![97,61,53,29,13];
        assert!(validate_print_order(&print_list, &pg2prior));

        let print_list = vec![75,29,13];
        assert!(validate_print_order(&print_list, &pg2prior));

        let print_list = vec![75,97,47,61,53];
        assert!(!validate_print_order(&print_list, &pg2prior));

        let print_list = vec![61,13,29];
        assert!(!validate_print_order(&print_list, &pg2prior));

        let print_list = vec![97,13,75,29,47];
        assert!(!validate_print_order(&print_list, &pg2prior));



    }
}
