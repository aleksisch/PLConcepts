
type Cb<'a, V> = &'a dyn Fn(V) -> V;

pub struct Select<I, F> {
    pub(crate) it: I,
    pub(crate) cb: F,
}

impl<I, F> Select<I, F> {
    fn new(it: I, cb: F) -> Select<I, F> {
        Select { it, cb }
    }
}

impl<I: Iterator, F> Iterator for Select<I, F>
    where
        F: Fn(I::Item) -> I::Item {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.cb)(self.it.next()?))
    }
}
