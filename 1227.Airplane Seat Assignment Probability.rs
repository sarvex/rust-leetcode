impl Solution {
    /// Mathematical observation for the airplane seat probability puzzle.
    ///
    /// # Intuition
    /// For n = 1, the first passenger always sits in their seat (probability 1).
    /// For n ≥ 2, by induction the probability that the last passenger gets
    /// their assigned seat is always exactly 0.5, regardless of n.
    ///
    /// # Approach
    /// 1. Return 1.0 if n = 1
    /// 2. Return 0.5 for all n ≥ 2
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n == 1 {
            1.0
        } else {
            0.5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_passenger() {
        assert!((Solution::nth_person_gets_nth_seat(1) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn two_passengers() {
        assert!((Solution::nth_person_gets_nth_seat(2) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn large_n() {
        assert!((Solution::nth_person_gets_nth_seat(1000) - 0.5).abs() < f64::EPSILON);
    }
}
