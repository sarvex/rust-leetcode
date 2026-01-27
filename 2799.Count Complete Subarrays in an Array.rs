use std::collections::HashSet;

impl Solution {
    /// Count subarrays containing all distinct elements of the array.
    ///
    /// # Intuition
    /// A subarray is complete when it contains every distinct value. For each
    /// starting index, find the earliest ending index achieving completeness;
    /// all extensions beyond that point are also complete.
    ///
    /// # Approach
    /// 1. Count total distinct elements `total`.
    /// 2. For each start index, expand until distinct count matches `total`.
    /// 3. Add `n - end` to the answer (all further extensions are also complete).
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n)
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let total = nums.iter().collect::<HashSet<_>>().len();
        let n = nums.len();
        let mut ans = 0i32;
        for i in 0..n {
            let mut seen = HashSet::new();
            for j in i..n {
                seen.insert(nums[j]);
                if seen.len() == total {
                    ans += (n - j) as i32;
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_complete_subarrays() {
        assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
    }

    #[test]
    fn single_distinct_element() {
        assert_eq!(Solution::count_complete_subarrays(vec![5, 5, 5]), 6);
    }
}
