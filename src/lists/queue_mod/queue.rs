use std::fmt::Debug;

use crate::lists::traits::collections::Collections;

use super::queue_node::QueueNode;

#[derive(Debug, Clone)]
pub struct Queue<T>
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    tail: Option<Box<QueueNode<T>>>,
}

impl<T> Queue<T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    pub fn new() -> Self {
        Self {
            tail: None,
        }
    }

    pub fn get_tail(&self) ->&Option<Box<QueueNode<T>>> {
        &self.tail
    }

    pub fn dequeue(&mut self) -> Result<T, String> {

        let current = &mut self.tail;
        match current {
            None => return Err("Queue is empty".to_string()),
            Some(node) => {
                let result = node.get_data();
                *current = node.previous.take();
                return Ok(result);
            },
        }
    }
}

impl<T> Collections<T> for Queue<T>
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    fn add(&mut self, data: T) {

        let mut current = &mut self.tail;

        loop {
            match current {
                None => {
                    *current = Some(Box::new(QueueNode::new(data.clone())));
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