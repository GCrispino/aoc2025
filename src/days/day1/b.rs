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

    println!("pos: {:}, {:}", pos, (pos + (48 as i32)) / 100);
    for inst in instructions.iter() {
        let Instruction { dir, n } = inst;
        let n_div = n / 100;

        println!("n_div: {:}", n_div);
        let mut n_reached_0: i32 = 0;
        match dir {
            Direction::L => {
                let _p = pos - (*n as i32);
                if _p > 0 {
                    pos = _p
                } else if _p < 0 {
                    if pos != 0 {
                        n_reached_0 = 1 + n_div as i32;
                    }
                    pos = 100 + _p;
                } else {
                    pos = 0;
                };
            }
            Direction::R => {
                let _p = pos + (*n as i32);
                pos = _p % 100;
                n_reached_0 = _p / 100;
                // println!("OI");
            }
        };

        if pos == 0 {
            n_reached_0 = 1 + n_div as i32;
            // println!("OI2");
        }
        n_0 += n_reached_0;
        println!("n_reached_0: {:}", n_reached_0);
        println!(
            "inst: {:?}, n reached 0: {:}, n_0: {:}, new pos: {:}",
            inst, n_reached_0, n_0, pos
        );
    }

    println!("{}", n_0);

    Ok(())
}
