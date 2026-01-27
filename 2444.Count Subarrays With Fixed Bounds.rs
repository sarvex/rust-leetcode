impl Solution {
    /// Counts subarrays where the minimum equals min_k and maximum equals max_k.
    ///
    /// # Intuition
    /// Use three pointers: the last position of min_k, last position of max_k,
    /// and last position of an out-of-bounds element. Valid subarrays must start
    /// after the last bad element and after both min_k and max_k have appeared.
    ///
    /// # Approach
    /// 1. Track last occurrence of min_k, max_k, and any out-of-range value
    /// 2. For each position, the number of valid subarrays ending here is
    ///    max(0, min(last_min, last_max) - last_bad)
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1)
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        nums.iter()
            .enumerate()
            .fold(
                (0i64, -1i64, -1i64, -1i64),
                |(res, min_idx, max_idx, bad), (i, &num)| {
                    let i = i as i64;
                    let new_min = if num == min_k { i } else { min_idx };
                    let new_max = if num == max_k { i } else { max_idx };
                    let new_bad = if num < min_k || num > max_k { i } else { bad };
                    (
                        res + (new_min.min(new_max) - new_bad).max(0),
                        new_min,
                        new_max,
                        new_bad,
                    )
                },
            )
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    }

    #[test]
    fn test_no_valid() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 3], 1, 4), 0);
    }

    #[test]
    fn test_single_element_bounds() {
        assert_eq!(Solution::count_subarrays(vec![5], 5, 5), 1);
    }

    #[test]
    fn test_out_of_bounds_reset() {
        assert_eq!(Solution::count_subarrays(vec![1, 5, 100, 1, 5], 1, 5), 1);
    }
}
