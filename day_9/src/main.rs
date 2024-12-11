use std::{error, fs::File, io::{self, BufRead}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day9_example.txt";
const INPUT: &str = "data/day9.txt";

struct Entity {
    occupied: bool,
    id: u32,
    size: u8,
}

impl Entity {
    fn new(occupied: bool, id: u32, size: u8) -> Self {
        Self {
            occupied, id, size
        }
    }
}

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;
    let mut entities= parse_input(&in_file)?;

    print_blocks(&entities);

    /* Sort entities */
    sort_blocks(&mut entities);

    print_blocks(&entities);

    let checksum = calc_checksum(&entities);

    println!("Checksum: {}", checksum);

    Ok(())
}

fn calc_checksum( entities: &Vec<Entity>) -> u64 {
    let mut sum = 0u64;
    let mut block_number = 0u64;

    for entity in entities {
        if !entity.occupied {
            block_number += entity.size as u64;
            continue;
        }

        for _ in 0..entity.size {
            sum += block_number as u64 * entity.id as u64;

            block_number += 1;
        }
    }

    sum
}

fn sort_blocks( entities: &mut Vec<Entity>) {
    let mut start_idx = 0;
    let mut end_idx = entities.len();

    /* Sort entities */
    loop {
        /* Look for next end file block */
        end_idx -= 1;
        while end_idx > 0
          && !entities[end_idx].occupied {

            end_idx -= 1;
        }

        if end_idx == 0 {
            break;
        }

        /* Look for next free block big enough*/
        start_idx = 0;
        while start_idx < end_idx {

            if !entities[start_idx].occupied
            && entities[start_idx].size >= entities[end_idx].size {
                break;
            }

            start_idx += 1;
        }

        if start_idx == end_idx {
            continue;
        }

        let end_sz = entities[end_idx].size;
        let start_sz = entities[start_idx].size;

        /* Swap entities */
        entities.swap(start_idx, end_idx);

        /* insert a new entity to account for the remaining free space */
        if start_sz > end_sz {
            entities.insert(start_idx+1, Entity::new(false, 0, start_sz-end_sz));
            end_idx += 1;
            entities[end_idx].size = end_sz;
        }

        // print_blocks(&entities);
    }
}

fn print_blocks( entities: &Vec<Entity>) {
    // println!("{} Entities", entities.len());
    for block in entities {
        if block.occupied {
            for _ in 0..block.size {
                print!("{}", block.id);
            }
        }
        else {
            for _ in 0..block.size {
                print!(".");
            }
        }
        print!(" ");
    }

    println!("");
}


fn parse_input(f: &File) -> Result<Vec<Entity>> {
    let reader: io::BufReader<&File> = io::BufReader::new(f);
    let mut entities = Vec::new();
    let mut id = 0u32;

    for line in reader.lines() {
        let line = line?;

        for (i, n) in line.chars().map(|ch| ch as u8 - '0' as u8).enumerate() {
            if n == 0 {continue;}

            /* Even are files, odd are empty space */
            let is_file = i % 2 == 0;

            /* Add n sized entity */
            entities.push(Entity::new(is_file, id, n));

            if is_file { id += 1 }
        }
    }

    Ok(entities)
}
