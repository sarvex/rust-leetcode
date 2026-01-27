impl Solution {
    /// Determines if a person can attend all meetings by checking for overlaps.
    ///
    /// # Intuition
    /// Sort meetings by start time. If any meeting starts before the previous
    /// one ends, there is a conflict.
    ///
    /// # Approach
    /// 1. Sort intervals by start time.
    /// 2. Check consecutive pairs for overlap.
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(1) excluding sort
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by_key(|i| i[0]);
        intervals.windows(2).all(|w| w[0][1] <= w[1][0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_meetings() {
        assert!(!Solution::can_attend_meetings(vec![
            vec![0, 30],
            vec![5, 10],
            vec![15, 20],
        ]));
    }

    #[test]
    fn no_overlap() {
        assert!(Solution::can_attend_meetings(
            vec![vec![7, 10], vec![2, 4],]
        ));
    }

    #[test]
    fn single_meeting() {
        assert!(Solution::can_attend_meetings(vec![vec![1, 5]]));
    }
}
