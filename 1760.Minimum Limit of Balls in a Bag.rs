impl Solution {
    /// Binary search on the answer for minimum bag penalty after splitting.
    ///
    /// # Intuition
    /// Binary search on the maximum bag size. For a given limit, each bag of
    /// size x requires `ceil(x / limit) - 1` operations to split down.
    ///
    /// # Approach
    /// 1. Binary search over `[1, max(nums)]`.
    /// 2. For each midpoint, compute total operations needed.
    /// 3. If operations ≤ budget, search lower; otherwise search higher.
    ///
    /// # Complexity
    /// - Time: O(n log M) where M is the maximum element
    /// - Space: O(1)
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let (mut lo, mut hi) = (1, *nums.iter().max().unwrap());
        while lo < hi {
            let mid = (lo + hi) / 2;
            let ops: i64 = nums.iter().map(|&x| ((x - 1) / mid) as i64).sum();
            if ops <= max_operations as i64 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::minimum_size(vec![9], 2), 3);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::minimum_size(vec![2, 4, 8, 2], 4), 2);
    }

    #[test]
    fn test_no_operations() {
        assert_eq!(Solution::minimum_size(vec![7, 17], 0), 17);
    }
}
