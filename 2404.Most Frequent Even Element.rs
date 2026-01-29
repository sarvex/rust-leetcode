use std::collections::HashMap;

impl Solution {
    /// Finds the most frequent even element, returning the smallest on ties.
    ///
    /// # Intuition
    /// Count occurrences of each even number and pick the one with the highest
    /// frequency, breaking ties by choosing the smallest value.
    ///
    /// # Approach
    /// 1. Filter even elements and count frequencies using a HashMap
    /// 2. Fold over entries to find the element with maximum count and minimum value on tie
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let cnt: HashMap<i32, i32> =
            nums.iter()
                .filter(|&&x| x % 2 == 0)
                .fold(HashMap::new(), |mut acc, &x| {
                    *acc.entry(x).or_insert(0) += 1;
                    acc
                });

        cnt.iter()
            .fold((-1, 0), |(ans, mx), (&x, &v)| {
                if *v > mx || (v == mx && (ans == -1 || x < ans)) {
                    (*x, *v)
                } else {
                    (ans, mx)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::most_frequent_even(vec![4, 4, 4, 9, 2, 4]), 4);
    }

    #[test]
    fn test_no_even_elements() {
        assert_eq!(Solution::most_frequent_even(vec![29, 47, 21, 41, 13]), -1);
    }

    #[test]
    fn test_single_even() {
        assert_eq!(Solution::most_frequent_even(vec![1, 2, 3]), 2);
    }

    #[test]
    fn test_tie_returns_smallest() {
        assert_eq!(Solution::most_frequent_even(vec![2, 4, 2, 4]), 2);
    }
}
