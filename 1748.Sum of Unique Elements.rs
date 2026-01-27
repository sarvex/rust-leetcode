impl Solution {
    /// Sums elements that appear exactly once using a frequency array.
    ///
    /// # Intuition
    /// Count frequencies and add only those with count equal to one.
    ///
    /// # Approach
    /// 1. Build a frequency array (values are at most 100).
    /// 2. Sum indices whose count is exactly 1.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed-size array
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut cnt = [0u8; 101];
        nums.iter().for_each(|&x| cnt[x as usize] += 1);
        (1..101).filter(|&x| cnt[x] == 1).sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3]), 6);
    }
}
