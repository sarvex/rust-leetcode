impl Solution {
    /// Minimizes the distance of a good tuple (i, j, k) with equal elements.
    ///
    /// # Intuition
    /// For any triple of indices i ≤ j ≤ k, the distance formula simplifies:
    /// `|i-j| + |j-k| + |k-i| = (j-i) + (k-j) + (k-i) = 2*(k-i)`.
    /// Thus the distance depends only on the outermost two indices, and we
    /// need to minimize `k - i` across all triples sharing the same value.
    ///
    /// # Approach
    /// 1. Use a `Vec` indexed by value (since `1 <= nums[i] <= n`) to store
    ///    only the two most recent indices per value, avoiding HashMap overhead.
    /// 2. Single pass: when we see a third+ occurrence, compute `k - prev_prev`
    ///    and update the answer, then shift the window forward.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prev = vec![[u32::MAX; 2]; n + 1];
        let mut result = u32::MAX;

        for (i, &v) in nums.iter().enumerate() {
            let slot = &mut prev[v as usize];
            if slot[0] != u32::MAX {
                result = result.min(i as u32 - slot[0]);
            }
            slot[0] = slot[1];
            slot[1] = i as u32;
        }

        if result == u32::MAX {
            -1
        } else {
            2 * result as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::minimum_distance(vec![1]), -1);
    }

    #[test]
    fn test_no_triple() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 3, 4, 5]), -1);
    }

    #[test]
    fn test_only_pairs() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 2]), -1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::minimum_distance(vec![5, 5, 5]), 4);
    }

    #[test]
    fn test_consecutive_triple() {
        assert_eq!(Solution::minimum_distance(vec![3, 3, 3, 1, 2]), 4);
    }

    #[test]
    fn test_multiple_groups() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 2, 1, 2]), 8);
    }
}
