use std::{collections::BTreeSet, fmt::Display};

type Coord = (i64, i64);

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
}

pub struct FloorMap {
    pub coords: Vec<Coord>,
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
        FloorMap {
            coords: Vec::from(coords),
            bounds: Rect {
                x_min,
                x_max,
                y_min,
                y_max,
            },
        }
    }

}

impl Display for FloorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n_rows = 0i64;
        let mut n_cols = 0i64;

        let set: BTreeSet<&(i64, i64)> = BTreeSet::from_iter(self.coords.iter());
        println!("set: {:?}", set);

        let spacer: i64 = 2;
        for (x, y) in &self.coords {
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
                let val = if set.contains(&(j, i)) { "#" } else { "." };
                // let ending = if j == n_cols - 1 { "" } else { " " };
                write!(f, "{:}", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
