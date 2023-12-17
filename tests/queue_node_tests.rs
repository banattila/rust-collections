#[cfg(test)]
pub mod queue_node_tests {
    use collection::lists::queue_mod::queue_node::QueueNode;


    fn test_paniced() {
        panic!("Test failed");
    }

    #[test]
    fn create_node() {
        let data = 2i8;
        let node = QueueNode::new(data);

        assert!(!node.has_next());
        assert!(!node.has_previous());
        assert_eq!(node.get_data(), data);
    }

    #[test]
    fn set_data() {
        let data_one = 2i8;
        let data_two = 3i8;

        let mut node = QueueNode::new(data_one);

        assert_eq!(node.get_data(), data_one);

        node.set_data(&data_two);
        assert_eq!(node.get_data(), data_two);

    }

    #[test]
    fn set_next_test() {
        let data_one = 2i8;
        let data_two = 3i8;
        let mut node_one = QueueNode::new(data_one);
        let node_two = QueueNode::new(data_two);

        node_one.set_next(node_two);
        assert!(node_one.has_next());
        assert!(!node_one.has_previous());
        assert_eq!(node_one.get_data(), data_one);

        match node_one.get_next() {
            None => test_paniced(),
            Some(node) => {
                assert!(node.has_previous());
                assert_eq!(node.get_data(), data_two);
            },
        }

    }

    #[test]
    fn set_prevoius_test() {
        let data_one = 1i8;
        let data_two = 2i8;
        let data_three = 3i8;

        let mut node_one = QueueNode::new(data_one);
        let mut node_two = QueueNode::new(data_two);
        let node_three = QueueNode::new(data_three);

        assert!(!node_one.has_next());
        assert!(!node_two.has_next());
        assert!(!node_three.has_next());

        assert!(!node_one.has_previous());
        assert!(!node_two.has_previous());
        assert!(!node_three.has_previous());

        node_two.set_next(node_three);
        node_one.set_next(node_two);
        
        assert!(node_one.has_next());
        assert!(!node_one.has_previous());

        match node_one.get_previous() {
            None => (),
            Some(_) => test_paniced(),
        }

        match node_one.get_next() {
            None => test_paniced(),
            Some(node) => {
                assert!(node.has_next());
                assert!(node.has_previous());
                assert_eq!(node.get_data(), data_two);

                match node.get_next() {
                    None => test_paniced(),
                    Some(last_node) => {
                        assert!(!last_node.has_next());
                        assert!(last_node.has_previous());
                        assert_eq!(last_node.get_data(), data_three);
                    }
                }
            },
        }
    }
}