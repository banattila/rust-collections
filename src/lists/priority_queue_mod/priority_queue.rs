use std::fmt::Debug;

use crate::lists::{nodes_mod::queue_node::QueueNode, traits::collections::Collections};

#[derive(Debug, Clone)]
pub struct PriorityQueue<T>
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    tail: Option<Box<QueueNode<T>>>,
    min: bool,
}

impl<T> PriorityQueue<T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    pub fn new(min: bool) -> Self {
        Self {
            tail: None,
            min: min,
        }
    }

    pub fn dequeue(&mut self) -> Result<T, String> {
        
        let current = &mut self.tail;

        match current {
            None => return Err("Queue is empty".to_string()),
            Some(node) => {
                let result = node.get_data();
                *current = node.previous.take();
                return Ok(result);
            }
        }
    }
}

impl<T> Collections<T> for PriorityQueue<T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    fn add(&mut self, data: T) {

        let new_node = Box::new(QueueNode::new(data.clone()));
        let mut current = &mut self.tail;

        loop {
            match current {
                None => {
                    *current = Some(new_node);
                    return;
                },
                Some(ref mut node) if self.min && node.get_data() >= data.clone()=> {
                    let mut new_node = QueueNode::new(data.clone());
                    new_node.previous = Some(Box::new(std::mem::replace(node, QueueNode::new(data.clone()))));
                    *current = Some(Box::new(new_node));
                    return;
                    },
                Some(ref mut node) if !self.min && node.get_data() <= data.clone()=> {
                    let mut new_node = QueueNode::new(data.clone());
                    new_node.previous = Some(Box::new(std::mem::replace(node, QueueNode::new(data.clone()))));
                    *current = Some(Box::new(new_node));
                    return;
                    },
                Some(node) => {
                    current = &mut node.previous;
                },
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.tail.is_none()
    }
}