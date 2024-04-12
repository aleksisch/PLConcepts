
pub struct Select<I, F> {
    pub(crate) it: I,
    pub(crate) cb: F,
}

impl<I, F> Select<I, F> {}

impl<I: Iterator, F, V> Iterator for Select<I, F>
    where
        F: Fn(I::Item) -> V {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.cb)(self.it.next()?))
    }
}
