impl Solution {
    /// Linear scan merging a new interval into a sorted interval list.
    ///
    /// # Intuition
    /// Intervals before the new one (ending before it starts) and after it
    /// (starting after it ends) are kept as-is. Overlapping intervals are
    /// merged by extending the new interval's bounds.
    ///
    /// # Approach
    /// Iterate through intervals. Add all non-overlapping intervals that end
    /// before the new interval. Merge all overlapping intervals by expanding
    /// the new interval's start and end. Add the merged interval, then append
    /// all remaining intervals.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the intervals
    /// - Space: O(n) — result vector
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(intervals.len() + 1);
        let (mut start, mut end) = (new_interval[0], new_interval[1]);
        let mut i = 0;

        while i < intervals.len() && intervals[i][1] < start {
            result.push(intervals[i].clone());
            i += 1;
        }

        while i < intervals.len() && intervals[i][0] <= end {
            start = start.min(intervals[i][0]);
            end = end.max(intervals[i][1]);
            i += 1;
        }
        result.push(vec![start, end]);

        while i < intervals.len() {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_in_middle() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn merge_multiple() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[test]
    fn insert_at_start() {
        assert_eq!(
            Solution::insert(vec![vec![3, 5]], vec![1, 2]),
            vec![vec![1, 2], vec![3, 5]]
        );
    }

    #[test]
    fn empty_intervals() {
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
    }
}
