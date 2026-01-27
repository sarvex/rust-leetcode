impl Solution {
    /// Pick k items from ones, zeros, and negative-ones to maximize sum.
    ///
    /// # Intuition
    /// Greedily pick all 1s first, then 0s, then -1s. The sum is determined
    /// by how many -1s we're forced to pick.
    ///
    /// # Approach
    /// 1. If k <= num_ones, sum is k
    /// 2. If k <= num_ones + num_zeros, sum is num_ones
    /// 3. Otherwise, subtract excess picks (which are -1s)
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        _num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        if k <= num_ones {
            k
        } else if k <= num_ones + num_zeros {
            num_ones
        } else {
            num_ones - (k - num_ones - num_zeros)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_only_ones() {
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 2), 2);
    }

    #[test]
    fn test_pick_with_negatives() {
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 3, 7), 2);
    }

    #[test]
    fn test_zero_picks() {
        assert_eq!(Solution::k_items_with_maximum_sum(5, 5, 5, 0), 0);
    }
}
