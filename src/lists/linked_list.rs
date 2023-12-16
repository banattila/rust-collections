use std::fmt::Debug;

use super::{node::Node, collections::Collections, list::List};

#[derive(Debug, Clone)]
pub struct LinkedList<T> 
    where T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord {

        head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T>
    where T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord {

    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn get_head(&self) -> Option<Box<Node<T>>> {
        self.head.clone()
    }

    pub fn set_head(&mut self, other_node: Node<T>) {
        self.head = Some(Box::new(other_node));
    }

    pub fn append(&mut self, data: T) {
    
        let mut current = &mut self.head;

        loop {
            match current {
                None => {
                    let new_node = Some(Box::new(Node::new(data.clone())));
                    *current = new_node;
                    return;
                },
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }

    pub fn iter(&mut self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: &self.head
        }
    }

    pub fn create_from_vec(vector: &Vec<T>) -> Self {
        let mut list = LinkedList::<T>::new();

        for item in vector.into_iter() {
            list.append(item.clone());
        }
        list
    }

    pub fn into_vec(&mut self) -> Vec<T> {
        let mut vec: Vec<T> = vec![];

        let mut current = &mut self.head;

        loop {
            match current {
                None => break,
                Some(node) => {
                    vec.push(node.get_data());
                    current = &mut node.next;
                }
            }
        }
        vec
    }

    pub fn create_from_list(other_list: &LinkedList<T>) -> Self {
        Self {
            head: other_list.get_head()
        }
    }

}

impl<T> Collections<T> for LinkedList<T>
    where T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord {
        
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn add(&mut self, data: T) {

        let mut new_node = Node::new(data);
        
        if let Some(head) = self.get_head() {
            
            new_node.set_next(*head);
            self.set_head(new_node);
        }
        else {
            self.set_head(new_node);
        }
    }

    fn contains(&self, data: T) -> bool {
        
        let mut current = &mut self.get_head();

        loop {
            match current {
                None => return false,
                Some(node) if node.get_data() == data => {
                    return true;
                },
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }
}

impl<T> List<T> for LinkedList<T> 
    where T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord {
    
    fn remove(&mut self, data: T) -> Result<String, String> {
        
        if self.is_empty() {
            return Err("List is empty".to_string());
        }

        let mut current = &mut self.head;

        loop {
            match current {
                None => return Err("Item not found".to_string()),
                Some(ref mut node) if node.get_data() == data => {
                    *current = node.next.take();
                    return Ok("Item removed".to_string());
                },
                Some(node) => {
                    current = node.get_next();
                },
            }
        }
    }

    fn get(&self, index: usize) -> Result<T, String> {

        let mut counter = 0;

        let mut current = &mut self.get_head();

        loop {
            match current {
                None => return Err("Index is too big".to_string()),
                Some(node) if counter == index => {
                    return Ok(node.get_data());
                },
                Some(node) => {
                    current = node.get_next();
                    counter += 1;
                }
            }
        }
    }
}

pub struct LinkedListIterator<'a, T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    current: &'a Option<Box<Node<T>>>
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

impl<T> FromIterator<T> for LinkedList<T> 
where
T: Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
{
    fn from_iter<I>(iter: I) -> Self 
    where I: IntoIterator<Item = T>
    {
        let elements: Vec<T> = iter.into_iter().collect();
        let list = LinkedList::create_from_vec(&elements);

        list
    }
}