impl Solution {
    /// Count intervals not covered by any other interval using sort + greedy sweep.
    ///
    /// # Intuition
    /// After sorting by left endpoint (ties broken by right endpoint descending),
    /// an interval is covered if its right endpoint is ≤ the maximum right endpoint
    /// seen so far among all previously processed intervals.
    ///
    /// # Approach
    /// Sort intervals by left endpoint ascending; for equal left endpoints, sort by
    /// right endpoint descending (so a wider interval comes first and subsumes the
    /// narrower one immediately). Then sweep, tracking the running maximum right
    /// endpoint. Any interval whose right endpoint does not exceed `max_right` is
    /// fully covered and can be discarded.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) (sort in-place; result is a counter)
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1]) // wider interval first for equal left endpoints
            }
        });

        let (mut max_right, mut count) = (0, 0);
        for interval in &intervals {
            if interval[1] > max_right {
                max_right = interval[1];
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // [3,6] is covered by [2,8]
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
            2
        );
    }

    #[test]
    fn test_example_2() {
        // [2,3] is covered by [1,4]
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]),
            1
        );
    }

    #[test]
    fn test_no_coverage() {
        // No interval covers another
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 2], vec![3, 4], vec![5, 6]]),
            3
        );
    }

    #[test]
    fn test_single_interval() {
        assert_eq!(Solution::remove_covered_intervals(vec![vec![0, 10]]), 1);
    }

    #[test]
    fn test_all_covered_by_one() {
        assert_eq!(
            Solution::remove_covered_intervals(vec![
                vec![0, 100000],
                vec![1, 2],
                vec![3, 4],
                vec![50000, 99999]
            ]),
            1
        );
    }

    #[test]
    fn test_equal_left_endpoints() {
        // [1,4] covers [1,3]; both have left=1 but [1,4] is wider
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![1, 3]]),
            1
        );
    }
}
