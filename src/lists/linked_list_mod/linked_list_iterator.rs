use std::fmt::Debug;

use crate::lists::nodes_mod::node::Node;

pub struct LinkedListIterator<'a, T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    current: &'a Option<Box<Node<T>>>
}

impl<'a, T> LinkedListIterator<'a, T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    pub fn new(head: &'a Option<Box<Node<T>>>) -> Self {
        Self {
            current: head,
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(node.get_data())
            },
            None => None,
        }
    }
}