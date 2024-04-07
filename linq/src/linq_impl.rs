use std::hash::Hash;
use crate::group_by::GroupBy;
use crate::r#where::Where;
use crate::select::Select;
use crate::take::Take;

pub trait LinqExt: Iterator {
    fn select<F>(self, cb: F) -> Select<Self, F>
        where
            Self: Sized,
            F: Fn(Self::Item) -> Self::Item     {
        Select {
            it: self,
            cb
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
}

impl<I: Iterator> LinqExt for I {
}
