impl Solution {
    /// Greedy approach with dominant value tracking for minimum swap cost.
    ///
    /// # Intuition
    /// We only need to fix positions where nums1[i] == nums2[i] (conflicts). When swapping
    /// two conflict positions with different values, both get resolved. If one value
    /// dominates (appears more than half the conflicts), we need help from non-conflict indices.
    ///
    /// # Approach
    /// 1. Identify all conflict indices and sum their costs
    /// 2. Track frequency of each value at conflict positions
    /// 3. Find the dominant value (most frequent among conflicts)
    /// 4. Calculate excess = 2 * max_freq - conflict_count (positions needed from outside)
    /// 5. Greedily add smallest non-conflict indices where both values differ from dominant
    /// 6. Return -1 if insufficient valid non-conflict indices exist
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through arrays
    /// - Space: O(n) — frequency array for value counts
    pub fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut cost: i64 = 0;
        let mut conflict_count = 0;
        let mut freq = vec![0i32; n + 1];
        let mut dominant_val = 0;
        let mut max_freq = 0;

        for (i, (&v1, &v2)) in nums1.iter().zip(nums2.iter()).enumerate() {
            if v1 == v2 {
                cost += i as i64;
                conflict_count += 1;
                freq[v1 as usize] += 1;
                if freq[v1 as usize] > max_freq {
                    max_freq = freq[v1 as usize];
                    dominant_val = v1;
                }
            }
        }

        let mut excess = 2 * max_freq - conflict_count;

        if excess > 0 {
            for (i, (&v1, &v2)) in nums1.iter().zip(nums2.iter()).enumerate() {
                if v1 != v2 && v1 != dominant_val && v2 != dominant_val {
                    cost += i as i64;
                    excess -= 1;
                    if excess == 0 {
                        break;
                    }
                }
            }
        }

        if excess > 0 {
            -1
        } else {
            cost
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_equal_arrays() {
        assert_eq!(
            Solution::minimum_total_cost(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            10
        );
    }

    #[test]
    fn test_dominant_value_needs_external() {
        assert_eq!(
            Solution::minimum_total_cost(vec![2, 2, 2, 1, 3], vec![1, 2, 2, 3, 3]),
            10
        );
    }

    #[test]
    fn test_impossible_case() {
        assert_eq!(
            Solution::minimum_total_cost(vec![1, 2, 2], vec![1, 2, 2]),
            -1
        );
    }

    #[test]
    fn test_no_conflicts() {
        assert_eq!(
            Solution::minimum_total_cost(vec![1, 2, 3], vec![3, 1, 2]),
            0
        );
    }

    #[test]
    fn test_single_conflict() {
        assert_eq!(
            Solution::minimum_total_cost(vec![1, 2, 3], vec![1, 1, 2]),
            2
        );
    }

    #[test]
    fn test_two_conflicts_different_values() {
        assert_eq!(
            Solution::minimum_total_cost(vec![1, 2, 3], vec![1, 2, 1]),
            1
        );
    }

    #[test]
    fn test_large_dominant() {
        assert_eq!(
            Solution::minimum_total_cost(vec![1, 1, 1, 2, 3], vec![1, 1, 1, 3, 2]),
            -1
        );
    }
}
