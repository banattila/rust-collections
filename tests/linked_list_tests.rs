#[cfg(test)] 
pub mod linked_list_tests {
    use collection::lists::linked_list_mod::linked_list::LinkedList;
    use collection::lists::nodes_mod::node::Node;
    use collection::lists::traits::{collections::Collections, list::List};


    fn test_paniced() {
        panic!("{}", "Test failed");
    }

    #[test]
    fn create_list() {
        let list = LinkedList::<i8>::new();
        assert!(list.is_empty());
    }

    #[test]
    fn set_head_test() {
        let node_data = 2i8;
        let node = Node::new(node_data);
        let mut list = LinkedList::<i8>::new();

        assert!(list.is_empty());

        list.set_head(node);

        assert!(!list.is_empty());

        if let Some(head) = list.get_head() {
            assert_eq!(head.get_data(), node_data);
        }
        else {
            test_paniced();
        }
    }

    #[test]
    fn add_if_list_is_empty() {
        let data = 2i8;
        let mut list = LinkedList::<i8>::new();

        assert!(list.is_empty());

        list.add(data);

        assert!(!list.is_empty());

        if let Some(head) = list.get_head() {
            assert_eq!(head.get_data(), data);
        }
        else {
            test_paniced();
        }
    }

    #[test]
    fn add_more_item() {
        let data_one = 2i8;
        let data_two = 3i8;

        let mut list = LinkedList::<i8>::new();

        assert!(list.is_empty());

        list.add(data_one);
        
        assert!(!list.is_empty());

        if let Some(head) = list.get_head() {
            assert_eq!(head.get_data(), data_one);
        }
        else {
            test_paniced();
        }

        list.add(data_two);

        if let Some(head) = list.get_head() {
            assert_eq!(head.get_data(), data_two);
        }
        else {
            test_paniced();
        }
    }

    #[test]
    fn remove_if_list_is_empty() {
        let mut list = LinkedList::<i8>::new();

        match list.remove(2i8) {
            Ok(_) => {
                test_paniced();
            },
            Err(msg) => assert_eq!(msg, "List is empty".to_string()),
        };
    }

    #[test]
    fn remove_if_list_not_contains() {
        let data = 2i8;
        let mut list = LinkedList::<i8>::new();
        list.add(data);

        assert!(!list.is_empty());

        match list.remove(1i8) {
            Ok(_) => test_paniced(),
            Err(msg) => assert_eq!(msg, "Item not found".to_string()),
        }
    }

    #[test]
    fn remove_if_list_contains_item_and_it_is_head() {
        let data = 2i8;
        let data_two = 3i8;
        let mut list = LinkedList::<i8>::new();
        list.add(data);

        assert!(!list.is_empty());

        match list.remove(data) {
            Ok(msg) => assert_eq!(msg, "Item removed".to_string()),
            Err(_) => test_paniced(),
        }

        list.add(data);
        list.add(data_two);

        match list.remove(data_two) {
            Ok(msg) => {
                assert_eq!(msg, "Item removed".to_string());
                if let Some(head) = list.get_head() {
                    assert_eq!(head.get_data(), data);
                }
                else {
                    test_paniced();
                }
            },
            Err(_) => test_paniced(),
        }
    }

    #[test]
    fn remove_from_list_if_item_not_is_head() {
        let data_one = 2i8;
        let data_two = 3i8;
        let data_three = 4i8;

        let mut list = LinkedList::<i8>::new();

        list.add(data_one);
        list.add(data_two);
        list.add(data_three);

        assert!(!list.is_empty());

        match list.remove(data_two) {
            Ok(msg) => {
                assert_eq!(msg, "Item removed".to_string());
                if let Some(mut head) = list.get_head() {
                    assert_eq!(head.get_data(), data_three);
                    let next = head.get_next();

                    if let Some(next_node) = next{
                        assert_eq!(next_node.get_data(), data_one);
                    }
                    else {
                        test_paniced();
                    }
                }
                else {
                    test_paniced();
                }
            },
            Err(_) => test_paniced(),
        }
    }

    #[test]
    fn if_not_contains_item() {
        let data = 2i8;
        let searched = 3i8;
        let mut list = LinkedList::<i8>::new();
        list.add(data);

        assert!(!list.is_empty());

        assert!(!list.contains(searched));
    }

    #[test]
    fn if_contains_item() {
        let data = 2i8;
        let mut list = LinkedList::<i8>::new();

        list.add(data);

        assert!(!list.is_empty());
        assert!(list.contains(data));
    }

    #[test]
    fn append_if_list_is_empty() {
        let data = 2i8;
        let mut list = LinkedList::<i8>::new();

        assert!(list.is_empty());

        list.append(data);

        assert!(!list.is_empty());

        if let Some(head) = list.get_head() {
            assert_eq!(head.get_data(), data);
        }   
        else {
            test_paniced();
        }
    }

    #[test]
    fn append_if_list_is_not_empty() {
        let data_one = 2i8;
        let data_two = 3i8;
        let data_three = 5i8;

        let mut list = LinkedList::<i8>::new();

        assert!(list.is_empty());

        list.append(data_one);

        assert!(!list.is_empty());
        assert!(list.contains(data_one));

        if let Some(head) = list.get_head() {
            assert_eq!(head.get_data(), data_one);
        }
        else {
            test_paniced();
        }

        list.append(data_two);

        assert!(list.contains(data_two));

        if let Some(head) = list.get_head() {
            assert_eq!(head.get_data(), data_one);
        }
        else {
            test_paniced();
        }

        list.append(data_three);

        assert!(list.contains(data_three));

        let current = &mut list.get_head();
        
        loop {
            match current {
                None => test_paniced(),
                Some(node) => {
                    if node.has_next() {
                        *current = node.next.take();
                    }
                    else {
                        break;
                    }
                },
            }
        }
        
        assert_eq!(current.clone().unwrap().get_data(), data_three);
    }

    #[test]
    fn get_item_if_index_to_big() {

        let data = 2i8;
        let index = 2;

        let mut list = LinkedList::<i8>::new();

        list.add(data);
        assert!(!list.is_empty());
        assert!(list.contains(data));

        match list.get(index) {
            Ok(_) => test_paniced(),
            Err(msg) => assert_eq!(msg, "Index is too big".to_string())
        }
    }

    #[test]
    fn get_item_if_index_ok() {
        let data_one = 1i8;
        let data_two = 2i8;
        let data_three = 3i8;
        let mut list = LinkedList::<i8>::new();

        list.add(data_one);
        list.add(data_two);
        list.add(data_three);

        assert!(!list.is_empty());
        assert!(list.contains(data_one));
        assert!(list.contains(data_two));
        assert!(list.contains(data_three));

        match list.get(0) {
            Ok(item) => assert_eq!(item, data_three),
            Err(_) => test_paniced(),
        }

        match list.get(1) {
            Ok(item) => assert_eq!(item, data_two),
            Err(_) => test_paniced(),
        }

        match list.get(2) {
            Ok(item) => assert_eq!(item, data_one),
            Err(_) => test_paniced(),
        }
    }

    #[test]
    fn create_from_vector() {
        let vector: Vec<i8> = vec![1, 2, 3, 4, 5, 6];

        let list = LinkedList::create_from_vec(&vector);

        for (index, item) in vector.into_iter().enumerate() {
            match  list.get(index){
                Err(_) => test_paniced(),
                Ok(item_from_list) => assert_eq!(item_from_list, item),
            }
        }
    }

    #[test]
    fn into_vec_test() {
        let mut list = LinkedList::<i32>::new();
        for i in 1..=10 {
            list.append(i);
        }

        let vector = list.into_vec();

        assert!(!vector.is_empty());

        for (index, item) in vector.into_iter().enumerate() {
            match list.get(index) {
                Err(_) => test_paniced(),
                Ok(item_from_list) => assert_eq!(item, item_from_list),
            }
        }
    }

    #[test]
    fn create_from_other_list() {
        let mut original = LinkedList::<i8>::new();

        for i in 0..=10 {
            original.append(i);
        }

        let copied = LinkedList::create_from_list(&original);
        let mut counter = 0;

        loop {
            if counter == 10 {
                break;
            }

            let original_value = original.get(counter);
            let copied_value = copied.get(counter);

            assert_eq!(original_value, copied_value);
            counter += 1;
        }
    }

    #[test]
    fn iterator_test() {
        let mut list = LinkedList::<i8>::new();
        for i in 0..=10 {
            list.append(i);
        }

        let iterator = list.into_iter();

        for (index, item) in iterator.enumerate() {
            let i: i8 = index.try_into().unwrap();
            assert_eq!(item, i);
        }
    }

    #[test]
    fn filter_with_iterator() {
        let mut list = LinkedList::<i8>::new();

        for i in 0..=10 {
            list.append(i);
        }

        let iterator = list.into_iter();

        let mut filtered_list: LinkedList<i8>= iterator.filter(|x| x % 2 == 0).collect();

        for item in filtered_list.into_iter() {
            assert!(item % 2 != 1);
        }

        for (index, item) in list.into_iter().enumerate() {
            let i: i8 = index.try_into().unwrap();
            assert_eq!(item, i);
        }
    }

    #[test]
    fn map_test_with_iterator() {
        let mut list = LinkedList::<i8>::new();

        for i in 0..=10 {
            list.append(i);
        }

        assert!(!list.is_empty());

        let mut mapped_list: LinkedList<i8> = list.into_iter().map(|item|  item * 2).collect();

        for (index, item) in mapped_list.into_iter().enumerate() {
            let i: i8 = index.try_into().unwrap();
            assert_eq!(item, (i * 2));
        }

        for (index, item) in list.into_iter().enumerate() {
            let i: i8 = index.try_into().unwrap();
            assert_eq!(item, i);
        }
    }
}