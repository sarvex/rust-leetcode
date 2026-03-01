impl Solution {
    /// Counts subarrays with exactly k distinct integers each appearing at least m
    /// times using a single-window approach with greedy left-boundary expansion.
    ///
    /// # Intuition
    /// Maintain one sliding window `[left, right]` with at most k distinct values.
    /// Within that window, greedily advance a second pointer `trim` that tracks
    /// how far inward from `left` we can trim while every distinct value still
    /// has at least m occurrences. The number of valid starting positions for
    /// a given `right` is `trim - left + 1`.
    ///
    /// # Approach
    /// 1. Use a fixed-size array `freq` for the main window frequency counts and
    ///    `trimmed` for tracking how many copies of each value have been logically
    ///    removed from the left.
    /// 2. Expand `right` one step at a time, updating `freq`, `distinct`, and
    ///    `qualified` (number of values with freq >= m).
    /// 3. Shrink `left` while `distinct > k`, keeping `trimmed` consistent by
    ///    rewinding the trim pointer when `left` overtakes it.
    /// 4. When exactly k distinct values are all qualified, advance `trim` as far
    ///    as possible while each value retains at least m un-trimmed copies.
    /// 5. Accumulate `trim - left + 1` valid subarrays.
    ///
    /// # Complexity
    /// - Time: O(n) — each pointer moves right at most n times total
    /// - Space: O(V) — where V = 100_000 is the maximum element value
    pub fn count_subarrays(nums: Vec<i32>, k: i32, m: i32) -> i64 {
        let m = m as usize;
        let k = k as usize;
        let n = nums.len();

        let mut freq = [0_usize; 100_001];
        let mut trimmed = [0_usize; 100_001];

        let mut distinct = 0_usize;
        let mut qualified = 0_usize;
        let mut result = 0_i64;

        let mut left = 0_usize;
        let mut trim = 0_usize;

        for right in 0..n {
            let val = nums[right] as usize;
            freq[val] += 1;
            if freq[val] == 1 {
                distinct += 1;
            }
            if freq[val] == m {
                qualified += 1;
            }

            // Shrink left to restore at most k distinct
            let mut shrank = false;
            while distinct > k {
                shrank = true;
                let lv = nums[left] as usize;
                freq[lv] -= 1;
                if freq[lv] == 0 {
                    distinct -= 1;
                }
                if freq[lv] == m - 1 {
                    qualified -= 1;
                }
                left += 1;
            }

            // Sync trim pointer: must be >= left
            if trim < left {
                trim = left;
            }

            // If left jumped past trim, rewind trimmed counts
            while shrank && trim != left {
                trimmed[nums[trim] as usize] -= 1;
                trim -= 1;
            }

            // Greedily advance trim while all values keep >= m un-trimmed copies
            if distinct == k && qualified == k {
                while trim < n
                    && freq[nums[trim] as usize] - (trimmed[nums[trim] as usize] + 1) >= m
                {
                    trimmed[nums[trim] as usize] += 1;
                    trim += 1;
                }
                result += (trim - left + 1) as i64;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 1, 2, 2], 2, 2), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_subarrays(vec![3, 1, 2, 4], 2, 1), 3);
    }

    #[test]
    fn test_single_element_valid() {
        assert_eq!(Solution::count_subarrays(vec![1], 1, 1), 1);
    }

    #[test]
    fn test_no_valid_subarray() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 3], 2, 2), 0);
    }

    #[test]
    fn test_all_same_elements() {
        assert_eq!(Solution::count_subarrays(vec![5, 5, 5], 1, 2), 3);
    }

    #[test]
    fn test_k_exceeds_distinct_count() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1], 2, 1), 0);
    }

    #[test]
    fn test_m_exceeds_length() {
        assert_eq!(Solution::count_subarrays(vec![1, 2], 1, 3), 0);
    }

    #[test]
    fn test_bad_elements_present() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 9966], 1, 2), 1);
    }

    #[test]
    fn test_alternating_pattern() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 1, 2, 1, 2], 2, 2), 6);
    }

    #[test]
    fn test_three_distinct_high_m() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 2, 2, 3, 3], 3, 2), 1);
    }
}
