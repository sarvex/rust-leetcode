impl Solution {
    /// Minimum Number of Operations to Make Arrays Similar
    ///
    /// # Intuition
    /// Since each operation adds 2 to one element and subtracts 2 from another,
    /// the parity (odd/even) of elements is preserved. This means odd elements
    /// can only be transformed into other odd values, and even elements into
    /// other even values. We must match elements by parity.
    ///
    /// # Approach
    /// 1. Separate both arrays into odd and even groups
    /// 2. Sort each group independently
    /// 3. Match sorted elements by position (greedy pairing)
    /// 4. Calculate total absolute difference between matched pairs
    /// 5. Divide by 4 since each operation contributes 4 units of change
    ///    (+2 to one element, -2 to another)
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(n) for storing separated groups
    pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let (mut nums_odd, mut nums_even): (Vec<i64>, Vec<i64>) =
            nums.iter().map(|&x| x as i64).partition(|&x| x % 2 == 1);

        let (mut target_odd, mut target_even): (Vec<i64>, Vec<i64>) =
            target.iter().map(|&x| x as i64).partition(|&x| x % 2 == 1);

        nums_odd.sort_unstable();
        nums_even.sort_unstable();
        target_odd.sort_unstable();
        target_even.sort_unstable();

        let total_diff: i64 = nums_odd
            .iter()
            .zip(target_odd.iter())
            .chain(nums_even.iter().zip(target_even.iter()))
            .map(|(num, tgt)| (num - tgt).abs())
            .sum();

        total_diff / 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![8, 12, 6];
        let target = vec![2, 14, 10];
        assert_eq!(Solution::make_similar(nums, target), 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 5];
        let target = vec![4, 1, 3];
        assert_eq!(Solution::make_similar(nums, target), 1);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::make_similar(nums, target), 0);
    }

    #[test]
    fn test_all_even() {
        let nums = vec![2, 4, 6];
        let target = vec![4, 6, 2];
        assert_eq!(Solution::make_similar(nums, target), 0);
    }

    #[test]
    fn test_all_odd() {
        let nums = vec![1, 3, 5];
        let target = vec![5, 3, 1];
        assert_eq!(Solution::make_similar(nums, target), 0);
    }

    #[test]
    fn test_large_difference() {
        let nums = vec![2, 10];
        let target = vec![6, 6];
        assert_eq!(Solution::make_similar(nums, target), 1);
    }
}
