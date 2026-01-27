impl Solution {
    /// Counts minimum rungs to add so each gap is at most dist.
    ///
    /// # Intuition
    /// For each gap between consecutive rungs (or from ground), compute
    /// how many intermediate rungs are needed.
    ///
    /// # Approach
    /// 1. Iterate through rungs, tracking the previous height.
    /// 2. For each rung, add (gap - 1) / dist extra rungs for the gap.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        rungs
            .iter()
            .fold((0, 0), |(count, prev), &rung| {
                (count + (rung - prev - 1) / dist, rung)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::add_rungs(vec![1, 3, 5, 10], 2), 2);
    }

    #[test]
    fn test_no_additions() {
        assert_eq!(Solution::add_rungs(vec![3, 6, 8, 10], 3), 0);
    }

    #[test]
    fn test_large_gap() {
        assert_eq!(Solution::add_rungs(vec![3, 4, 6, 7], 2), 1);
    }
}
