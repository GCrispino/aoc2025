use std::error::Error;

use crate::{
    days::day9::common::{Coord, FloorMap, Rect},
    utils,
};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_number_tuples("day9/input/test.txt".to_string())?;

    println!("building floor map");
    let fl_map = FloorMap::new(&input);

    // println!("{:?}", input);

    println!("finding largest valid rect");
    let mut max_area = 0;
    for (i, coord1) in fl_map.red_coords.iter().enumerate() {
        println!("coord {:?} - {:} out of {:}", coord1, i, fl_map.red_coords.len());
        for (j, coord2) in fl_map.red_coords.iter().enumerate() {
            if coord1.eq(coord2) {
                continue;
            }

            if j % 100 == 0 {
                println!("  coord {:} out of {:}", j, fl_map.red_coords.len());
            }
            let rect = Rect::new_from_two_points(*coord1, *coord2);

            if !fl_map.is_valid_rect(&rect) {
                continue;
            }

            let rect_area = rect.area();
            if rect_area > max_area {
                max_area = rect_area;
            }
            println!(
                "rect of coords {:?} and {:?} is valid. Its area is {:}",
                coord1, coord2, rect_area
            );
        }
    }

    // println!("{:}", fl_map);
    println!("{:}", max_area);

    Ok(())
}
