use std::fmt::Debug;

pub trait List<T>
    where T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord {

        fn remove(&mut self, data: T) -> Result<String, String>;
        fn get(&self, index: usize) -> Result<T, String>;
}