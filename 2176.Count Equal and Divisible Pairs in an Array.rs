impl Solution {
    /// Count pairs (i, j) where nums[i] == nums[j] and i * j is divisible by k.
    ///
    /// # Intuition
    /// Brute-force all index pairs since constraints are small (n ≤ 100).
    ///
    /// # Approach
    /// For each pair (i, j) with i < j, check value equality and divisibility.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(1)
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        (1..nums.len())
            .flat_map(|j| {
                nums[..j]
                    .iter()
                    .enumerate()
                    .filter(move |(i, x)| *x == nums[j] && (i * j) as i32 % k == 0)
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
    }

    #[test]
    fn no_pairs() {
        assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 1), 0);
    }

    #[test]
    fn all_equal_k_one() {
        assert_eq!(Solution::count_pairs(vec![1, 1, 1], 1), 3);
    }
}
