use std::error::Error;

use crate::utils;

use crate::days::day4::common;

fn apply_positions(diagram: &common::Diagram, positions: &[(usize, usize)]) -> common::Diagram {
    let mut new_diagram: common::Diagram = diagram.clone();

    for (i, j) in positions {
        let mut chars: Vec<char> = new_diagram[*i].chars().collect();
        chars[*j] = '.';
        new_diagram[*i] = chars.iter().collect();
    }

    new_diagram
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list("day4/input/real.txt".to_string())?;

    // println!("{:}", is_pos_accessible(&input, 0, 0));
    // println!("{:}", is_pos_accessible(&input, 0, 2));

    // common::print_input(&input);

    let mut diagram = input.clone();
    let mut n_rolls = 0;
    let mut i = 0;
    loop {
        i += 1;
        let accessible_positions = common::get_accessible_positions(&diagram);
        let n_accessible_positions = accessible_positions.len();
        println!(
            "Run number {:}: number of accessible positions: {:}",
            i, n_accessible_positions,
        );

        if n_accessible_positions == 0 {
            break;
        }

        n_rolls += n_accessible_positions;

        let new_diagram = apply_positions(&diagram, &accessible_positions);
        diagram = new_diagram;
    }

    println!("{:}", n_rolls);

    Ok(())
}
