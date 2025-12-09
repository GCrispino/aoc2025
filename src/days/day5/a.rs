use std::error::Error;

use crate::{days::day5::common, utils};

fn is_ingredient_fresh(ranges: &[common::Range], ingredient: u64) -> bool {
    ranges
        .iter()
        .any(|r| ingredient >= r.start && ingredient <= r.end)
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list("day5/input/real.txt".to_string())?;

    // println!("{:?}", input);
    let mut spl = input.split(|s| s == "");
    let ranges: Vec<common::Range> = spl.next().unwrap().iter().map(common::get_range).collect();
    let ingredient_ids: Vec<u64> = spl
        .next()
        .unwrap()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // println!("ranges: {:?}", ranges);
    // println!("ingredient_ids: {:?}", ingredient_ids);

    let ans = ingredient_ids
        .iter()
        .filter(|ingredient| is_ingredient_fresh(&ranges, **ingredient))
        .count();

    println!("{:?}", ans);

    Ok(())
}
