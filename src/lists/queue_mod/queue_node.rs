use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueNode<T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    data: T,
    next: Option<Box<QueueNode<T>>>,
    previous: Option<Box<QueueNode<T>>>,
}

impl<T> QueueNode<T>
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    pub fn new(data: T) -> Self {
        Self {
            data: data,
            next: None,
            previous: None,
        }
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
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

    pub fn set_next(&mut self, mut next_node: QueueNode<T>) {
        next_node.previous = Some(Box::new(self.clone()));
        self.next = Some(Box::new(next_node));

    }

    pub fn set_prevoius(&mut self, mut prev_node: QueueNode<T>) {
        prev_node.next = Some(Box::new(self.clone()));
        self.previous = Some(Box::new(prev_node));
    }

    pub fn get_next(&mut self) -> &mut Option<Box<QueueNode<T>>> {
        &mut self.next
    }

    pub fn get_previous(&mut self) -> &mut Option<Box<QueueNode<T>>> {
        &mut self.previous
    }
}