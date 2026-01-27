impl Solution {
    /// Finds the highest altitude reached via prefix sum of gains.
    ///
    /// # Intuition
    /// Starting at altitude 0, each gain value changes the altitude. The answer
    /// is the maximum altitude encountered along the way (including the start).
    ///
    /// # Approach
    /// 1. Fold over gains, tracking current altitude and max altitude.
    /// 2. Return the max altitude.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter()
            .fold((0, 0), |(max_alt, alt), &g| {
                let new_alt = alt + g;
                (max_alt.max(new_alt), new_alt)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
