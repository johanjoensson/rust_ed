// use std::convert::TryInto;
use std::option::Option;
use std::collections::HashMap;

pub enum AC {
    Create(u64),
    Annihilate(u64),
}

pub struct Operator {
    pub acs: Vec<(f64, AC)>,
}

impl Operator{
    pub fn new(acs: Vec<(f64, AC)>) -> Operator{
        Operator {acs}
    }
}

pub struct Config {
    pub index: u64,
}

impl Config {
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
        Ok (Config {index})
    }

    pub fn set(&mut self, &j: &u64) -> Option<()> {
        match self.index & (1 << j) {
            0 => Some(self.index = self.index | (1 << j)),
            _ => None,
        }
    }

    pub fn clear(&mut self, &j: &u64) -> Option<()> {
        match self.index & (1 << j) {
            0 => None,
            _ => Some(self.index = self.index & !(1 << j)),
        }
    }
}

pub struct State {
    pub amplitudes: HashMap<u64, f64>,
}

impl State {
    pub fn new(states: Vec<(u64, f64)>) -> State {
        let amplitudes = states.into_iter().collect();
        State {amplitudes}
    }

    pub fn apply(self, mut op: Operator) -> State {
        let mut new_amps: HashMap<u64, f64> = HashMap::new();
        op.acs.reverse();
        'states: for (state, amp) in &self.amplitudes{
            let mut ci = Config::from_uint(state).unwrap();
            'ops: for (a, c) in &op.acs {
                let mut phase = *a;
                let new_state =
                if let AC::Create(pos) = c {
                    match ci.set(&pos){
                        Some(_) => {},
                        None => continue 'states
                    }
                    if (state & (1 << pos) - 1 ).count_ones() % 2 != 0 {
                        phase = -phase;
                    }
                    ci
                }else if let AC::Annihilate(pos) = c {
                    match ci.clear(&pos){
                        Some(_) => {},
                        None => continue 'states
                    };
                    if (!state & (1 << pos) - 1 ).count_ones() % 2 != 0{
                        phase = -phase;
                    }
                    ci
                } else {
                    continue 'ops;
                };

                let a = new_amps.entry(new_state.index).or_insert(0 as f64);
                *a += amp*phase;
            }
        }
        new_amps.retain(|_, amp| amp.abs() > 1e-12);
        State { amplitudes : new_amps}
    }
}

pub fn run() -> Result<(), &'static str> {
    let a = Operator::new(vec![(1.0, AC::Create(0)), (1.0, AC::Annihilate(1))]);
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
    fn test_new_state(){
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
    fn test_apply_state(){
        let a = Operator::new(vec![(1.0, AC::Create(0)), (1.0, AC::Annihilate(1))]);
        let s = State::new(vec![(7, 0.33), (2, 0.33), (14, 0.33)]);
        let ns = s.apply(a);
        let mut check = HashMap::new();
        check.insert(1, 0.33);
        check.insert(13, 0.33);
        for (key, val) in &ns.amplitudes{
            assert_eq!(val, check.get(key).unwrap());
        }

    }

}
