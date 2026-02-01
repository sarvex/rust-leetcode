impl Solution {
    /// Finds the median distinct-count across all subarrays.
    ///
    /// # Intuition
    /// The median is the smallest value `k` such that at least half of all subarrays have
    /// at most `k` distinct elements. Counting subarrays with at most `k` distinct elements
    /// can be done with a sliding window.
    ///
    /// # Approach
    /// - Let `total = n * (n + 1) / 2` and `target = (total + 1) / 2` (lower median).
    /// - Binary search `k` in `[1, distinct_total]`.
    /// - For each `k`, count subarrays with at most `k` distinct elements using a two-pointer
    ///   window that maintains frequencies and the current distinct count.
    /// - Return the smallest `k` whose count reaches `target`.
    ///
    /// # Complexity
    /// - Time: O((n + V) log D), where `V` is the max value and `D` is distinct count.
    /// - Space: O(V)
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let max_value = nums.iter().copied().max().unwrap_or(0).max(0) as usize;
        let distinct_total = count_distinct_total(&nums, max_value);
        let total_subarrays = (n as i64) * (n as i64 + 1) / 2;
        let target = (total_subarrays + 1) / 2;

        let mut low = 1usize;
        let mut high = distinct_total.max(1);
        while low < high {
            let mid = (low + high) / 2;
            let count = count_at_most_distinct(&nums, max_value, mid);
            if count >= target {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low as i32
    }
}

fn count_distinct_total(nums: &[i32], max_value: usize) -> usize {
    let mut seen = vec![false; max_value + 1];
    let mut distinct = 0usize;
    for &value in nums {
        let index = value as usize;
        if !seen[index] {
            seen[index] = true;
            distinct += 1;
        }
    }
    distinct
}

fn count_at_most_distinct(nums: &[i32], max_value: usize, limit: usize) -> i64 {
    if limit == 0 {
        return 0;
    }

    let mut freq = vec![0u32; max_value + 1];
    let mut distinct = 0usize;
    let mut left = 0usize;
    let mut total = 0i64;

    for right in 0..nums.len() {
        let idx = nums[right] as usize;
        if freq[idx] == 0 {
            distinct += 1;
        }
        freq[idx] += 1;

        while distinct > limit {
            let left_idx = nums[left] as usize;
            freq[left_idx] -= 1;
            if freq[left_idx] == 0 {
                distinct -= 1;
            }
            left += 1;
        }

        total += (right - left + 1) as i64;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::median_of_uniqueness_array(nums), 1);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 4, 3, 4, 5];
        assert_eq!(Solution::median_of_uniqueness_array(nums), 2);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![4, 3, 5, 4];
        assert_eq!(Solution::median_of_uniqueness_array(nums), 2);
    }

    #[test]
    fn test_all_same() {
        let nums = vec![7, 7, 7, 7];
        assert_eq!(Solution::median_of_uniqueness_array(nums), 1);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        assert_eq!(Solution::median_of_uniqueness_array(nums), 1);
    }
}
