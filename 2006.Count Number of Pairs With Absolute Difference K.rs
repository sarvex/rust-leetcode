impl Solution {
    /// Counts pairs with absolute difference equal to k.
    ///
    /// # Intuition
    /// For each element, check how many previous elements differ by exactly k
    /// using a frequency count array.
    ///
    /// # Approach
    /// 1. Maintain a frequency array of seen values.
    /// 2. For each element, count occurrences of val-k and val+k.
    /// 3. Increment the frequency of the current value.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(max_val)
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = [0i32; 201];
        let mut count = 0;

        for &val in &nums {
            let v = val as usize;
            if val - k > 0 {
                count += freq[(val - k) as usize];
            }
            if (val + k) <= 200 {
                count += freq[(val + k) as usize];
            }
            freq[v] += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
    }

    #[test]
    fn test_no_pairs() {
        assert_eq!(Solution::count_k_difference(vec![1, 3], 3), 0);
    }

    #[test]
    fn test_large_k() {
        assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
    }
}
