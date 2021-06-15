// use std::convert::TryInto;
use std::collections::HashMap;
use std::option::Option;

pub enum AC {
    Create(u64),
    Annihilate(u64),
}

pub struct Operator {
    pub acs: Vec<(f64, AC)>,
}

impl Operator {
    pub fn new(acs: Vec<(f64, AC)>) -> Operator {
        Operator { acs }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub index: u64,
}

impl Config {
    pub fn new(index: u64) -> Self {
        Self { index }
    }

    pub fn from_vec(arr: Vec<u64>) -> Result<Config, &'static str> {
        let mut index: u64 = 0;
        let mut added_indices: Vec<u64> = Vec::new();
        for i in arr.iter() {
            index += 1 << *i;
            match added_indices.binary_search(&i) {
                Ok(_) => return Err("State array contains repeated index!"),
                Err(pos) => added_indices.insert(pos, *i),
            }
        }
        Ok(Config { index })
    }

    pub fn from_uint(&index: &u64) -> Result<Config, &'static str> {
        Ok(Config::new(index))
    }

    fn set(self, &j: &u64) -> Option<Config> {
        match self.index & (1 << j) {
            0 => Some(Config {
                index: self.index | (1 << j),
            }),
            _ => None,
        }
    }

    fn clear(self, &j: &u64) -> Option<Config> {
        match self.index & (1 << j) {
            0 => None,
            _ => Some(Config {
                index: self.index & !(1 << j),
            }),
        }
    }

    pub fn apply(&self, op: &AC) -> Option<(i32, Config)> {
        match op {
            AC::Create(pos) => {
                if let Some(new_state) = self.set(&pos) {
                    if (self.index & (1 << pos) - 1).count_ones() % 2 != 0 {
                        return Some((-1, new_state));
                    } else {
                        return Some((1, new_state));
                    };
                } else {
                    return None;
                }
            }
            AC::Annihilate(pos) => {
                if let Some(new_state) = self.clear(&pos) {
                    if (self.index & (1 << pos) - 1).count_ones() % 2 != 0 {
                        return Some((-1, new_state));
                    } else {
                        return Some((1, new_state));
                    };
                } else {
                    return None;
                }
            }
        }
    }
}

pub struct State {
    pub amplitudes: HashMap<u64, f64>,
}

impl State {
    pub fn new(states: Vec<(u64, f64)>) -> State {
        let amplitudes = states.into_iter().collect();
        State { amplitudes }
    }

    pub fn apply(self, mut op: Operator) -> State {
        op.acs.reverse();
        let mut res: HashMap<u64, f64> = HashMap::new();
        let mut tmp_states: HashMap<u64, f64> = HashMap::new();
        for (state, amp) in &self.amplitudes {
            tmp_states.insert(*state, *amp);
            for (a, c) in &op.acs {
                let mut next_states: HashMap<u64, f64> = HashMap::new();
                'states: for (s, v) in &tmp_states {
                    let conf = Config::from_uint(s).unwrap();
                    if let Some((phase, ns)) = conf.apply(c) {
                        let ai = next_states.entry(ns.index).or_insert(0 as f64);
                        *ai += a * v * phase as f64;
                    } else {
                        next_states.clear();
                        continue 'states;
                    }
                }
                next_states.retain(|_, amp| amp.abs() > f64::EPSILON);
                tmp_states = next_states;
                println!("{:?}", tmp_states);
            }
            for (s, v) in &tmp_states {
                let a = res.entry(*s).or_insert(0 as f64);
                *a += v;
            }
        }
        res.retain(|_, v| v.abs() > f64::EPSILON);
        State { amplitudes: res }
    }
}

pub fn run() -> Result<(), &'static str> {
    let a = Operator::new(vec![
        (1.0, AC::Create(1)),
        (1.0, AC::Create(0)),
        (1.0, AC::Annihilate(1)),
    ]);
    let s = State::new(vec![(7, 0.33), (2, 0.33), (14, 0.33)]);
    println!("Initial state :");
    print!("\t");
    for (key, val) in &s.amplitudes {
        print!(" {:+5}|{:08b}> ", val, key)
    }
    print!("\n");

    let ns = s.apply(a);
    println!("Final state :");
    print!("\t");
    for (key, val) in &ns.amplitudes {
        print!(" {:+5}|{:08b}> ", val, key)
    }
    print!("\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let state = Config::from_vec(vec![0, 1, 2]).unwrap();
        assert_eq!(state.index, 7);
    }
    #[test]
    fn test_from_uint() {
        let state = Config::from_uint(&7).unwrap();
        assert_eq!(state.index, 7);
    }

    #[test]
    fn test_set() {
        let mut state = Config::from_vec(Vec::new()).unwrap();
        state.set(&2);
        assert_eq!(state.index, 4);
    }
    #[test]
    fn test_clear() {
        let mut state = Config::from_vec(vec![0, 1]).unwrap();
        state.clear(&0);
        assert_eq!(state.index, 2);
    }

    #[test]
    fn test_new_state() {
        let s = State::new(vec![(7, 0.33), (2, 0.33), (14, 0.33)]);
        let mut check = HashMap::new();
        check.insert(7, 0.33);
        check.insert(2, 0.33);
        check.insert(14, 0.33);
        for (key, val) in &check {
            assert_eq!(val, s.amplitudes.get(key).unwrap());
        }
    }

    #[test]
    fn test_apply_state() {
        let a = Operator::new(vec![(1.0, AC::Create(0)), (1.0, AC::Annihilate(1))]);
        let s = State::new(vec![(7, 0.33), (2, 0.33), (14, 0.33)]);
        let ns = s.apply(a);
        let mut check = HashMap::new();
        check.insert(1, 0.33);
        check.insert(13, 0.33);
        for (key, val) in &ns.amplitudes {
            assert_eq!(val, check.get(key).unwrap());
        }
    }
}
