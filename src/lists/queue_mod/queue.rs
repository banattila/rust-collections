use std::fmt::Debug;

use crate::lists::traits::collections::Collections;
use crate::lists::nodes_mod::queue_node::QueueNode;

#[derive(Debug, Clone)]
pub struct Queue<T: Clone> {
    tail: Option<Box<QueueNode<T>>>,
    size: usize,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Self {
            tail: None,
            size: 0,
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
                self.size -= 1;
                return Ok(result);
            },
        }
    }
}

impl<T: Clone> Collections<T> for Queue<T> 
{
    fn add(&mut self, data: T) {

        let mut current = &mut self.tail;

        loop {
            match current {
                None => {
                    *current = Some(Box::new(QueueNode::new(data.clone())));
                    self.size += 1;
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

    fn get_size(&self) -> usize {
        self.size
    }
}