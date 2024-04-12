
pub struct Where<I, F> {
    pub(crate) it: I,
    pub(crate) cb: F,
}

impl<I, F> Where<I, F> {}

impl<I: Iterator, F> Iterator for Where<I, F>
    where
        F: Fn(&I::Item) -> bool {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let cur = self.it.next()?;
            if (&self.cb)(&cur) {
                return Some(cur);
            }
        }
    }
}
