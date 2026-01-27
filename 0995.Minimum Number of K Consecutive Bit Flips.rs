impl Solution {
    /// Finds minimum k-length flips to make all bits 1 using a difference array.
    ///
    /// # Intuition
    /// Track the cumulative number of flips affecting each position. If the
    /// effective value at position i is 0, a flip starting at i is required.
    ///
    /// # Approach
    /// Use a difference array to track flip effects. Scan left-to-right;
    /// when a position needs flipping, increment the difference and count.
    /// If a flip would extend beyond the array, return -1.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the difference array
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut diff = vec![0; n + 1];
        let mut flips = 0;
        let mut cumulative = 0;

        for i in 0..n {
            cumulative += diff[i];
            if (cumulative + nums[i]) % 2 == 0 {
                if i + k > n {
                    return -1;
                }
                diff[i] += 1;
                diff[i + k] -= 1;
                cumulative += 1;
                flips += 1;
            }
        }
        flips
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
    }

    #[test]
    fn test_longer() {
        assert_eq!(
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
            3
        );
    }
}
