use crate::lists::nodes_mod::node::Node;

pub struct LinkedListIterator<'a, T: Clone> {
    current: &'a Option<Box<Node<T>>>
}

impl<'a, T: Clone> LinkedListIterator<'a, T> {
    pub fn new(head: &'a Option<Box<Node<T>>>) -> Self {
        Self {
            current: head,
        }
    }
}

impl<'a, T: Clone> Iterator for LinkedListIterator<'a, T> {
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