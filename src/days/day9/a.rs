use std::error::Error;

use crate::{
    days::day9::common::{FloorMap, Rect},
    utils,
};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_number_tuples("day9/input/real.txt".to_string())?;

    let fl_map = FloorMap::new(&input);

    // println!("{:?}", input);

    // println!("{:}", fl_map);

    let mut max_area = 0;
    for coord1 in &fl_map.red_coords {
        for coord2 in &fl_map.red_coords {
            let rect = Rect::new_from_two_points(*coord1, *coord2);

            let rect_area = rect.area();
            if rect_area > max_area {
                max_area = rect_area;
            }
        }
    }

    println!("{:}", max_area);

    Ok(())
}
