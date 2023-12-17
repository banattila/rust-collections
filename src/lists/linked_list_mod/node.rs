use std::fmt::Debug;

#[derive(Debug, Clone)]

pub struct Node<T> 
    where T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord {
        data: T,
        pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
    where T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord {
    
    pub fn new(data: T) -> Self {
        Self {
            data: data,
            next: None,
        }
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: &T) {
        self.data = data.clone();
    }

    pub fn get_next(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.next
    }

    pub fn set_next(&mut self, other_node: Node<T>) {
        self.next = Some(Box::new(other_node));
    }
}
