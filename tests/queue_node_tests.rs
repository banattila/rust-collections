#[cfg(test)]
pub mod queue_node_tests {

    use collection::lists::queue_mod::queue_node::QueueNode;

    #[test]
    fn create_node() {
        let data = 2i8;
        let node = QueueNode::new(data);

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
    fn set_previous_test() {
        let data_one = 2i8;
        let data_two = 3i8;
        let mut node_one = QueueNode::new(data_one);
        let node_two = QueueNode::new(data_two);
        node_one.set_previous(node_two);
        assert!(node_one.has_previous());
        assert_eq!(node_one.get_data(), data_one);
    }
}