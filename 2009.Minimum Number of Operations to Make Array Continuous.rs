impl Solution {
    /// Finds minimum operations to make the array continuous.
    ///
    /// # Intuition
    /// After deduplication and sorting, find the window of size n that
    /// contains the most elements. Elements outside this window need replacement.
    ///
    /// # Approach
    /// 1. Deduplicate and sort using a BTreeSet.
    /// 2. For each starting element, binary search for the upper bound of
    ///    the n-wide range.
    /// 3. The answer is n minus the maximum window coverage.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sorted: Vec<i32> = {
            let mut set = std::collections::BTreeSet::new();
            for &x in &nums {
                set.insert(x);
            }
            set.into_iter().collect()
        };
        let m = sorted.len();

        let mut best = 0;
        for i in 0..m {
            let target = sorted[i] + n as i32;
            let j = sorted.partition_point(|&x| x < target);
            best = best.max(j - i);
        }

        (n - best) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_operations(vec![4, 2, 5, 3]), 0);
    }

    #[test]
    fn test_with_gaps() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3, 5, 6]), 1);
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(Solution::min_operations(vec![1, 10, 100, 1000]), 3);
    }
}
