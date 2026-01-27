impl Solution {
    /// Minimizes total moves to seat all students by sorting and pairing.
    ///
    /// # Intuition
    /// Sorting both arrays and pairing by index minimizes total displacement.
    ///
    /// # Approach
    /// 1. Sort seats and students.
    /// 2. Sum absolute differences of paired elements.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();
        seats
            .iter()
            .zip(students.iter())
            .map(|(&s, &t)| (s - t).abs())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }

    #[test]
    fn test_already_seated() {
        assert_eq!(Solution::min_moves_to_seat(vec![1, 2, 3], vec![1, 2, 3]), 0);
    }
}
