use std::error::Error;

use crate::utils;

fn get_input() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let x = utils::read_from_file_to_string("day6/input/real.txt".to_string())?;

    let spl: Vec<Vec<String>> = x
        .split("\n")
        .map(|x| {
            x.to_string()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    Ok(spl)
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    // println!("{:?}", input);

    let n_rows = input.len();
    let n_cols = input[0].len();
    let operators = input[n_rows - 1].clone();

    let mut vals: Vec<u64> = vec![];
    for j in 0..n_cols {
        let operator = operators[j].clone();
        let operands: Vec<u64> = (0..n_rows - 1)
            .map(|i| input[i][j].parse::<u64>().unwrap())
            .collect();

        // println!("  j = {:}, operands = {:?}", j, operands);
        let res: u64 = match operator.as_str() {
            "*" => operands.iter().product(),
            "+" => operands.iter().sum(),
            s => panic!("invalid operator {:}", s),
        };

        vals.push(res);
    }

    // println!("{:?}", vals);
    let ans: u64 = vals.iter().sum();
    println!("{:}", ans);
    Ok(())
}
