use std::convert::TryInto;


struct Config {
    index: u64,
}

impl Config {
    fn new(arr: Vec<u64>) -> Config{
        let mut index : u64 = 0;
        for i in arr.iter() {
            index += 2_u64.pow((*i).try_into().unwrap());
        }
        Config {index}
    }

    fn set(&mut self, j: u64) -> (){
        self.index = self.index | 2_u64.pow(j.try_into().unwrap());
    }

    fn clear(&mut self, j: u64) -> (){
        self.index = self.index & !2_u64.pow(j.try_into().unwrap());
    }

}

fn main() {
    let state = vec![0, 1, 3];
    let mut c = Config::new(state);
    c.set(4);
    c.clear(1);
    println!("Index of config is {}", c.index);
}
