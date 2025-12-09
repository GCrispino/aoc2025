use std::error::Error;
// use std::io;

use crate::{
    days::day5::common::{get_range, Range},
    utils,
};

fn consolidate_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mut result: Vec<Range> = ranges.clone();
    let mut changed = true;
    while changed {
        // println!("  result ranges: {:?}", result);
        changed = false;
        let mut i = 0;
        loop {
            // println!("    i: {:}, result ranges: {:?}", i, result);
            if i == result.len() - 1 {
                break;
            }

            let cur_range = result[i];
            let next_range = result[i + 1];

            if let Some(res_range) = cur_range.merge(&next_range) {
                // println!(
                //     "      replacing ranges {:?} and {:?} with: {:?}",
                //     cur_range, next_range, res_range
                // );
                result.splice(i..i + 2, [res_range]);

                changed = true;
            } else {
                i += 1;
            }
            // let mut input = String::new();
            // let _ = io::stdin().read_line(&mut input); // Read input into the `input` variable
        }
    }

    result
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list("day5/input/real.txt".to_string())?;

    let mut spl = input.split(|s| s == "");
    let mut ranges: Vec<Range> = spl.next().unwrap().iter().map(get_range).collect();

    ranges.sort();
    // println!("ranges sorted");
    // for r in &ranges {
    //     println!("  {:?}", r);
    // }

    let res_ranges = consolidate_ranges(&ranges);
    // println!("consolidated ranges sorted");
    // for r in &res_ranges {
    //     println!("  {:?}", r);
    // }
    let ans: u64 = res_ranges.iter().map(|r| r.size()).sum();
    println!("{:?}", ans);

    Ok(())
}
