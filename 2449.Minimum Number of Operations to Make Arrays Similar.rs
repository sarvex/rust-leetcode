impl Solution {
    /// Finds minimum operations to make nums similar to target via +2/-2 swaps.
    ///
    /// # Intuition
    /// Since each operation adds 2 to one element and subtracts 2 from another,
    /// parity is preserved. Odd elements can only match odd targets and even
    /// elements can only match even targets.
    ///
    /// # Approach
    /// 1. Separate both arrays into odd and even groups
    /// 2. Sort each group independently
    /// 3. Match sorted elements by position (greedy pairing)
    /// 4. Sum absolute differences and divide by 4 (each operation moves 4 total units)
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(n) for separated groups
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
        assert_eq!(Solution::make_similar(vec![8, 12, 6], vec![2, 14, 10]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::make_similar(vec![1, 2, 5], vec![4, 1, 3]), 1);
    }

    #[test]
    fn test_already_similar() {
        assert_eq!(
            Solution::make_similar(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn test_all_even() {
        assert_eq!(Solution::make_similar(vec![2, 4, 6], vec![4, 6, 2]), 0);
    }

    #[test]
    fn test_all_odd() {
        assert_eq!(Solution::make_similar(vec![1, 3, 5], vec![5, 3, 1]), 0);
    }

    #[test]
    fn test_large_difference() {
        assert_eq!(Solution::make_similar(vec![2, 10], vec![6, 6]), 1);
    }
}
