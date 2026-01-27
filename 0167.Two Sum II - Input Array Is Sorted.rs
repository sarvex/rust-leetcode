use std::cmp::Ordering;

impl Solution {
    /// Finds two numbers in a sorted array that sum to target using two pointers.
    ///
    /// # Intuition
    /// With a sorted array, two pointers converging from both ends can efficiently
    /// find the pair: move left pointer right when sum is too small, right pointer
    /// left when too large.
    ///
    /// # Approach
    /// 1. Initialize left at start, right at end.
    /// 2. Compare the sum with the target using `cmp`.
    /// 3. Adjust pointers until the target sum is found.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);
        loop {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => break,
            }
        }
        vec![left as i32 + 1, right as i32 + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn adjacent_elements() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
