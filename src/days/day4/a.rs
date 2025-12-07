use std::error::Error;

use crate::utils;

type Diagram = Vec<String>;

fn get_neighbor_positions(map: &Diagram, i: usize, j: usize) -> Vec<(usize, usize)> {
    let n_rows = map.len();
    let n_columns = map.get(0).unwrap().len();

    let mut positions = vec![];

    if i > 0 {
        positions.push((i - 1, j));

        if j > 0 {
            positions.push((i - 1, j - 1));
        }
        if j < n_columns - 1 {
            positions.push((i - 1, j + 1));
        }
    }

    if i < n_rows - 1 {
        positions.push((i + 1, j));

        if j > 0 {
            positions.push((i + 1, j - 1));
        }
        if j < n_columns - 1 {
            positions.push((i + 1, j + 1));
        }
    }

    if j > 0 {
        positions.push((i, j - 1));
    }

    if j < n_columns - 1 {
        positions.push((i, j + 1));
    }

    positions
}

fn is_pos_accessible(diagram: &Diagram, i: usize, j: usize) -> bool {
    if &diagram.as_slice()[i][j..j + 1] != "@" {
        return false;
    }

    let n_paper_neighbors: u32 = get_neighbor_positions(&diagram, i, j)
        .iter()
        .map(|(_i, _j)| {
            if &diagram.as_slice()[*_i][*_j..*_j + 1] == "@" {
                // println!("    neighbor ({:}, {:})", _i, _j);
                1
            } else {
                0
            }
        })
        .sum();

    // println!("n_paper_neighbors: {:}", n_paper_neighbors);

    return n_paper_neighbors < 4;
}

fn print_input(diagram: &Diagram) {
    for i in 0..diagram.len() {
        println!("");
        for j in 0..diagram[i].len() {
            let c = &diagram[i][j..j + 1];
            if c.to_string() != "@" {
                print!("{:}", c);
            } else {
                let is_accessible = is_pos_accessible(diagram, i, j);
                let c_print = if is_accessible { "x" } else { &c.to_string() };
                print!("{:}", c_print);
            }
        }
    }
    println!("");
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list("day4/input/real.txt".to_string())?;

    // println!("{:}", is_pos_accessible(&input, 0, 0));
    // println!("{:}", is_pos_accessible(&input, 0, 2));

    // print_input(&input);

    let ans: i32 = input
        .iter()
        .enumerate()
        .map(|(i, s_row)| {
            s_row
                .chars()
                .enumerate()
                .map(|(j, _)| {
                    if is_pos_accessible(&input, i, j) {
                        1_i32
                    } else {
                        0_i32
                    }
                })
                .sum::<i32>()
        })
        .sum();
    println!("{:}", ans);

    Ok(())
}
