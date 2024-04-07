
pub struct Take<I> {
    pub(crate) it: I,
    pub sz: usize,
}

impl<I> Take<I> {
}

impl<I: Iterator> Iterator for Take<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.sz {
            0 => None,
            _ => {
                self.sz -= 1;
                self.it.next()
            }
        }
    }
}
