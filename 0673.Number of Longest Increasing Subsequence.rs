impl Solution {
    /// Counts the number of longest increasing subsequences using DP.
    ///
    /// # Intuition
    /// Track both the LIS length and its count ending at each index. When a
    /// longer subsequence is found, reset the count; when an equal-length one
    /// is found, accumulate counts.
    ///
    /// # Approach
    /// 1. For each i, scan j < i where nums[j] < nums[i].
    /// 2. Update f[i] (length) and cnt[i] (count) accordingly.
    /// 3. Track the global max length and sum counts of that length.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n)
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f = vec![1; n];
        let mut cnt = vec![1; n];
        let mut max_len = 0;
        let mut result = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if f[i] < f[j] + 1 {
                        f[i] = f[j] + 1;
                        cnt[i] = cnt[j];
                    } else if f[i] == f[j] + 1 {
                        cnt[i] += cnt[j];
                    }
                }
            }
            match f[i].cmp(&max_len) {
                std::cmp::Ordering::Greater => {
                    max_len = f[i];
                    result = cnt[i];
                }
                std::cmp::Ordering::Equal => {
                    result += cnt[i];
                }
                _ => {}
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::find_number_of_lis(vec![1]), 1);
    }
}
