#[derive(Debug)]
pub struct Myrand {
    seed: u32,
}
impl Myrand {
    pub fn new(seed: u16) -> Myrand {
        Myrand { seed: seed as u32 }
    }
    pub fn set_seed(&mut self, new_seed: u16) {
        self.seed = new_seed as u32;
    }
    pub fn in_lim(&mut self, nr_vals: u16) -> u16 {
        // nr_vals ==3 => returns in range (0,1,2) (number of values!)
        let x = self.rand();
        return x % nr_vals;
    }
    pub fn rand(&mut self) -> u16 {
        let tmp64: u64 = (self.seed as u64 * (1103515245 as u64)).into();
        self.seed = (tmp64 + 12345 as u64) as u32;
        return (self.seed / 2 ^ 16 % 2 ^ 15) as u16;
    }
}
