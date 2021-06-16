// use std::convert::TryInto;
use std::collections::HashMap;
use std::option::Option;

pub enum AC {
    Create(u64),
    Annihilate(u64),
}

pub struct Operator {
    pub a: f64,
    pub acs: Vec<AC>,
}

impl Operator {
    pub fn new(a : f64, acs: Vec<AC>) -> Operator {
        Operator { a, acs }
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
                    if (!self.index & (1 << pos) - 1).count_ones() % 2 == 0 {
                        println!("{:b} has an even number of zeros before position {}", self.index, pos);
                        return Some((1, new_state));
                    } else {
                        println!("{:b} has an odd number of zeros before position {}", self.index, pos);
                        return Some((-1, new_state));
                    };
                } else {
                    return None;
                }
            }
            AC::Annihilate(pos) => {
                if let Some(new_state) = self.clear(&pos) {
                    if (self.index & (1 << pos) - 1).count_ones() % 2 == 0 {
                        println!("{:b} has an even number of ones before position {}", self.index, pos);
                        return Some((1, new_state));
                    } else {
                        println!("{:b} has an odd number of ones before position {}", self.index, pos);
                        return Some((-1, new_state));
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
        'states: for (state, amp) in &self.amplitudes {
            let mut tmp_states: HashMap<u64, f64> = HashMap::new();
            tmp_states.insert(*state, *amp);
            for c in &op.acs {
                let mut next_states: HashMap<u64, f64> = HashMap::new();
                for (s, v) in &tmp_states {
                    let conf = Config::from_uint(s).unwrap();
                    if let Some((phase, ns)) = conf.apply(c) {
                        let ai = next_states.entry(ns.index).or_insert(0 as f64);
                        *ai += v * phase as f64;
                    } else {
                        next_states.clear();
                        continue 'states;
                    }
                }
                next_states.retain(|_, amp| amp.abs() > f64::EPSILON);
                tmp_states = next_states;
            }
            for (s, v) in &tmp_states {
                let a = res.entry(*s).or_insert(0 as f64);
                *a += op.a*v;
            }
        }
        res.retain(|_, v| v.abs() > f64::EPSILON);
        State { amplitudes: res }
    }
}

pub fn run() -> Result<(), &'static str> {
    let n1 = Operator::new(1.0, vec![
        AC::Create(1),
        AC::Annihilate(1),
    ]);
    let s = State::new(vec![(7, 0.33), (2, 0.33), (14, 0.33)]);
    println!("Initial state :");
    print!("\t");
    for (key, val) in &s.amplitudes {
        print!(" {:+5}|{:08b}> ", val, key)
    }
    print!("\n");

    let ns = s.apply(n1);
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
        let state = Config::from_vec(Vec::new()).unwrap();
        assert_eq!(state.set(&2).unwrap().index, 4);
    }
    #[test]
    fn test_clear() {
        let  state = Config::from_vec(vec![0, 1]).unwrap();
        assert_eq!(state.clear(&0).unwrap().index, 2);
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
        let a = Operator::new(1.0, vec![ AC::Create(0), AC::Annihilate(1)]);
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
