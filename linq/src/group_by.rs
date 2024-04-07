use std::collections::HashMap;
use std::hash::Hash;

type Cb<'a, V, K> = &'a dyn Fn(V) -> K;

pub struct GroupBy<I, F, K, V> {
    pub(crate) it: I,
    pub(crate) cb: F,
    elems: Option<HashMap<K, Vec<V>>>,
}

impl<I, F, K, V> GroupBy<I, F, K, V> {
    pub(crate) fn new(it: I, cb: F) -> GroupBy<I, F, K, V> {
        GroupBy { it, cb, elems: None }
    }
}

impl<I: Iterator, F, K> Iterator for GroupBy<I, F, K, I::Item>
    where
        F: Fn(&I::Item) -> K,
        K: Ord + Hash + Clone {
    type Item = (K, Vec<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut map_ref = self.elems.as_mut();
        if map_ref.is_none() {
            loop {
                match self.it.next() {
                    None => break,
                    Some(val) => {
                        let k = (self.cb)(&val);
                        map_ref.as_mut().unwrap().entry(k).or_insert(Vec::new()).push(val);
                    }
                }
            }
        }
        if map_ref.as_mut().unwrap().is_empty() {
            None
        } else {
            let k = map_ref.as_ref().unwrap().keys().take(1).next().unwrap().clone();
            map_ref.as_mut().unwrap().remove_entry(&k)
        }
    }
}
