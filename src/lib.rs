// use std::convert::TryInto;
use std::collections::HashMap;
use std::option::Option;
use std::fmt;

/// This represents a creation/annihilation operator
pub enum AC {
    /// Create and Annihilate requires a state/position to act on
    Create(u64),
    Annihilate(u64),
}

/// This represents an operator, acting on Slater determinants
pub struct Operator {
    /// Each operator consists of a sum of terms.
    /// Each term in the operator is an amplitude and a sequence of creation/annihilation operators.
    terms: Vec<(f64, Vec<AC>)>,
}

impl Operator {
    /// Returns an operator with the terms given
    ///
    /// # Arguments
    ///
    /// * `terms` - a Vec containing tuples of amplitudes and Vec<AC>
    pub fn new(terms : Vec<(f64, Vec<AC>)>) -> Operator {
        Operator { terms }
    }
}

/// This represents a single, unique, Slater determinant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Slater {
    /// The unique index of the Slater determinant.
    index: u64,
}

impl fmt::Binary for Slater {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        fmt::Binary::fmt(&self.index, f)
    }

}

impl Slater {
    /// Returns a Slater determinant with the supplied index.
    ///
    /// # Arguments
    ///
    /// * `index` - The unique index of the Slater determinant.
    pub fn new(index: u64) -> Self {
        Self { index }
    }

    /// Returns a Slater determinant corresponding to the supplied states being occupied.
    ///
    /// # Arguments
    ///
    /// * `arr` - The vector containing the states to occupy.
    ///
    /// # Errors
    ///
    /// * If the supplied vector contains duplicates of any index this function returns an Error.
    pub fn from_vec(arr: Vec<u64>) -> Result<Self, &'static str> {
        let mut index: u64 = 0;
        let mut added_indices: Vec<u64> = Vec::new();
        for i in arr.iter() {
            index += 1 << *i;
            match added_indices.binary_search(&i) {
                Ok(_) => return Err("State array contains repeated index!"),
                Err(pos) => added_indices.insert(pos, *i),
            }
        }
        Ok(Self { index })
    }

    /// Returns a Slater determinant corresponding to creating a particle in state j (ignoring phase factors).
    ///
    /// # Arguments
    ///
    /// * `j` - The single particle state in which to create a particle.
    ///
    /// # Errors
    ///
    /// * If the single particle state j is already occupied, this function returns None.
    fn create(self, &j: &u64) -> Option<Self> {
        match self.index & (1 << j) {
            0 => Some(Self {
                index: self.index | (1 << j),
            }),
            _ => None,
        }
    }

    /// Returns a Slater determinant corresponding to annihilating a particle in state j (ignoring phase factors).
    /// # Arguments
    ///
    /// * `j` - The single particle state in which to annihilate a particle.
    ///
    /// # Errors
    ///
    /// * If the single particle state j is already empty, this function returns None.
    fn annihilate(self, &j: &u64) -> Option<Self> {
        match self.index & (1 << j) {
            0 => None,
            _ => Some(Self {
                index: self.index & !(1 << j),
            }),
        }
    }

    /// Returns a Slater determinant corresponding to applying the creation/annihilation operator op to this state (including phase factors).
    /// # Arguments
    ///
    /// * `op` - The creation/annihilation operator to apply to this state.
    ///
    /// # Errors
    ///
    /// * If the reuslt of applying op to this state is None, this function returns None.
    pub fn apply(&self, op: &AC) -> Option<(i32, Self)> {
        match op {
            AC::Create(pos) => {
                if let Some(new_state) = self.create(&pos) {
                    if (!self.index & (1 << pos) - 1).count_ones() % 2 == 0 {
                        return Some((1, new_state));
                    } else {
                        return Some((-1, new_state));
                    };
                } else {
                    return None;
                }
            }
            AC::Annihilate(pos) => {
                if let Some(new_state) = self.annihilate(&pos) {
                    if (self.index & (1 << pos) - 1).count_ones() % 2 == 0 {
                        return Some((1, new_state));
                    } else {
                        return Some((-1, new_state));
                    };
                } else {
                    return None;
                }
            }
        }
    }
}

/// Represents a many body state as a linear combination of Slater determinants.
pub struct State {
    /// A HashMap with the Slater determinants as keys and their amplitudes as values.
    /// Slater determinants with 0 amplitude should not be included in this map.
    amplitudes: HashMap<Slater, f64>,
}

impl State {
    /// Returns a State corresponding to the linear combination of Slater determinants supplied.
    ///
    /// # Arguments
    ///
    /// * `amplitudes` - A vector of tuples of Slater determinants and their corresponding amplitudes.
    pub fn new(states: Vec<(Slater, f64)>) -> State {
        let amplitudes = states.into_iter().collect();
        State { amplitudes }
    }

    /// Returns a State object corresponding to the result of applying the operator `op` to this state.
    ///
    /// # Arguments
    ///
    /// * `op` - The operator object to apply to this state.
    pub fn apply(self, op: Operator) -> State {
        let mut res: HashMap<Slater, f64> = HashMap::new();
        for (fac, mut ac) in op.terms {
            ac.reverse();
            'states: for (state, amp) in &self.amplitudes {
                let mut tmp_states: HashMap<Slater, f64> = HashMap::new();
                tmp_states.insert(*state, *amp);
                for c in &ac {
                    let mut next_states: HashMap<Slater, f64> = HashMap::new();
                    for (s, v) in &tmp_states {
                        if let Some((phase, ns)) = s.apply(c) {
                            let ai = next_states.entry(ns).or_insert(0 as f64);
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
                    *a += fac*v;
                }
            }
        }
        res.retain(|_, v| v.abs() > f64::EPSILON);
        State { amplitudes: res }
    }
}


pub fn run() -> Result<(), &'static str> {
    let n1 = Operator::new(vec![(1.0, vec![
        AC::Create(1),
        AC::Annihilate(1),
    ])]);
    let s = State::new(vec![(Slater::new(7), 0.33), (Slater::new(2), 0.33), (Slater::new(14), 0.33)]);
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
        let state = Slater::from_vec(vec![0, 1, 2]).unwrap();
        assert_eq!(state.index, 7);
    }
    #[test]
    fn test_from_uint() {
        let state = Slater::new(7);
        assert_eq!(state.index, 7);
    }

    #[test]
    fn test_create() {
        let state = Slater::from_vec(Vec::new()).unwrap();
        assert_eq!(state.create(&2).unwrap().index, 4);
    }
    #[test]
    fn test_annihilate() {
        let  state = Slater::from_vec(vec![0, 1]).unwrap();
        assert_eq!(state.annihilate(&0).unwrap().index, 2);
    }

    #[test]
    fn test_new_state() {
        let s = State::new(vec![(Slater::new(7), 0.33), (Slater::new(2), 0.33), (Slater::new(14), 0.33)]);
        let mut check = HashMap::new();
        check.insert(7, 0.33);
        check.insert(2, 0.33);
        check.insert(14, 0.33);
        for (key, val) in &check {
            assert_eq!(val, s.amplitudes.get(&Slater::new(*key)).unwrap());
        }
    }

    #[test]
    fn test_apply_state() {
        let a = Operator::new(vec![(1.0, vec![ AC::Create(0), AC::Annihilate(1)])]);
        let s = State::new(vec![(Slater::new(7), 0.33), (Slater::new(2), 0.33), (Slater::new(14), 0.33)]);
        let ns = s.apply(a);
        let mut check = HashMap::new();
        check.insert(1, 0.33);
        check.insert(13, 0.33);
        for (key, val) in &ns.amplitudes {
            assert_eq!(val, check.get(&key.index).unwrap());
        }
    }
}
