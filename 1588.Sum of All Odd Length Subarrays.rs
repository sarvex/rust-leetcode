impl Solution {
    /// DP tracking odd and even length subarray sums ending at each index.
    ///
    /// # Intuition
    /// Maintain two running values: `f[i]` = sum of all odd-length subarrays
    /// ending at `i`, `g[i]` = sum of all even-length subarrays ending at `i`.
    /// Extending an even-length subarray by one element creates an odd-length one.
    ///
    /// # Approach
    /// 1. `f[i] = g[i-1] + arr[i] * (i/2 + 1)` (odd from even + new single)
    /// 2. `g[i] = f[i-1] + arr[i] * ((i+1)/2)` (even from odd)
    /// 3. Sum all `f[i]` values
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the DP arrays
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut f = vec![0; n];
        let mut g = vec![0; n];
        f[0] = arr[0];
        let mut total = f[0];

        for i in 1..n {
            f[i] = g[i - 1] + arr[i] * (i as i32 / 2 + 1);
            g[i] = f[i - 1] + arr[i] * ((i as i32 + 1) / 2);
            total += f[i];
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![2]), 2);
    }

    #[test]
    fn two_elements() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
    }
}
