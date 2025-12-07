use std::error::Error;

use crate::utils;

fn find_largest_joltage(joltages: &Vec<u64>) -> u64 {
    let (i_max, mut max) = joltages
        .iter()
        .enumerate()
        .max_by(
            |(i_x, x), (i_y, y)| {
                if x == y {
                    i_y.cmp(i_x)
                } else {
                    x.cmp(y)
                }
            },
        )
        .expect("should never be empty");

    let second_max: &u64;
    if i_max < joltages.len() - 1 {
        second_max = joltages[i_max + 1..]
            .iter()
            .max()
            .expect("should never be empty");
    } else {
        second_max = max;
        max = joltages[..joltages.len() - 1]
            .iter()
            .max()
            .expect("should never be empty");
    }

    let max_joltage = (max * 10 + second_max).into();
    println!("  joltages: {:?}", joltages);
    println!(
        "  max: {:}, i_max: {:}, second_max: {:}, max_joltage: {:}",
        max, i_max, second_max, max_joltage
    );
    max_joltage
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list("day3/input/real.txt".to_string())?;

    // println!("input: {:?}", joltages);
    let joltages_opt: Option<Vec<Vec<u64>>> = input
        .iter()
        .map(|s_joltages| s_joltages.chars())
        .map(|cs| cs.map(|c| c.to_digit(10).map(|x| x.into())).collect())
        .collect();

    let joltages = joltages_opt.ok_or("option is None")?;

    let largest_joltages: Vec<u64> = joltages.iter().map(|js| find_largest_joltage(js)).collect();
    let ans: u64 = largest_joltages.iter().sum();
    println!("{:}", ans);
    Ok(())
}
