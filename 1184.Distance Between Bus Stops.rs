impl Solution {
    /// Finds the shortest distance between two bus stops on a circular route.
    ///
    /// # Intuition
    /// The circular route has two paths between any two stops. The shorter
    /// one is `min(clockwise, total - clockwise)`.
    ///
    /// # Approach
    /// Sum the clockwise distance from start to destination. The counter-
    /// clockwise distance is `total - clockwise`. Return the minimum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let total: i32 = distance.iter().sum();
        let n = distance.len();
        let (mut pos, dest) = (start as usize, destination as usize);
        let mut clockwise = 0;
        while pos != dest {
            clockwise += distance[pos];
            pos = (pos + 1) % n;
        }
        clockwise.min(total - clockwise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1),
            1
        );
    }

    #[test]
    fn test_counter_clockwise() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3),
            4
        );
    }

    #[test]
    fn test_same_stop() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 0),
            0
        );
    }
}
