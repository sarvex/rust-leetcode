impl Solution {
    /// Count employees whose hours meet or exceed the target.
    ///
    /// # Intuition
    /// A simple filter-and-count over the hours array.
    ///
    /// # Approach
    /// 1. Filter elements >= target.
    /// 2. Count matching elements.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours.iter().filter(|&&h| h >= target).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_meet_target() {
        assert_eq!(
            Solution::number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2),
            3
        );
    }

    #[test]
    fn none_meet_target() {
        assert_eq!(
            Solution::number_of_employees_who_met_target(vec![1, 1, 1], 5),
            0
        );
    }
}
