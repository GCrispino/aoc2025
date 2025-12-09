use std::cmp::{max, min, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn merge(&self, other: &Range) -> Option<Range> {
        let start1 = self.start;
        let end1 = self.end;

        let start2 = other.start;
        let end2 = other.end;

        let cmp = self.cmp(other);
        if cmp == Ordering::Equal {
            return Some(self.clone());
        }
        if cmp == Ordering::Greater {
            panic!("first needs to be greater than second");
        }

        if start1 == end1 {
        }

        let ranges_cross = end1 >= start2;
        if !ranges_cross {
            return None;
        }
        let contains_other = end1 >= end2;
        if contains_other {
            return Some(self.clone());
        }

        let start = min(start1, start2);
        let end = max(end1, end2);

        Some(Range { start, end })
    }

    pub fn size(&self) -> u64 {
        self.end - self.start + 1
    }
}

pub fn get_range(s: &String) -> Range {
    let mut spl = s.split("-");
    let start = spl.next().unwrap().parse::<u64>().unwrap();
    let end = spl.next().unwrap().parse::<u64>().unwrap();

    Range { start, end }
}
