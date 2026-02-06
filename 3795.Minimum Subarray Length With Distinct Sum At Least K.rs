impl Solution {
    /// Sliding window with vec-based counts. Expand until distinct sum >= k,
    /// then shrink from the left; track minimum length.
    ///
    /// # Intuition
    /// For a contiguous subarray, "sum of distinct values" increases when we
    /// add a new distinct value and stays unchanged when we add a duplicate.
    /// Shrinking from the left decreases the distinct sum only when we remove
    /// the last occurrence of a value. Use a count array indexed by value
    /// (nums[i] in 1..=10^5) for O(1) updates and no hashing.
    ///
    /// # Approach
    /// 1. Early exits: empty → -1; k <= 1 → 1; any single element >= k → 1.
    /// 2. Count array `cnt[0..=max_val]`; expand with `right`, update distinct
    ///    sum when count goes 0→1; shrink from `left` when sum >= k.
    /// 3. Return minimum length seen, or -1 if none valid.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(max(nums)) for the count array.
    pub fn min_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return -1;
        }
        if k <= 1 {
            return 1;
        }

        let target = k as i64;
        let max_val = nums.iter().copied().max().unwrap() as usize;
        let mut cnt = vec![0u32; max_val + 1];

        let mut distinct_sum: i64 = 0;
        let mut left = 0;
        let mut best = n + 1;

        for (right, &v) in nums.iter().enumerate() {
            let v_i64 = v as i64;
            if v_i64 >= target {
                return 1;
            }

            let idx = v as usize;
            if cnt[idx] == 0 {
                distinct_sum += v_i64;
            }
            cnt[idx] += 1;

            while distinct_sum >= target {
                best = best.min(right - left + 1);
                let u = nums[left] as usize;
                cnt[u] -= 1;
                if cnt[u] == 0 {
                    distinct_sum -= nums[left] as i64;
                }
                left += 1;
            }
        }

        if best > n {
            -1
        } else {
            best as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_length(vec![2, 2, 3, 1], 4), 2);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_length(vec![3, 2, 3, 4], 5), 2);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::min_length(vec![5, 5, 4], 5), 1);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::min_length(vec![1, 2], 10), -1);
    }

    #[test]
    fn test_single_element_sufficient() {
        assert_eq!(Solution::min_length(vec![7], 5), 1);
    }

    #[test]
    fn test_all_distinct() {
        assert_eq!(Solution::min_length(vec![1, 2, 3, 4], 10), 4);
    }
}
