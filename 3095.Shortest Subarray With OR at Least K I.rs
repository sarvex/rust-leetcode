impl Solution {
    /// Shortest subarray whose bitwise OR is at least k.
    ///
    /// # Intuition
    /// Use a sliding window with bit counting to track the OR value. When removing
    /// an element from the left, decrement per-bit counters; if a counter reaches
    /// zero, that bit drops from the aggregate OR.
    ///
    /// # Approach
    /// 1. Maintain a 32-element bit counter array and running OR value.
    /// 2. Expand the window to the right, updating counters and OR.
    /// 3. While OR >= k, record the window length and shrink from the left.
    /// 4. Return the minimum length found, or -1 if none qualifies.
    ///
    /// # Complexity
    /// - Time: O(32 * n) = O(n)
    /// - Space: O(1)
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut cnt = [0i32; 32];
        let mut ans = n as i32 + 1;
        let mut s = 0;
        let mut left = 0;

        for (right, &x) in nums.iter().enumerate() {
            s |= x;
            (0..32)
                .filter(|&h| (x >> h) & 1 == 1)
                .for_each(|h| cnt[h] += 1);

            while s >= k && left <= right {
                ans = ans.min((right - left + 1) as i32);
                let y = nums[left];
                (0..32).filter(|&h| (y >> h) & 1 == 1).for_each(|h| {
                    cnt[h] -= 1;
                    if cnt[h] == 0 {
                        s ^= 1 << h;
                    }
                });
                left += 1;
            }
        }

        match ans > n as i32 {
            true => -1,
            false => ans,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarray_exists() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
    }

    #[test]
    fn entire_array_needed() {
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
    }

    #[test]
    fn no_valid_subarray() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
