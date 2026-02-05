// The rand7() API is already defined for you.
// fn rand7() -> i32;

impl Solution {
    /// Implements rand10() using rejection sampling with rand7().
    ///
    /// # Intuition
    /// Two calls to rand7() produce a uniform distribution over 49 outcomes.
    /// Reject values above 40 and map the rest to 1–10 uniformly.
    ///
    /// # Approach
    /// 1. Generate a value in [1, 49] using (rand7()-1)*7 + rand7().
    /// 2. If the value exceeds 40, retry.
    /// 3. Return (value % 10) + 1.
    ///
    /// # Complexity
    /// - Time: O(1) expected — each iteration succeeds with probability 40/49
    /// - Space: O(1)
    pub fn rand10() -> i32 {
        loop {
            let val = (rand7() - 1) * 7 + rand7();
            if val <= 40 {
                return (val % 10) + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    // Thread-safe mock for rand7 using thread_local
    thread_local! {
        static RAND7_SEQUENCE: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
        static RAND7_INDEX: RefCell<usize> = const { RefCell::new(0) };
    }

    fn set_rand7_sequence(seq: Vec<i32>) {
        RAND7_SEQUENCE.with(|s| *s.borrow_mut() = seq);
        RAND7_INDEX.with(|i| *i.borrow_mut() = 0);
    }

    fn rand7() -> i32 {
        RAND7_SEQUENCE.with(|seq| {
            RAND7_INDEX.with(|idx| {
                let mut index = idx.borrow_mut();
                let sequence = seq.borrow();
                if *index >= sequence.len() {
                    *index = 0;
                }
                let result = sequence[*index];
                *index += 1;
                result
            })
        })
    }

    fn rand10() -> i32 {
        loop {
            let val = (rand7() - 1) * 7 + rand7();
            if val <= 40 {
                return (val % 10) + 1;
            }
        }
    }

    #[test]
    fn test_rand10_basic_values() {
        // Test case where (rand7()-1)*7 + rand7() = 1 (0*7 + 1)
        set_rand7_sequence(vec![1, 1]);
        assert_eq!(rand10(), 1);

        // Test case where (rand7()-1)*7 + rand7() = 10 (1*7 + 3)
        set_rand7_sequence(vec![2, 3]);
        assert_eq!(rand10(), 10);
    }

    #[test]
    fn test_rand10_rejection_sampling() {
        // Test rejection: val = 49 (rejected), then val = 20 (accepted)
        set_rand7_sequence(vec![7, 7, 2, 6]);
        assert_eq!(rand10(), 10); // 20 % 10 + 1 = 10
    }

    #[test]
    fn test_rand10_distribution() {
        // Test that all values 1-10 can be generated
        let test_cases = vec![
            (vec![1, 1], 1),  // val = 1 -> 1
            (vec![1, 2], 2),  // val = 2 -> 2
            (vec![1, 3], 3),  // val = 3 -> 3
            (vec![1, 4], 4),  // val = 4 -> 4
            (vec![1, 5], 5),  // val = 5 -> 5
            (vec![1, 6], 6),  // val = 6 -> 6
            (vec![1, 7], 7),  // val = 7 -> 7
            (vec![2, 1], 8),  // val = 8 -> 8
            (vec![2, 2], 9),  // val = 9 -> 9
            (vec![2, 3], 10), // val = 10 -> 10
        ];

        test_cases.into_iter().for_each(|(sequence, expected)| {
            set_rand7_sequence(sequence);
            assert_eq!(rand10(), expected);
        });
    }

    #[test]
    fn test_rand10_multiple_rejections() {
        // Multiple rejections: 49, 48, 47, then 30 (accepted)
        set_rand7_sequence(vec![7, 7, 7, 6, 7, 5, 4, 2]);
        assert_eq!(rand10(), 10); // 30 % 10 + 1 = 10
    }

    #[test]
    fn test_rand10_edge_cases() {
        // Test val = 40 (boundary, should be accepted)
        set_rand7_sequence(vec![6, 5]);
        assert_eq!(rand10(), 10); // 40 % 10 + 1 = 10

        // Test val = 41 (should be rejected), then val = 11
        set_rand7_sequence(vec![6, 6, 2, 4]);
        assert_eq!(rand10(), 1); // 11 % 10 + 1 = 1
    }
}
