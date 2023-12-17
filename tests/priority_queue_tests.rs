#[cfg(test)]
pub mod priority_queue_tests {
    use collection::lists::{priority_queue_mod::priority_queue::PriorityQueue, traits::collections::Collections};
    use rand::Rng;


    fn test_paniced() {
        panic!("Test failed");
    }

    #[test]
    fn create_prior_queue() {
        let p_queue = PriorityQueue::<i8>::new(true);
        assert!(p_queue.is_empty());

        assert_eq!(p_queue.get_size(), 0);
    }

    #[test]
    fn add_to_p_queue_if_it_is_empty() {
        let data = 1i8;
        let data_two = 5i8;
        let data_three = 2i8;
        let mut p_queue = PriorityQueue::<i8>::new(true);

        assert!(p_queue.is_empty());

        p_queue.add(data);
        assert!(!p_queue.is_empty());
        p_queue.add(data_two);
        assert!(!p_queue.is_empty());
        p_queue.add(data_three);
        assert!(!p_queue.is_empty());
        assert_eq!(p_queue.get_size(), 3);

    }

    #[test]
    fn dequeue_test_from_p_queue_if_it_is_empty() {
        let mut p_queue = PriorityQueue::<i8>::new(true);
        assert!(p_queue.is_empty());

        match p_queue.dequeue() {
            Ok(_) => test_paniced(),
            Err(msg) => assert_eq!(msg, "Queue is empty".to_string()),
        }

        assert_eq!(p_queue.get_size(), 0);
    }

    #[test]
    fn dequeue_test_from_p_queue_if_it_is_not_empty_and_min() {
        let mut p_queue = PriorityQueue::<i8>::new(true);
        let mut rng = rand::thread_rng();

        assert!(p_queue.is_empty());

        for _ in 0..=10 {
            let random: i8 = rng.gen_range(0..=100);
            p_queue.add(random);
        }

        assert_eq!(p_queue.get_size(), 11);

        let mut prev_number: i8 = 0;
        match p_queue.dequeue() {
            Err(_) => test_paniced(),
            Ok(item) => prev_number = item,
        }

        for _ in 0..10 {
            match p_queue.dequeue() {
                Err(_) => test_paniced(),
                Ok(item) =>{
                    assert!(item >= prev_number);
                    prev_number = item;
                },
            }
        }
        assert_eq!(p_queue.get_size(), 0);
    }

    #[test]
    fn dequeue_test_from_p_queue_if_it_is_not_empty_and_max() {
        let mut p_queue = PriorityQueue::<i8>::new(false);
        let mut rng = rand::thread_rng();

        assert!(p_queue.is_empty());

        for _ in 0..=10 {
            let random: i8 = rng.gen_range(0..=100);
            p_queue.add(random);
        }
        assert_eq!(p_queue.get_size(), 11);

        let mut prev_number: i8 = 120;
        match p_queue.dequeue() {
            Err(_) => test_paniced(),
            Ok(item) => prev_number = item,
        }

        for _ in 0..10 {
            match p_queue.dequeue() {
                Err(_) => test_paniced(),
                Ok(item) =>{
                    assert!(item <= prev_number);
                    prev_number = item;
                },
            }
        }
        assert_eq!(p_queue.get_size(), 0);
    }
}