impl Solution {
    /// Finds minimum operations to make nums1 equal to nums2 with increment/decrement by k.
    ///
    /// # Intuition
    /// Each difference must be divisible by k. The total positive differences must
    /// balance the total negative differences (zero-sum requirement).
    ///
    /// # Approach
    /// 1. If k == 0, arrays must already be identical
    /// 2. Check each difference is divisible by k
    /// 3. Verify positive and negative adjustments balance out
    /// 4. Answer is the total positive adjustment divided by k
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;

        if k == 0 {
            return if nums1.iter().zip(nums2.iter()).all(|(a, b)| a == b) {
                0
            } else {
                -1
            };
        }

        let (sum, ops) =
            nums1
                .iter()
                .zip(nums2.iter())
                .fold((0i64, 0i64), |(sum, ops), (&a, &b)| {
                    let diff = (a as i64) - (b as i64);
                    if diff % k != 0 {
                        (i64::MIN, -1)
                    } else if ops == -1 {
                        (sum, -1)
                    } else {
                        (sum + diff, ops + diff.abs())
                    }
                });

        if ops == -1 || sum != 0 {
            -1
        } else {
            ops / (k * 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::min_operations(vec![4, 3, 1, 4], vec![1, 3, 7, 1], 3),
            2
        );
    }

    #[test]
    fn test_impossible() {
        assert_eq!(
            Solution::min_operations(vec![3, 8, 5, 2], vec![2, 4, 1, 6], 1),
            -1
        );
    }

    #[test]
    fn test_k_zero_equal() {
        assert_eq!(Solution::min_operations(vec![1, 2], vec![1, 2], 0), 0);
    }

    #[test]
    fn test_k_zero_unequal() {
        assert_eq!(Solution::min_operations(vec![1, 2], vec![1, 3], 0), -1);
    }

    #[test]
    fn test_already_equal() {
        assert_eq!(Solution::min_operations(vec![5, 5], vec![5, 5], 3), 0);
    }
}
