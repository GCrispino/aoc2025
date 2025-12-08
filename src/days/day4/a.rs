use std::error::Error;

use crate::utils;

use crate::days::day4::common;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list("day4/input/real.txt".to_string())?;

    // println!("{:}", is_pos_accessible(&input, 0, 0));
    // println!("{:}", is_pos_accessible(&input, 0, 2));

    // print_input(&input);

    let accessible_positions = common::get_accessible_positions(&input);
    let ans: i32 = accessible_positions
        .iter()
        .map(|(i, j)| {
            if common::is_pos_accessible(&input, *i, *j) {
                1_i32
            } else {
                0_i32
            }
        })
        .sum::<i32>();
    println!("{:}", ans);

    Ok(())
}
