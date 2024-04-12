// use std::cmp::Ordering;
// use std::collections::HashMap;
// use std::hash::Hash;
// use crate::linq_impl::LinqExt;
//
// type Cb<'a, V, K> = &'a dyn Fn(V) -> Ordering;
//
// pub struct OrderBy<K> {
//     pub(crate) data: Vec<K>,
//     idx: usize
// }
//
// impl<I: Iterator, F> OrderBy<I::Item> {
//     pub(crate) fn new(it: I, cb: F) -> OrderBy<I::Item> {
//         let mut data = it.to_list();
//         data.sort_by(cb);
//         let ire = data.into_iter();
//         OrderBy { data, idx: 0 }
//     }
// }
//
// impl<K> Iterator for OrderBy<K> {
//     type Item = K;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.idx += 1;
//         self.data[]
//     }
// }
