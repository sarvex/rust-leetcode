impl Solution {
    /// Greedy approach with dominant value tracking for minimum swap cost
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
    /// 5. Greedily add smallest non-conflict indices where both nums1[i] and nums2[i] differ
    ///    from the dominant value
    /// 6. Return -1 if insufficient valid non-conflict indices exist
    ///
    /// # Complexity
    /// - Time: O(n) - single pass through arrays
    /// - Space: O(n) - frequency array for value counts
    pub fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut cost: i64 = 0;
        let mut conflict_count = 0;
        let mut freq = vec![0_i32; n + 1];
        let mut dominant_val = 0;
        let mut max_freq = 0;

        // Pass 1: Identify conflicts and track value frequencies
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

        // Calculate how many extra positions needed from non-conflict indices
        // If dominant value appears more than half, we can't pair them internally
        let mut excess = 2 * max_freq - conflict_count;

        // Pass 2: Add non-conflict indices if needed
        if excess > 0 {
            for (i, (&v1, &v2)) in nums1.iter().zip(nums2.iter()).enumerate() {
                // Valid non-conflict: different values and neither equals dominant
                if v1 != v2 && v1 != dominant_val && v2 != dominant_val {
                    cost += i as i64;
                    excess -= 1;

                    if excess == 0 {
                        break;
                    }
                }
            }
        }

        // Impossible if we couldn't find enough valid positions
        if excess > 0 {
            return -1;
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_equal_arrays() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let nums2 = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::minimum_total_cost(nums1, nums2), 10);
    }

    #[test]
    fn test_dominant_value_needs_external() {
        let nums1 = vec![2, 2, 2, 1, 3];
        let nums2 = vec![1, 2, 2, 3, 3];
        assert_eq!(Solution::minimum_total_cost(nums1, nums2), 10);
    }

    #[test]
    fn test_impossible_case() {
        let nums1 = vec![1, 2, 2];
        let nums2 = vec![1, 2, 2];
        assert_eq!(Solution::minimum_total_cost(nums1, nums2), -1);
    }

    #[test]
    fn test_no_conflicts() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![3, 1, 2];
        assert_eq!(Solution::minimum_total_cost(nums1, nums2), 0);
    }

    #[test]
    fn test_single_conflict() {
        // Conflict at index 0 (value 1). excess = 2*1 - 1 = 1
        // Need external index where neither value is 1
        // Index 1: nums1=2, nums2=1 -> v2=1=dominant, skip
        // Index 2: nums1=3, nums2=2 -> valid, cost += 2
        // Total: 0 + 2 = 2
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![1, 1, 2];
        assert_eq!(Solution::minimum_total_cost(nums1, nums2), 2);
    }

    #[test]
    fn test_two_conflicts_different_values() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![1, 2, 1];
        assert_eq!(Solution::minimum_total_cost(nums1, nums2), 1);
    }

    #[test]
    fn test_large_dominant() {
        let nums1 = vec![1, 1, 1, 2, 3];
        let nums2 = vec![1, 1, 1, 3, 2];
        // Conflicts at 0,1,2 all with value 1. max_freq=3, cnt=3
        // excess = 2*3 - 3 = 3
        // Non-conflicts: index 3 (nums1=2, nums2=3), index 4 (nums1=3, nums2=2)
        // Need 3 but only have 2 valid -> should be -1
        assert_eq!(Solution::minimum_total_cost(nums1, nums2), -1);
    }
}
