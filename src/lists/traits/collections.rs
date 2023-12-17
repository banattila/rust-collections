use std::fmt::Debug;
pub trait Collections<T>
    where T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord {

    fn is_empty(&self) -> bool;
    fn add(&mut self, data: T);
}