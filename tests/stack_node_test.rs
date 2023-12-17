#[cfg(test)]
pub mod stack_node_test {
    use collection::lists::stack_mod::stack_node::StackNode;

    fn test_paniced() {
        panic!("Test failed");
    }

    #[test]
    fn create_node() {
        let data = 2i8;
        let node = StackNode::new(data);

        assert!(!node.has_next());
        assert_eq!(node.get_data(), data)
    }

    #[test]
    fn set_next() {
        let data_one = 2i8;
        let data_two = 3i8;

        let mut node = StackNode::new(data_one);
        let node_two = StackNode::new(data_two);

        node.set_next(Some(Box::new(node_two)));

        assert!(node.has_next());

        assert_eq!(node.get_data(), data_one);

        match node.get_next() {
            None => test_paniced(),
            Some(node) => assert_eq!(node.get_data(), data_two),
        }
    }

    #[test]
    fn set_data_test() {
        let data_one = 2i8;
        let data_two = 3i8;

        let mut node = StackNode::new(data_one);

        assert!(!node.has_next());
        assert_eq!(node.get_data(), data_one);

        node.set_data(&data_two);

        assert!(!node.has_next());
        assert_eq!(node.get_data(), data_two);
    }
}