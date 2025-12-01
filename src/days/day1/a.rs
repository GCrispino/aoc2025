use std::{error::Error, i32};

use crate::utils;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let parsed = utils::read_from_file_to_number_tuples("day1/input/real.txt".to_string())?;

    let mut first_list: Vec<i32> = parsed.clone().into_iter().map(|x| x.0).collect();
    let mut second_list: Vec<i32> = parsed.clone().into_iter().map(|x| x.1).collect();

    first_list.sort();
    second_list.sort();

    println!("{:?}", first_list);
    println!("{:?}", second_list);
    let distances = std::iter::zip(&first_list, &second_list).map(|(x, y)| {
        let diff = x - y;
        diff.abs()
    });

    let s: i32 = distances.into_iter().sum();
    println!("{}", s);
    Ok(())
}
