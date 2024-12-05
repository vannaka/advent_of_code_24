use std::{collections::HashMap, error, fs::File, io::{self, BufRead}, ops::Index};

use petgraph::{algo::toposort, prelude::*, visit::Topo};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const INPUT: &str = "data/day5_1_example.txt";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    let relations = parse_input(&in_file)?;

    // let mut pages: HashMap<u32, NodeIndex> = HashMap::new(); // map page #s to index within graph
    let mut g: Graph<u32, ()> = DiGraph::new();

    g = DiGraph::from_edges(&relations);

    // for pair in &relations {
    //     for page in pair {

    //         if !pages.contains_key(page) {
    //             let idx = g.add_node(*page);
    //             pages.insert(*page, idx);
    //         }
    //     }

    //     g.add_edge(pages[&pair[0]], pages[&pair[1]], ());
    //     // Could add duplicate edge. Use `update_edge()` instead to avoid this.
    // }

    let sorted = toposort(&g, None).expect("Unexpected cycles");
    let sorted_pages = sorted.iter().map(|n| g[*n]).collect::<Vec<u32>>();

    // Find page#'s in sorted, compare indecies. If always increasing, we're good.



    Ok(())
}


fn parse_input(f: &File) -> Result<Vec<(u32, u32)>> {
    let mut input: Vec<(u32, u32)> = Vec::new();
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<_> = line.split_whitespace().filter(|k| !k.is_empty()).collect();
        assert_eq!(tokens.len(), 2);

        input.push((tokens[0].parse()?, tokens[1].parse()?));
    }

    Ok(input)
}
