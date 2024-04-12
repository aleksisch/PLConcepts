use std::cmp::Ordering;
use std::hash::Hash;
use std::vec::IntoIter;
use crate::group_by::GroupBy;
use crate::my_flatten::Flatten;
use crate::r#where::Where;
use crate::select::Select;
use crate::take::Take;

pub trait LinqExt: Iterator {
    fn select<F, V>(self, cb: F) -> Select<Self, F>
        where
            Self: Sized,
            F: Fn(Self::Item) -> V {
        Select {
            it: self,
            cb
        }
    }

    fn my_flatten(mut self) -> Flatten<Self, Self::Item>
        where
            Self::Item: Iterator,
            Self: Sized {
        let it = self.next();
        Flatten {
            it: self,
            inner_it: it,
        }
    }

    fn my_where<F>(self, cb: F) -> Where<Self, F>
        where
            Self: Sized,
            F: Fn(&Self::Item) -> bool     {
        Where {
            it: self,
            cb
        }
    }

    fn my_take(self, sz: usize) -> Take<Self>
        where
            Self: Sized {
        Take {
            it: self,
            sz
        }
    }

    fn group_by<F, K>(self, cb: F) -> GroupBy<Self, F, K, Self::Item>
        where
            Self: Sized,
            F: Fn(&Self::Item) -> K,
            K: Ord + Hash + Clone  {
        GroupBy::new(self, cb)
    }

    fn order_by<F>(self, cb: F) -> IntoIter<Self::Item>
        where
            Self: Sized,
            F: FnMut(&Self::Item, &Self::Item) -> Ordering {
        let mut data = self.to_list();
        data.sort_by(cb);
        data.into_iter()
    }
    fn to_list(self) -> Vec::<Self::Item>
        where
            Self: Sized {
        self.collect::<Vec<Self::Item>>()
    }
}

impl<I: Iterator> LinqExt for I {
}
