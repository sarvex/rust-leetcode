impl Solution {
    /// Count students active at query time via interval containment check.
    ///
    /// # Intuition
    /// A student is doing homework at `query_time` if their interval
    /// `[start, end]` contains `query_time`.
    ///
    /// # Approach
    /// 1. Zip start and end times
    /// 2. Count pairs where `start <= query_time <= end`
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(end_time.iter())
            .filter(|(s, e)| *s <= query_time && query_time <= *e)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    }

    #[test]
    fn all_active() {
        assert_eq!(
            Solution::busy_student(vec![1, 1, 1], vec![10, 10, 10], 5),
            3
        );
    }

    #[test]
    fn none_active() {
        assert_eq!(Solution::busy_student(vec![1, 2], vec![3, 4], 5), 0);
    }
}
