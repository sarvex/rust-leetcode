impl Solution {
    /// Prefix sum parity counting for odd-sum subarrays.
    ///
    /// # Intuition
    /// A subarray has odd sum when the prefix sums at its endpoints have
    /// different parities. Track the count of even and odd prefix sums
    /// seen so far; for each new prefix sum, the count of valid subarrays
    /// equals the count of prefix sums with opposite parity.
    ///
    /// # Approach
    /// 1. Initialize counts: `[1, 0]` (one even prefix sum of 0)
    /// 2. For each element, update running sum and its parity
    /// 3. Add count of opposite-parity prefix sums to the answer
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut parity_count = [1, 0];
        let mut sum = 0;
        let mut result = 0;

        for &x in &arr {
            sum += x;
            result = (result + parity_count[((sum & 1) ^ 1) as usize]) % MOD;
            parity_count[(sum & 1) as usize] += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
    }

    #[test]
    fn even_elements() {
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
    }

    #[test]
    fn mixed() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    }
}
