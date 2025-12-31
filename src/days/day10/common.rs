use regex::Regex;
use std::{
    collections::HashMap,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Button {
    wiring: Vec<u64>,
}

impl Hash for Button {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wiring.hash(state);
    }
}

impl TryFrom<String> for Button {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.len();
        if length <= 2 {
            return Err("length of string should be higher than 2".into());
        }

        let chars = value.chars();
        if &value[0..1] != "(" || &value[length - 1..length] != ")" {
            return Err("invalid string format".into());
        }

        let n_chars = length - 2;

        let wiring: Result<Vec<u64>, _> = chars
            .skip(1)
            .take(n_chars)
            .filter(|c| *c != ',')
            .map(|c| c.to_string().parse::<u64>())
            .collect();

        wiring
            .map(|w| Button { wiring: w })
            .map_err(|_| "invalid string input".to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub n_lights: u64,
    pub lights: Vec<bool>,
}

impl State {
    pub fn apply_button(&self, btn: &Button) -> State {
        let State { n_lights, lights } = self;
        let mut new_state = State {
            n_lights: *n_lights,
            lights: lights.clone(),
        };

        for _i in btn.wiring.clone() {
            let i = _i as usize;
            new_state.lights[i] = !lights[i];
        }

        new_state
    }

    pub fn to_string(&self) -> String {
        let inner_str: String = self
            .lights
            .iter()
            .map(|b| if *b { "#" } else { "." })
            .collect();
        format!("[{:}]", inner_str)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.to_string())
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let state_str = self.to_string();
        state_str.hash(state);
    }
}

impl TryFrom<String> for State {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.len();
        if length <= 2 {
            return Err("length of string should be higher than 2".into());
        }

        let chars = value.chars();
        if &value[0..1] != "[" || &value[length - 1..length] != "]" {
            return Err("invalid string format".into());
        }

        let n_lights = length - 2;
        let mut lights = vec![false; n_lights];

        for (i, c) in chars.skip(1).enumerate().take(n_lights) {
            if c != '.' && c != '#' {
                return Err("invalid string format".into());
            }

            lights[i] = c == '#';
        }

        // println!("n_lights: {:}, {:}", n_lights, value);
        Ok(State {
            n_lights: n_lights as u64,
            lights: lights,
        })
    }
}

#[derive(Debug)]
pub struct Problem {
    pub goal_state: State,
    pub initial_state: State,
    pub transitions: HashMap<State, HashMap<Button, State>>,
}

impl Problem {
    pub fn new(goal_state: State, buttons: &[Button]) -> Self {
        let mut transitions: HashMap<State, HashMap<Button, State>> = HashMap::new();

        let n_lights = goal_state.n_lights;
        for state in get_all_states(n_lights).unwrap() {
            let state_transitions_iter = buttons
                .iter()
                .map(|btn| (btn.clone(), state.apply_button(btn)));
            let inner_map: HashMap<Button, State> = HashMap::from_iter(state_transitions_iter);
            transitions.insert(state, inner_map);
        }

        let initial_state = State {
            n_lights,
            lights: vec![false; n_lights as usize],
        };

        Problem {
            initial_state,
            goal_state,
            transitions,
        }
    }

    /// bellman-ford algorithm for solving the inner planning problem
    pub fn solve(&self) -> HashMap<String, u64> {
        let transitions = &self.transitions;
        let n_states = transitions.len();
        let states = transitions.keys();

        let mut sol: HashMap<String, u64> =
            HashMap::from_iter(states.map(|s| (s.to_string(), 0u64)));
        for _ in 0..n_states {
            for (s, b_map) in transitions {
                if &self.goal_state == s {
                    continue;
                }

                let s_str = s.to_string();
                let min_next_cost = b_map.values().map(|next_s| sol[&next_s.to_string()]).min();
                if min_next_cost.is_some() {
                    sol.insert(s_str.clone(), 1 + min_next_cost.unwrap());
                }
            }
        }

        sol
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let transitions = &self.transitions;
        let n_states = transitions.len();
        write!(
            f,
            "Problem with {:} states, initial state: {:}\n",
            n_states, self.initial_state
        )?;
        for (s, b_map) in &self.transitions {
            let is_goal = &self.goal_state == s;
            write!(f, "  state: {:}{:}\n", s, if is_goal { "[G]" } else { "" })?;
            write!(f, "    transitions: \n")?;
            for (btn, _s) in b_map {
                let _is_goal = &self.goal_state == _s;
                write!(
                    f,
                    "      {:?} -> {:}{:}\n",
                    btn.wiring,
                    _s,
                    if _is_goal { "[G]" } else { "" }
                )?;
            }
        }
        Ok(())
        // write!(f, "{:}", val)?;
    }
}

impl TryFrom<&str> for Problem {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        println!("loading problem");
        let re = Regex::new(r"(\[[\.|#]+\]) (.+) (\{(?:\d+,)*\d+\})").unwrap();

        let mut captures = re.captures_iter(value);

        captures
            .next()
            .ok_or("error capturing".to_string())
            .and_then(|c| {
                let (_, [state_str, buttons_str, _joltages_str]) = c.extract();

                let goal_state = State::try_from(state_str.to_owned()).unwrap();

                let buttons_res: Result<Vec<Button>, String> = buttons_str
                    .split(" ")
                    .map(|s| Button::try_from(s.to_string()))
                    .collect();

                let res: Result<Problem, String> =
                    buttons_res.map(|buttons| Problem::new(goal_state, buttons.as_slice()));

                res
            })
    }
}

fn get_all_states(n_lights: u64) -> Result<Vec<State>, String> {
    // println!("n_lights: {:}, {:}", n_lights, 2 ^ n_lights);
    (0..(2u64.pow(n_lights as u32)))
        .map(|n| {
            let b_string = format!("{n:b}");
            // println!("n: {:}, b_string: {:}", n, b_string);
            let remaining_leading_zeros = (n_lights as usize) - b_string.len();
            let b_string_final = "0".repeat(remaining_leading_zeros) + b_string.as_str();

            let state_string: String = format!(
                "[{:}]",
                b_string_final
                    .chars()
                    .map(|s| if s == '0' { '.' } else { '#' })
                    .collect::<String>()
            );

            // println!(
            //     "n: {:}, b_string: {:}, remaining zeros: {:}, final: {:}, state string: {:}",
            //     n, b_string, remaining_leading_zeros, b_string_final, state_string
            // );
            State::try_from(state_string)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*; // Import all the names from the outer module

    #[test]
    fn test_apply_button() {
        let state = State {
            n_lights: 4,
            lights: vec![false, true, true, false],
        };
        let btn = Button { wiring: vec![1, 3] };

        let expected = State {
            n_lights: 4,
            lights: vec![false, false, true, true],
        };

        let actual = state.apply_button(&btn);

        assert_eq!(actual, expected);
    }
}
