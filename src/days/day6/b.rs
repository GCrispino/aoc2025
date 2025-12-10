use std::{error::Error, iter::zip};

use crate::utils;

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    let x = utils::read_from_file_to_string_no_trim("day6/input/real.txt".to_string())?;

    let spl: Vec<String> = x.split("\n").map(|x| x.to_string()).collect();

    Ok(spl)
}

fn transform_input(transposed_input: Vec<Vec<char>>) -> Vec<Vec<u64>> {
    // println!("transposed_input: {:?}", transposed_input);
    let joined: Vec<String> = transposed_input
        .iter()
        .map(|x| x.iter().collect::<String>().trim().to_string())
        .collect();

    // println!("joined: {:?}", joined);
    let mut numbers_split: Vec<Vec<u64>> = vec![];
    let mut current_numbers: Vec<u64> = vec![];
    for (i, s) in joined.iter().enumerate() {
        if s == "" {
            numbers_split.push(current_numbers);
            current_numbers = vec![];
            continue;
        }

        // println!("i = {:}, s = {:}", i, s);
        current_numbers.push(s.parse().unwrap());
        if i == joined.len() - 1 {
            numbers_split.push(current_numbers);
            current_numbers = vec![];
        }
    }

    numbers_split
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;

    let operands_char_lines: Vec<Vec<char>> = input[0..input.len() - 2]
        .iter()
        .map(|x| x.chars().collect())
        .collect();

    let operators_line = &input[input.len() - 2];
    // println!("\n{:?}", &operands_char_lines);
    // println!("{:?}", operators_line);
    let transposed_operands_char_lines = utils::transpose_matrix(&operands_char_lines);
    // utils::print_matrix(&transposed_operands_char_lines);
    // println!("\n{:?}", &transposed_operands_char_lines);

    let transformed_input = transform_input(transposed_operands_char_lines);
    // println!("{:?}", transformed_input);

    let mut vals: Vec<u64> = vec![];
    for (operands, operator) in zip(transformed_input, operators_line.split_whitespace()) {
        let res: u64 = match operator {
            "*" => operands.iter().product(),
            "+" => operands.iter().sum(),
            s => panic!("invalid operator {:}", s),
        };
        vals.push(res);
    }

    let ans: u64 = vals.iter().sum();
    println!("{:}", ans);
    Ok(())
}
