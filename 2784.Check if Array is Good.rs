pub struct Solution;

impl Solution {
    /// Check if array is a permutation of base[n] using frequency counting.
    ///
    /// # Intuition
    /// base[n] contains exactly 1..=n-1 once each, plus n twice. The maximum
    /// element must equal n, and the array length must be n+1. We can verify
    /// this in a single pass using a frequency array.
    ///
    /// # Approach
    /// 1. Find the maximum element n — this is the only valid candidate.
    /// 2. Check length equals n+1.
    /// 3. Count frequencies; every value 1..=n-1 must appear exactly once,
    ///    and n must appear exactly twice.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = *nums.iter().max().unwrap_or(&0) as usize;

        if nums.len() != n + 1 {
            return false;
        }

        let mut freq = vec![0u8; n + 1];
        for &x in &nums {
            let x = x as usize;
            if x > n {
                return false;
            }
            freq[x] += 1;
        }

        freq[n] == 2 && freq[1..n].iter().all(|&c| c == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_false_wrong_length() {
        assert!(!Solution::is_good(vec![2, 1, 3]));
    }

    #[test]
    fn test_example_2_true_permutation() {
        assert!(Solution::is_good(vec![1, 3, 3, 2]));
    }

    #[test]
    fn test_example_3_true_base_1() {
        assert!(Solution::is_good(vec![1, 1]));
    }

    #[test]
    fn test_example_4_false_wrong_length() {
        assert!(!Solution::is_good(vec![3, 4, 4, 1, 2, 1]));
    }

    #[test]
    fn test_edge_missing_element() {
        // [2, 2] — max is 2, length is 2 (need 3), so false
        assert!(!Solution::is_good(vec![2, 2]));
    }

    #[test]
    fn test_edge_duplicate_non_max() {
        // [1, 1, 2] — max is 2, length 3 ok, but 1 appears twice and 2 once
        assert!(!Solution::is_good(vec![1, 1, 2]));
    }

    #[test]
    fn test_edge_single_element() {
        // length 1 can never be n+1 >= 2
        assert!(!Solution::is_good(vec![1]));
    }

    #[test]
    fn test_large_valid() {
        let mut nums: Vec<i32> = (1..=4).collect();
        nums.push(4);
        assert!(Solution::is_good(nums));
    }
}
