pub trait List<T> {

        fn remove(&mut self, data: T) -> Result<String, String>;
        fn get(&self, index: usize) -> Result<T, String>;
        fn contains(&self, data: T) -> bool;
}