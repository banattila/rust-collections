use std::usize;

use crate::lists::traits::{collections::Collections, list::List};

#[derive(Debug)]
pub struct ArrayList<T: Clone + Default> {
    capacity: usize,
    size: usize,
    array: Vec<T>,
}

impl<T: Clone + Default> ArrayList<T> {
    pub fn new() -> Self {
        Self {
            capacity: 20,
            size: 0,
            array: Vec::with_capacity(20),
        }
    }
}

impl<T: Clone + Default> Collections<T> for ArrayList<T>{
    fn add(&mut self, data: T) {
        if self.size + 1 < self.capacity {
            self.array.push(data);
            self.size += 1;
            return;
        }

        self.capacity *= 2;
        let mut new_array = Vec::<T>::with_capacity(self.capacity);
        new_array.extend_from_slice(&self.array);
        self.array = new_array;
        self.size += 1;
    }

    fn get_size(&self) -> usize {
        self.size.clone()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: Clone + Default> List<T> for ArrayList<T>  {
    fn contains(&self, data: T) -> bool {
        todo!()
    }

    fn get(&self, index: usize) -> Result<T, String> {
        todo!()
    }

    fn remove(&mut self, data: T) -> Result<String, String> {
        todo!()
    }
}