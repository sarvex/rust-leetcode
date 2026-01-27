impl Solution {
    /// Checks if array can be partitioned into three equal-sum parts.
    ///
    /// # Intuition
    /// If the total sum is divisible by 3, scan for prefix sums equaling
    /// one-third and two-thirds.
    ///
    /// # Approach
    /// Compute total. If not divisible by 3, return false. Scan accumulating
    /// sum; count how many times the running sum equals `sum/3`. Need at
    /// least 3 hits.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let total: i32 = arr.iter().sum();
        if total % 3 != 0 {
            return false;
        }
        let target = total / 3;
        let mut parts = 0;
        let mut running = 0;
        for &x in &arr {
            running += x;
            if running == target {
                parts += 1;
                running = 0;
            }
        }
        parts >= 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible() {
        assert!(Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1
        ]));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1
        ]));
    }

    #[test]
    fn test_all_zeros() {
        assert!(Solution::can_three_parts_equal_sum(vec![0, 0, 0, 0]));
    }
}
