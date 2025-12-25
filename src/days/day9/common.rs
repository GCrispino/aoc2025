use std::{collections::BTreeSet, fmt::Display};

pub type Coord = (i64, i64);

#[derive(Debug, Clone)]
pub struct Rect {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
}

impl Rect {
    pub fn new_from_two_points(p1: Coord, p2: Coord) -> Self {
        let x_min;
        let x_max;
        let y_min;
        let y_max;

        if p1.0 < p2.0 {
            x_min = p1.0;
            x_max = p2.0;
        } else {
            x_max = p1.0;
            x_min = p2.0;
        }

        if p1.1 < p2.1 {
            y_min = p1.1;
            y_max = p2.1;
        } else {
            y_max = p1.1;
            y_min = p2.1;
        }

        Rect {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn area(&self) -> i64 {
        ((self.x_max - self.x_min) + 1) * ((self.y_max - self.y_min) + 1)
    }

    pub fn expand_coords(&self) -> BTreeSet<Coord> {
        let Rect {
            x_min,
            x_max,
            y_min,
            y_max,
        } = self;

        let mut coords: BTreeSet<Coord> = BTreeSet::new();
        for x in *x_min..*x_max + 1 {
            for y in *y_min..*y_max + 1 {
                coords.insert((x, y));
            }
        }

        coords
    }
}

pub struct FloorMap {
    pub red_coords: Vec<Coord>,
    sorted_red_coords: Vec<Coord>,
    pub bounds: Rect,
}

impl FloorMap {
    pub fn new(coords: &[(i64, i64)]) -> Self {
        let mut x_min = i64::MAX;
        let mut x_max = 0i64;
        let mut y_min = i64::MAX;
        let mut y_max = 0i64;

        for (x, y) in coords {
            if *x > x_max {
                x_max = *x;
            }

            if *x < x_min {
                x_min = *x;
            }

            if *y > y_max {
                y_max = *y;
            }

            if *y < y_min {
                y_min = *y;
            }
        }

        let red_coords = Vec::from(coords);
        let mut sorted_red_coords = red_coords.clone();
        sorted_red_coords.sort();

        FloorMap {
            red_coords,
            sorted_red_coords,
            bounds: Rect {
                x_min,
                x_max,
                y_min,
                y_max,
            },
        }
    }

    pub fn is_coord_inside(&self, coord: &Coord) -> bool {
        let x_max = self.bounds.x_max;

        // TODO: not sure if this is correct, as this only counts points instead of line segments between points
        let mut n_intersections = 0;
        for x in coord.0..x_max + 1 {
            if self.red_coords.contains(&(x, coord.1)) {
                n_intersections += 1;
            }
        }

        n_intersections % 2 != 0
    }

    pub fn is_valid_rect(&self, rect: &Rect) -> bool {
        // TODO: incomplete and incorrect
        let Rect {
            x_min,
            x_max,
            y_min,
            y_max,
        } = rect;

        let top_left = (*x_min, *y_min);
        let top_right = (*x_max, *y_min);
        let bottom_left = (*x_min, *y_min);
        let bottom_right = (*x_max, *y_max);

        if !self.is_coord_inside(&top_left) {
            return false;
        }

        if !self.is_coord_inside(&top_right) {
            return false;
        }

        if !self.is_coord_inside(&bottom_left) {
            return false;
        }

        if !self.is_coord_inside(&bottom_right) {
            return false;
        }

        true
    }
}

impl Display for FloorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n_rows = 0i64;
        let mut n_cols = 0i64;

        let red_set: BTreeSet<&(i64, i64)> = BTreeSet::from_iter(self.red_coords.iter());
        // let green_set: BTreeSet<&(i64, i64)> = BTreeSet::from_iter(self.green_coords.iter());
        println!("red_set: {:?}", red_set);

        let spacer: i64 = 2;
        for (x, y) in &self.red_coords {
            if *x + spacer + 1 > n_cols {
                n_cols = *x + spacer + 1;
            }

            if *y + spacer + 1 > n_rows {
                n_rows = *y + spacer + 1;
            }
        }

        println!("n_rows: {:}, n_cols: {:}", n_rows, n_cols);

        for i in 0..n_rows {
            for j in 0..n_cols {
                // println!("{:?}", (i, j));
                let val = if red_set.contains(&(j, i)) {
                    "#"
                // } else if green_set.contains(&(j, i)) {
                //     "X"
                } else {
                    "."
                };
                // let ending = if j == n_cols - 1 { "" } else { " " };
                write!(f, "{:}", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
