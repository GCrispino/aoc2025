use std::error::Error;

use crate::utils;

#[derive(Debug)]
pub struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn invalid_ids(&self) -> Vec<u64> {
        let mut ids: Vec<u64> = vec!();
        for id in self.start..self.end + 1 {
            if !is_id_valid(id) {
                ids.push(id);
            }
        }
        ids
    }
}

impl TryFrom<String> for IdRange {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let spl: Vec<&str> = value.split("-").collect();

        if spl.len() != 2 {
            return Err("invalid string".into());
        }

        let start_res = spl[0].parse();
        let start: u64 = start_res.map_err(|_| "invalid string")?;

        let end_res = spl[1].parse();
        let end: u64 = end_res.map_err(|_| "invalid string")?;

        Ok(IdRange { start, end })
    }
}


fn is_id_valid(id: u64) -> bool {
    let s: String = id.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return true;
    }

    let half = len / 2;
    let left = &s[..half];
    let right = &s[half..];

    left != right
}


pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string("day2/input/real.txt".to_string())?;

    let spl: Vec<&str> = input.split(",").collect();
    // println!("{:?}", spl);

    let ranges_result: Result<Vec<IdRange>, String> = spl
        .iter()
        .map(|s_range| IdRange::try_from(s_range.to_string()))
        .collect();

    let ranges: Vec<IdRange> = ranges_result?;


    // let invalid_ids: Vec<Vec<u64>> = ranges.iter().map(|r| r.invalid_ids()).collect();
    // println!("{:?}", invalid_ids);
    let ans: u64 = ranges.iter().flat_map(|r| r.invalid_ids()).sum();


    // println!("{:?}", is_id_valid(1212));
    println!("{:?}", ans);
    Ok(())
}
