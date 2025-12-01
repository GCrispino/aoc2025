use std::collections::{HashMap, HashSet};
use std::error::Error;

use crate::utils;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let parsed = utils::read_from_file_to_number_tuples("day1/input/real.txt".to_string())?;
    let n = parsed.len();

    let first_list: Vec<i32> = parsed.clone().into_iter().map(|x| x.0).collect();
    let second_list: Vec<i32> = parsed.clone().into_iter().map(|x| x.1).collect();

    println!("{:?}", first_list);
    println!("{:?}", second_list);

    let second_list_set: HashSet<i32> = HashSet::from_iter(second_list.iter().cloned());
    let mut second_list_count: HashMap<i32, i32> = HashMap::new();

    for i in 0..n {
        let item = second_list.get(i).ok_or("no item")?;
        println!("{}", item);

        if second_list_set.contains(item) {
            let cur_count = second_list_count.get(item).unwrap_or(&i32::from(0)).clone();
            second_list_count.insert(*item, cur_count + 1);
        }
    }

    let distances = first_list.into_iter().map(|item| {
        let val = item * second_list_count.get(&item).unwrap_or(&0);

        val
    });

    let s: i32 = distances.into_iter().sum();
    println!("{}", s);
    Ok(())
}
