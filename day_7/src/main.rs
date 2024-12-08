use std::{error, fs::File, io::{self, BufRead}};

// This lets us bubble up all errors to main() regardless of type
type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// const INPUT: &str = "data/day7_example.txt";
const INPUT: &str = "data/day7.txt";

struct Equation {
    result: u64,
    operands: Vec<u64>,
}

enum OpType {
    Addition,
    Multiplication,
    Concatination,
}


fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    // println!("The current directory is {}", cwd.display());

    /* Open Input file */
    let in_file = File::open(cwd.join(INPUT))?;

    let equations = parse_input(&in_file)?;
    let mut total_res = 0;

    for eq in equations {
        if is_eq_valid(&eq) {
            total_res += eq.result;
        }
    }

    println!("Total result: {}", total_res);

    Ok(())
}

fn is_eq_valid(eq: &Equation) -> bool {
    return evaluate(OpType::Addition,       eq.operands[0], &eq.operands[1..], eq.result)
        || evaluate(OpType::Multiplication, eq.operands[0], &eq.operands[1..], eq.result)
        || evaluate(OpType::Concatination,  eq.operands[0], &eq.operands[1..], eq.result);
}

fn evaluate( op: OpType, mut accum: u64, operands: &[u64], desired_res: u64) -> bool {
    match op {
        OpType::Addition => accum += operands[0],
        OpType::Multiplication => accum *= operands[0],
        OpType::Concatination => {
            let mut s1 = accum.to_string();
            let s2 = operands[0].to_string();
            s1.push_str(&s2);
            accum = s1.parse().unwrap();
        }
    }

    // Base case
    if operands.len() == 1 {
        return accum == desired_res;
    }

    return evaluate(OpType::Addition,       accum, &operands[1..], desired_res)
        || evaluate(OpType::Multiplication, accum, &operands[1..], desired_res)
        || evaluate(OpType::Concatination,  accum, &operands[1..], desired_res);
}


fn parse_input(f: &File) -> Result<Vec<Equation>> {
    let mut equations = Vec::new();
    let reader: io::BufReader<&File> = io::BufReader::new(f);

    for line in reader.lines() {
        let line = line?;

        let mut operands = line.split(&[' ', ':']).filter(|c| !c.is_empty()).map(|ch| ch.parse().expect("nan")).collect::<Vec<u64>>();
        let result = operands.remove(0);

        equations.push(Equation{result, operands});
    }

    Ok(equations)
}
