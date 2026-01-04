use core::panic;
use std::{
    collections::{HashMap, HashSet, LinkedList},
    error::Error,
};

use crate::utils;

#[derive(Debug)]
struct Problem {
    links: HashMap<String, HashSet<String>>,
}

impl Problem {
    fn get_n_paths_from_state(
        &self,
        state: String,
        visited: &mut Option<HashMap<String, u64>>,
    ) -> u64 {
        if state == "out" {
            return 1;
        }

        let m = HashMap::new();
        if visited.is_none() {
            *visited = Some(m);
        }


        let mut n_paths = 0;
        let empty_set = HashSet::new();
        let neighbors: &HashSet<String> = self.links.get(&state).unwrap_or(&empty_set);

        for neighbor in neighbors {
            let neighbor_weight = match visited.as_mut().unwrap().get(neighbor) {
                Some(w) => *w,
                None => self.get_n_paths_from_state(neighbor.clone(), visited),
            };
            n_paths += neighbor_weight;
        }
        // println!("number of paths from {:}: {:}", state, n_paths);
        n_paths
    }

    pub fn get_n_paths(&self) -> u64 {
        let mut map = None;
        self.get_n_paths_from_state("you".to_string(), &mut map)
    }
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input = utils::read_from_file_to_string_list_no_split_whitespace(
        "day11/input/real.txt".to_string(),
    )?;

    let x = input.iter().map(|line| {
        let spl: Vec<String> = line.split(":").map(|s| s.to_string()).collect();
        if spl.len() != 2 {
            panic!("wrong string format");
        }

        let (src, target_str) = (spl[0].clone(), spl[1].clone());
        let targets: HashSet<String> = target_str
            .trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        // println!("OI: {:?}", spl);

        (src, targets)
    });

    let m: HashMap<String, HashSet<String>> = HashMap::from_iter(x);
    let p = Problem { links: m };

    let ans = p.get_n_paths();

    println!("{:}", ans);

    Ok(())
}
