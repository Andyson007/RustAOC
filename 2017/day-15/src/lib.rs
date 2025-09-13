const MODULUS: u64 = 2147483647;

pub struct Generator {
    last: u64,
    multiplier: u64,
}

impl Generator {
    pub fn new(start: u64, multiplier: u64) -> Self {
        Self {
            last: start,
            multiplier,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.last = (self.last * self.multiplier) % MODULUS;
        Some(self.last)
    }
}
