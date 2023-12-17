use std::fmt::Debug;

use crate::lists::nodes_mod::node::Node;
use crate::lists::traits::collections::Collections;


#[derive(Clone, Debug)]
pub struct Stack<T> 
where 
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    head: Option<Box<Node<T>>>,
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

    pub fn get_head(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.head
    }

    pub fn set_head(&mut self, other_node: Node<T>) {
        self.head = Some(Box::new(other_node));
    }

    pub fn pop(&mut self) -> Option<T> {

        let head = &mut self.head;

        match head {
            None => return None,
            Some(node) => {
                let result = node.get_data();
                *head = node.get_next().take();
                return Some(result);
            }
        }
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
        
        let mut new_node = Node::new(data);
        
        if self.is_empty() {
            self.head = Some(Box::new(new_node));

        }
        else {
            new_node.next = self.head.take();
            self.head = Some(Box::new(new_node));
        }
    }
}