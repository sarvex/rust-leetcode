impl Solution {
    /// Finds the longest chain of pairs using a greedy interval scheduling approach.
    ///
    /// # Intuition
    /// Sorting by the second element and greedily selecting non-overlapping
    /// pairs maximizes the chain length — identical to the activity selection problem.
    ///
    /// # Approach
    /// 1. Sort pairs by their right endpoint.
    /// 2. Greedily pick each pair whose left endpoint exceeds the previous right.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) in-place sort
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|p| p[1]);
        let mut count = 0;
        let mut prev_end = i32::MIN;
        for pair in &pairs {
            if pair[0] > prev_end {
                count += 1;
                prev_end = pair[1];
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        );
    }

    #[test]
    fn test_no_overlap() {
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
            3
        );
    }
}
