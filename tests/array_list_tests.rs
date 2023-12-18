#[cfg(test)]
pub mod array_list_tests {
    use collection::lists::{array_list_mod::array_list::ArrayList, traits::collections::Collections};


    #[test]
    fn create_array_list() {
        let list = ArrayList::<i8>::new();
        assert!(list.is_empty());
        assert_eq!(list.get_size(), 0);
    }

    #[test]
    fn add_element_to_array_list() {
        let mut list = ArrayList::<i8>::new();
        assert!(list.is_empty());
        assert_eq!(list.get_size(), 0);

        list.add(1i8);
        assert!(!list.is_empty());
        assert_eq!(list.get_size(), 1);
    }

    #[test]
    fn add_more_than_twenty_element_to_array_list() {
        let add_item_count = 30i8;
        let mut list = ArrayList::<i8>::new();

        assert!(list.is_empty());
        assert_eq!(list.get_size(), 0);

        for i in 0..=add_item_count {
            list.add(i);
        }

        assert!(!list.is_empty());
        assert_eq!(list.get_size(), 31);
        dbg!(&list);
    }
}