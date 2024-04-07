
pub struct Fib {
    x: (usize, usize),
}

impl Fib {
    pub fn new() -> Fib {
        Fib { x: (0, 1) }
    }
}

impl Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x.1, self.x.0 + self.x.1);
        Some(self.x.0)
    }
}