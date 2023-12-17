use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct StackNode<T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    data: T,
    next: Option<Box<StackNode<T>>>
}

impl<T> StackNode<T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    pub fn new(data: T) -> Self {
        Self {
            data: data,
            next: None,
        }
    }

    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: &T) {
        self.data = data.clone();
    }

    pub fn get_next(&mut self) -> &mut Option<Box<StackNode<T>>> {
        &mut self.next
    }

    pub fn set_next(&mut self, new_next: Option<Box<StackNode<T>>>) {
        self.next = new_next
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
}