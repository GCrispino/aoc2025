use std::error::Error;

use crate::{days::day10::common::Problem, utils};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string("day10/input/real.txt".to_string())?;

    let lines: Vec<&str> = input.split("\n").collect();

    let mut i = 0;
    let res: u64 = lines
        .iter()
        .map(|line| Problem::try_from(*line))
        .map(|p| {
            let problem = p.unwrap();


            println!("solving problem {:}/{:}", i, lines.len());
            i += 1;

            let sol = problem.solve()[&problem.initial_state.to_string()];
            // sol[&problem.initial_state.to_string()]
            sol
        })
        .sum();


    println!("res: {:}", res);

    Ok(())
}
