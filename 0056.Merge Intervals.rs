impl Solution {
    /// Sort-then-merge for overlapping interval consolidation.
    ///
    /// # Intuition
    /// Sorting intervals by start time ensures that overlapping intervals
    /// are adjacent. A single pass can then merge them by extending the
    /// current interval's end whenever overlap is detected.
    ///
    /// # Approach
    /// Sort by start value. Initialize with the first interval. For each
    /// subsequent interval, if it overlaps with the last merged interval
    /// (start <= previous end), extend the end. Otherwise, push a new
    /// merged interval.
    ///
    /// # Complexity
    /// - Time: O(n log n) — dominated by sorting
    /// - Space: O(log n) — sorting stack (excluding output)
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|interval| interval[0]);
        let mut merged: Vec<Vec<i32>> = Vec::new();

        for interval in intervals {
            if let Some(last) = merged.last_mut() {
                if interval[0] <= last[1] {
                    last[1] = last[1].max(interval[1]);
                    continue;
                }
            }
            merged.push(interval);
        }

        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_intervals() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn fully_overlapping() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }

    #[test]
    fn single_interval() {
        assert_eq!(Solution::merge(vec![vec![1, 1]]), vec![vec![1, 1]]);
    }

    #[test]
    fn no_overlap() {
        assert_eq!(
            Solution::merge(vec![vec![1, 2], vec![4, 5]]),
            vec![vec![1, 2], vec![4, 5]]
        );
    }
}
