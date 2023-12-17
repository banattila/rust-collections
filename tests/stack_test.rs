#[cfg(test)]
pub mod stack_tests {
    use collection::lists::{stack_mod::stack::Stack, traits::collections::Collections};

    fn test_paniced() {
        panic!("Test failed");
    }

    #[test]
    fn create_stack() {

        let stack = Stack::<i8>::new();

        assert!(stack.is_empty());
    }

    #[test]
    fn add_element_to_stack_if_empty() {
        let data = 2i8;
        let mut stack = Stack::<i8>::new();

        assert!(stack.is_empty());

        stack.add(data);

        assert!(!stack.is_empty());

        match stack.get_head() {
            None => test_paniced(),
            Some(node) => assert_eq!(node.get_data(), data),
        }
    }

    #[test]
    fn add_to_stack_if_stack_is_not_empty() {

        let data_one = 2i8;
        let data_two = 3i8;

        let mut stack = Stack::new();

        assert!(stack.is_empty());

        stack.add(data_one);

        assert!(!stack.is_empty());

        match stack.get_head() {
            None => test_paniced(),
            Some(node) => assert_eq!(node.get_data(), data_one),
        }

        stack.add(data_two);
        assert!(!stack.is_empty());

        match stack.get_head() {
            None => test_paniced(),
            Some(node) => assert_eq!(node.get_data(), data_two),
        }
    }

    #[test]
    fn stack_add_many_items() {

        let mut stack = Stack::<i8>::new();

        assert!(stack.is_empty());

        for i in 0..=10 {
            stack.add(i);
        }

        assert!(!stack.is_empty());

        match stack.get_head() {
            None => test_paniced(),
            Some(node) => {
              assert!(node.has_next());
              assert_eq!(node.get_data(), 10i8);  
            },
        }
    }
}