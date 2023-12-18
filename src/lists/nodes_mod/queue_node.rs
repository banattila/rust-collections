use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueNode<T> {
    data: T,
    pub previous: Option<Box<QueueNode<T>>>,
}

impl<T: Clone> QueueNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: data,
            previous: None,
        }
    }

    pub fn has_previous(&self) -> bool {
        self.previous.is_some()
    }

    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    pub fn set_data(&mut self, new_data: &T) {
        self.data = new_data.clone();
    }

    pub fn set_previous(&mut self, prev_node: QueueNode<T>) {
        self.previous = Some(Box::new(prev_node));
    }

    pub fn get_previous(&self) -> &Option<Box<QueueNode<T>>> {
        &self.previous
    }

}