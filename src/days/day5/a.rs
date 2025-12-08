use std::error::Error;

use crate::utils;

fn get_range(s: &String) -> (u64, u64) {
    let mut spl = s.split("-");
    let start = spl.next().unwrap().parse::<u64>().unwrap();
    let end = spl.next().unwrap().parse::<u64>().unwrap();

    (start, end)
}

fn is_ingredient_fresh(ranges: &[(u64, u64)], ingredient: u64) -> bool {
    ranges
        .iter()
        .any(|(start, end)| ingredient >= *start && ingredient <= *end)
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list("day5/input/real.txt".to_string())?;

    // println!("{:?}", input);
    let mut spl = input.split(|s| s == "");
    let ranges: Vec<(u64, u64)> = spl.next().unwrap().iter().map(get_range).collect();
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
