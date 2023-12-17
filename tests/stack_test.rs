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
        assert_eq!(stack.get_size(), 0);
    }

    #[test]
    fn add_element_to_stack_if_empty() {
        let data = 2i8;
        let mut stack = Stack::<i8>::new();

        assert!(stack.is_empty());
        assert_eq!(stack.get_size(), 0);

        stack.add(data);

        assert!(!stack.is_empty());
        assert_eq!(stack.get_size(), 1);

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
        assert_eq!(stack.get_size(), 2);

        match stack.get_head() {
            None => test_paniced(),
            Some(node) => assert_eq!(node.get_data(), data_two),
        }
    }

    #[test]
    fn stack_add_many_items() {

        let mut stack = Stack::<i8>::new();

        assert!(stack.is_empty());
        assert_eq!(stack.get_size(), 0);

        for i in 0..=10 {
            stack.add(i);
        }
        assert_eq!(stack.get_size(), 11);

        assert!(!stack.is_empty());

        match stack.get_head() {
            None => test_paniced(),
            Some(node) => {
              assert!(node.has_next());
              assert_eq!(node.get_data(), 10i8);  
            },
        }
    }
    #[test]
    fn test_pop_if_stack_is_empty() {
        let mut stack = Stack::<i8>::new();

        assert!(stack.is_empty());

        match stack.pop() {
            None => assert!(stack.is_empty()),
            Some(_) => test_paniced()
        }
        assert_eq!(stack.get_size(), 0);
    }

    #[test]
    fn test_pop_if_stack_has_one_element() {
        let data = 2i8;
        let mut stack = Stack::<i8>::new();

        assert!(stack.is_empty());

        match stack.pop() {
            None => assert!(stack.is_empty()),
            Some(_) => test_paniced()
        }

        stack.add(data);
        assert_eq!(stack.get_size(), 1);

        assert!(!stack.is_empty());

        match stack.pop() {
            None => test_paniced(),
            Some(item) => {
                assert!(stack.is_empty());
                assert_eq!(item, data);
            },
        }
        assert_eq!(stack.get_size(), 0);
    }

    #[test]
    fn test_pop_if_stack_has_many_element() {
        let mut stack = Stack::<i8>::new();
        assert!(stack.is_empty());
        assert_eq!(stack.get_size(), 0);

        for i in 0..=10 {
            stack.add(i);
        }
        assert_eq!(stack.get_size(), 11);

        assert!(!stack.is_empty());

        for i in 0..=10 {
            match stack.pop() {
                None => test_paniced(),
                Some(item) => {
                    assert_eq!(item, 10 - i);
                    if i < 10 {
                        assert!(!stack.is_empty());
                    }
                    else {
                        assert!(stack.is_empty());
                    }
                },
            }
        }
        assert_eq!(stack.get_size(), 0);
    }
}