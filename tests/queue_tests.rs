#[cfg(test)]
pub mod queue_tests {
    use collection::lists::{queue_mod::queue::Queue, traits::collections::Collections};



    fn test_paniced() {
        panic!("Test failed");
    }

    #[test]
    fn create_queue() {
        let queue = Queue::<i8>::new();

        assert!(queue.is_empty());
    }

    #[test]
    fn add_to_queue_if_queue_is_empty() {
        let data = 1i8;
        let mut queue = Queue::<i8>::new();
        assert!(queue.is_empty());

        queue.add(data);
        assert!(!queue.is_empty());
    }

    #[test]
    fn add_to_queue_if_not_empty() {
        let data_one = 1i8;
        let data_two = 2i8;

        let mut queue = Queue::<i8>::new();

        assert!(queue.is_empty());

        queue.add(data_one);
        assert!(!queue.is_empty());
        queue.add(data_two);
        assert!(!queue.is_empty());

        match queue.get_tail() {
            None => test_paniced(),
            Some(node) => {
                let mut _node = node.clone();
                assert_eq!(node.get_data(), data_one);

                match _node.get_previous() {
                    None => test_paniced(),
                    Some(first_node) => {
                        assert!(!first_node.has_previous());
                        assert_eq!(first_node.get_data(), data_two);
                    },
                }
            },
        }
    }

    #[test]
    fn dequeue_from_queue_if_queue_is_empty() {
        let mut queue = Queue::<i8>::new();

        assert!(queue.is_empty());

        match queue.dequeue() {
            Ok(_) => test_paniced(),
            Err(msg) => assert_eq!(msg, "Queue is empty".to_string()),
        }
    }

    #[test]
    fn dequeue_from_queue_if_queue_is_not_empty() {
        let data = 1i8;
        let mut queue = Queue::<i8>::new();

        assert!(queue.is_empty());

        queue.add(data);
        assert!(!queue.is_empty());

        match queue.dequeue() {
            Err(_) => test_paniced(),
            Ok(item) => {
                assert_eq!(item, data);
                assert!(queue.is_empty());
            }
        }
    }

    #[test]
    fn dequeue_from_queue_if_queue_contains_many_element() {
        let mut queue = Queue::<i8>::new();

        assert!(queue.is_empty());

        for i in 0..=10 {
            queue.add(i);
        }

        assert!(!queue.is_empty());

        for i in 0..=10 {
            match queue.dequeue() {
                Err(_) => test_paniced(),
                Ok(item) => {
                    assert_eq!(item, i);
                    if i < 10 {
                        assert!(!queue.is_empty());
                    }
                    else {
                        assert!(queue.is_empty());
                    }
                }
            }
        }
    }
}