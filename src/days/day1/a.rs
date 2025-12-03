use std::{error::Error, i32};

use crate::utils;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    n: u32,
}

fn get_instructions() -> Result<Vec<Instruction>, Box<dyn Error>> {
    let instructions = utils::read_from_file_to_string_list("day1/input/real.txt".to_string())?;

    let instructions: Vec<Instruction> = instructions
        .iter()
        .map(|s| {
            let (dir_s, n_s) = s.split_at(1);
            let dir = match dir_s {
                "L" => Direction::L,
                "R" => Direction::R,
                d => panic!("unsupported direction string {:}", d),
            };

            let n: u32 = n_s.parse::<u32>().unwrap();
            Instruction { dir, n }
        })
        .collect();

    Ok(instructions)
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let instructions = get_instructions()?;

    let mut n_0 = 0;
    let mut pos: i32 = 50;

    for inst in instructions.iter() {
        let Instruction { dir, n } = inst;
        let n_mod = n % 100;

        pos = match dir {
            Direction::L => {
                let _p = pos - (n_mod as i32);
                if _p > 0 {
                    _p
                } else if _p < 0 {
                    100 + _p
                } else {
                    0
                }
            }
            Direction::R => (pos + (n_mod as i32)) % 100,
        };

        // println!("inst: {:?}, new pos: {:}", inst, pos);
        if pos == 0 {
            n_0 += 1;
        }
    }

    println!("{}", n_0);

    Ok(())
}
