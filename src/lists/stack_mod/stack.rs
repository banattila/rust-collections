use std::fmt::Debug;

use crate::lists::traits::collections::Collections;

use super::stack_node::StackNode;

pub struct Stack<T> 
where 
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    head: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T>
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn get_head(&mut self) -> &mut Option<Box<StackNode<T>>> {
        &mut self.head
    }

    pub fn set_head(&mut self, other_node: StackNode<T>) {
        self.head = Some(Box::new(other_node));
    }
}

impl<T> Collections<T> for Stack<T>
where 
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn add(&mut self, data: T) {
        
        let mut new_node = StackNode::new(data);
        
        if self.is_empty() {
            self.head = Some(Box::new(new_node));

        }
        else {
            new_node.set_next(self.head.take());
            self.head = Some(Box::new(new_node));
        }
    }
}