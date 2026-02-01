impl Solution {
    /// Counts subarrays by compressing distinct AND values per suffix.
    ///
    /// # Intuition
    /// For any fixed end index, extending a subarray to the left can only clear bits in the
    /// bitwise AND, so the number of distinct AND results is small (bounded by the bit width).
    ///
    /// # Approach
    /// Maintain a list of pairs `(and_value, count)` for all subarrays ending at the previous
    /// index, with unique AND values in non-increasing order. For each new element `num`, build
    /// the next list by AND-ing `num` with every previous value, merging identical results that
    /// become adjacent. Add up the counts where the AND equals `k`.
    ///
    /// # Complexity
    /// - Time: O(n * b), where `b` is the number of bits (<= 31)
    /// - Space: O(b)
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut result = 0i64;
        let mut prev: Vec<(i32, i64)> = Vec::new();

        for &num in &nums {
            let mut next: Vec<(i32, i64)> = Vec::with_capacity(prev.len() + 1);
            next.push((num, 1));

            for &(value, count) in &prev {
                let merged = value & num;
                if let Some(last) = next.last_mut() {
                    if last.0 == merged {
                        last.1 += count;
                        continue;
                    }
                }
                next.push((merged, count));
            }

            for &(value, count) in &next {
                if value == k {
                    result += count;
                }
            }

            prev = next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 1, 1];
        assert_eq!(Solution::count_subarrays(nums, 1), 6);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 2];
        assert_eq!(Solution::count_subarrays(nums, 1), 3);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::count_subarrays(nums, 2), 2);
    }

    #[test]
    fn test_single_element_match() {
        let nums = vec![5];
        assert_eq!(Solution::count_subarrays(nums, 5), 1);
    }

    #[test]
    fn test_single_element_no_match() {
        let nums = vec![5];
        assert_eq!(Solution::count_subarrays(nums, 1), 0);
    }

    #[test]
    fn test_all_zeroes() {
        let nums = vec![0, 0, 0];
        assert_eq!(Solution::count_subarrays(nums, 0), 6);
    }
}
