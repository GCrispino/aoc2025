use std::error::Error;

use crate::utils;

fn launch_beam(s0: usize, diagram: Vec<Vec<char>>) -> (Vec<Vec<char>>, u64) {
    let mut result: Vec<Vec<char>> = diagram.clone();
    result[1][s0] = '|';

    let mut n_splits: u64 = 0;

    for i in 1..result.len() - 1 {
        let line = result[i].clone();
        // println!("  i = {:}", i);
        let next_line = &diagram[i + 1];
        for (j, c) in line.iter().enumerate() {
            // println!("   OI {:}, {:}", j, c);
            match c {
                '|' => match next_line[j] {
                    '.' => result[i + 1][j] = '|',
                    '^' => {
                        n_splits += 1;
                        // println!("   OI2 {:}", j);
                        if j > 0 {
                            result[i + 1][j - 1] = '|';
                        }
                        if j + 1 < next_line.len() {
                            result[i + 1][j + 1] = '|';
                        }
                    }
                    _ => {}
                },
                '^' => {}
                _c => {}
            }
        }
    }

    (result, n_splits)
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list("day7/input/real.txt".to_string())?;

    println!("input:");
    utils::print_iter(&input);

    let s0 = input[0].find("S").expect("couldn't find initial state");
    println!("s0: {:}", s0);

    let diagram: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    let (new_diagram, ans) = launch_beam(s0, diagram);
    utils::print_matrix(&new_diagram);

    println!("{:}", ans);
    Ok(())
}
