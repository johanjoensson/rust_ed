use std::convert::TryInto;
use std::error::Error;
use std::option::Option;
use std::process;

pub struct Config {
    pub index: u64,
}

impl Config {
    pub fn new(arr: Vec<u64>) -> Result<Config, &'static str> {
        let mut index: u64 = 0;
        let mut added_indices: Vec<u64> = Vec::new();
        for i in arr.iter() {
            index += 2_u64.pow((*i).try_into().unwrap());
            match added_indices.binary_search(&i) {
                Ok(_) => return Err("State array contains repeated index!"),
                Err(pos) => added_indices.insert(pos, *i),
            }
        }
        Ok(Config { index })
    }

    pub fn set(&mut self, j: u64) -> Option<()> {
        match self.index & 2_u64.pow(j.try_into().unwrap()) {
            0 => Some(self.index = self.index | 2_u64.pow(j.try_into().unwrap())),
            _ => None,
        }
    }

    pub fn clear(&mut self, j: u64) -> Option<()> {
        match self.index & 2_u64.pow(j.try_into().unwrap()) {
            0 => None,
            _ => Some(self.index = self.index & !2_u64.pow(j.try_into().unwrap())),
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let state = vec![0, 1, 3];
    let mut c = Config::new(state).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    match c.set(2) {
        None => {
            println!("No state!");
            return Ok(());
        }
        _ => {}
    }
    match c.clear(1) {
        None => {
            println!("No state!");
            return Ok(());
        }
        _ => {}
    }
    println!("Index of config is {}", c.index);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
