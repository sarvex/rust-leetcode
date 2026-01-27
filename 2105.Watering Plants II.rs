impl Solution {
    /// Counts refills needed when two people water plants from opposite ends.
    ///
    /// # Intuition
    /// Alice starts from the left and Bob from the right. Each waters the next
    /// plant if they have enough capacity; otherwise they refill first. When
    /// they meet at the same plant, the one with more water handles it.
    ///
    /// # Approach
    /// 1. Use two pointers moving inward from both ends.
    /// 2. Track remaining water for Alice and Bob independently.
    /// 3. Refill (increment counter) when current water is insufficient.
    /// 4. Handle the middle plant (odd-length) by choosing the fuller can.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut water_a = capacity_a;
        let mut water_b = capacity_b;
        let mut refills = 0;
        let mut left = 0usize;
        let mut right = plants.len() - 1;

        while left < right {
            if water_a < plants[left] {
                refills += 1;
                water_a = capacity_a;
            }
            water_a -= plants[left];

            if water_b < plants[right] {
                refills += 1;
                water_b = capacity_b;
            }
            water_b -= plants[right];

            left += 1;
            right -= 1;
        }

        if left == right && water_a.max(water_b) < plants[left] {
            refills += 1;
        }

        refills
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 5, 5), 1);
    }

    #[test]
    fn test_no_refill_needed() {
        assert_eq!(Solution::minimum_refill(vec![1, 1, 1, 1], 10, 10), 0);
    }

    #[test]
    fn test_odd_length_middle_plant() {
        assert_eq!(Solution::minimum_refill(vec![2, 2, 5, 2, 2], 5, 5), 1);
    }

    #[test]
    fn test_both_refill_frequently() {
        assert_eq!(Solution::minimum_refill(vec![5, 5, 5, 5], 3, 3), 4);
    }
}
