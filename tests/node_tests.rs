#[cfg(test)]
pub mod node_tests {
    use collection::lists::nodes_mod::node::Node;



    #[test]
    fn create_node() {
        let data = 2i8;
        let node = Node::new(data);

        assert!(!node.has_next());

        assert_eq!(node.get_data(), data);
    }

    #[test]
    fn set_data_test() {
        let data_first = 2i8;
        let data_second = 3i8;

        let mut node = Node::new(data_first);

        assert!(!node.has_next());
        assert_eq!(node.get_data(), data_first);

        node.set_data(&data_second);
        assert!(!node.has_next());
        assert_eq!(node.get_data(), data_second);
    }

    #[test]
    fn set_next_test() {
        let first_data = 2i8;
        let second_data = 4i8;

        let mut node_one = Node::new(first_data);
        let node_two = Node::new(second_data);

        assert!(!node_one.has_next());
        assert!(!node_two.has_next());
        assert_eq!(node_one.get_data(), first_data);

        node_one.set_next(node_two);
        assert!(node_one.has_next());
        assert_eq!(node_one.get_data(), first_data);

    }
}