// pub struct ToList<I> {
//     pub(crate) it: I,
//     pub(crate) cb: F,
// }
//
// impl<I, F> ToList<I, F> {
//     fn new(it: I, cb: F) -> ToList<I, F> {
//         ToList { it, cb }
//     }
// }
//
// impl<I: Iterator, F> Iterator for ToList<I, F>
//     where
//         F: Fn(I::Item) -> I::Item {
//     type Item = I::Item;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         Some((self.cb)(self.it.next()?))
//     }
// }
