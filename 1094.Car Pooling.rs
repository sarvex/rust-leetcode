
impl Solution {
    /// Checks if all trips can be completed within vehicle capacity.
    ///
    /// # Intuition
    /// Use a difference array to track passenger count changes. The prefix
    /// sum at any point must not exceed capacity.
    ///
    /// # Approach
    /// Build a difference array over the trip range. Compute prefix sums
    /// and verify none exceeds capacity.
    ///
    /// # Complexity
    /// - Time: O(n + max_location)
    /// - Space: O(max_location) for the difference array
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let max_loc = trips.iter().map(|t| t[2]).max().unwrap_or(0) as usize;
        let mut diff = vec![0i32; max_loc + 1];
        for trip in &trips {
            diff[trip[1] as usize] += trip[0];
            diff[trip[2] as usize] -= trip[0];
        }
        diff.iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .all(|load| load <= capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exceeds() {
        assert!(!Solution::car_pooling(
            vec![vec![2, 1, 5], vec![3, 3, 7]],
            4
        ));
    }

    #[test]
    fn test_fits() {
        assert!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5));
    }

    #[test]
    fn test_single_trip() {
        assert!(Solution::car_pooling(vec![vec![2, 1, 5]], 2));
    }
}
