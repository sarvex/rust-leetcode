use std::collections::HashSet;

impl Solution {
    /// Counts distinct integers after adding reversed versions of each number.
    ///
    /// # Intuition
    /// For each number, also include its digit-reversal. Use a HashSet to track
    /// all distinct values.
    ///
    /// # Approach
    /// 1. For each number, compute its reverse by extracting digits
    /// 2. Insert both the original and reversed value into a HashSet
    /// 3. Return the set size
    ///
    /// # Complexity
    /// - Time: O(n * d) where d is the max digit count
    /// - Space: O(n)
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let reverse_digits = |mut n: i32| -> i32 {
            let mut rev = 0;
            while n > 0 {
                rev = rev * 10 + n % 10;
                n /= 10;
            }
            rev
        };

        let set: HashSet<i32> = nums
            .iter()
            .flat_map(|&num| [num, reverse_digits(num)])
            .collect();

        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::count_distinct_integers(vec![1, 13, 10, 12, 31]),
            6
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_distinct_integers(vec![2, 2, 2]), 1);
    }

    #[test]
    fn test_palindrome_number() {
        assert_eq!(Solution::count_distinct_integers(vec![121]), 1);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::count_distinct_integers(vec![5]), 1);
    }

    #[test]
    fn test_trailing_zeros() {
        assert_eq!(Solution::count_distinct_integers(vec![100]), 2);
    }
}
