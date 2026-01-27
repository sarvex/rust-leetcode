impl Solution {
    /// Counts steps to water all plants, refilling at the start when needed.
    ///
    /// # Intuition
    /// Walk forward plant by plant. If the current water suffices, water
    /// the plant. Otherwise, walk back to refill and return.
    ///
    /// # Approach
    /// 1. Track current water level.
    /// 2. For each plant, either water (1 step) or refill (2*i + 1 steps).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut steps = 0;
        let mut water = capacity;

        for (i, &need) in plants.iter().enumerate() {
            if water >= need {
                water -= need;
                steps += 1;
            } else {
                water = capacity - need;
                steps += (i as i32) * 2 + 1;
            }
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::watering_plants(vec![2, 2, 3, 3], 5), 14);
    }

    #[test]
    fn test_no_refill() {
        assert_eq!(Solution::watering_plants(vec![1, 1, 1], 10), 3);
    }

    #[test]
    fn test_refill_every_time() {
        assert_eq!(Solution::watering_plants(vec![7, 7, 7, 7], 8), 30);
    }
}
