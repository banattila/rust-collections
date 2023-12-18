pub trait Collections<T>{
    fn is_empty(&self) -> bool;
    fn add(&mut self, data: T);
    fn get_size(&self) -> usize;
}