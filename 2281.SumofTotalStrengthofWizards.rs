impl Solution {
    /// Sum of Total Strength of Wizards
    ///
    /// # Intuition
    /// For each element, find all subarrays where it's the minimum, then efficiently
    /// compute the sum of subarray sums using prefix sums of prefix sums.
    ///
    /// # Approach
    /// 1. Use monotonic stack to find previous smaller element (left boundary) and
    ///    next smaller-or-equal element (right boundary) for each index.
    /// 2. Build prefix sum array and prefix sum of prefix sums for O(1) range queries.
    /// 3. For each element as minimum, compute its contribution using the formula:
    ///    contribution = strength[i] * ((i-L) * sum_of_right_prefixes - (R-i) * sum_of_left_prefixes)
    ///
    /// # Complexity
    /// - Time: O(n) - single pass for monotonic stack, single pass for contributions
    /// - Space: O(n) - for prefix arrays and stack
    pub fn total_strength(strength: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = strength.len();

        if n == 0 {
            return 0;
        }

        // Find previous smaller element index for each position
        let mut left = vec![-1i64; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            while !stack.is_empty() && strength[*stack.last().unwrap()] >= strength[i] {
                stack.pop();
            }
            left[i] = stack.last().map_or(-1, |&x| x as i64);
            stack.push(i);
        }

        // Find next smaller or equal element index for each position
        let mut right = vec![n as i64; n];
        stack.clear();

        for i in (0..n).rev() {
            while !stack.is_empty() && strength[*stack.last().unwrap()] > strength[i] {
                stack.pop();
            }
            right[i] = stack.last().map_or(n as i64, |&x| x as i64);
            stack.push(i);
        }

        // Build prefix sum array: prefix[i] = sum of strength[0..i]
        // prefix has length n+1, prefix[0] = 0
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = (prefix[i] + strength[i] as i64) % MOD;
        }

        // Build prefix sum of prefix sums: prefix_prefix[i] = sum of prefix[0..i]
        // prefix_prefix has length n+2, prefix_prefix[0] = 0
        let mut prefix_prefix = vec![0i64; n + 2];
        for i in 0..=n {
            prefix_prefix[i + 1] = (prefix_prefix[i] + prefix[i]) % MOD;
        }

        let mut result = 0i64;

        // For each element, compute its contribution as the minimum
        for i in 0..n {
            let l = left[i]; // exclusive left boundary
            let r = right[i]; // exclusive right boundary

            // Number of choices for left endpoint: (l+1, l+2, ..., i) => i - l choices
            // Number of choices for right endpoint: (i, i+1, ..., r-1) => r - i choices

            let left_count = i as i64 - l;
            let right_count = r - i as i64;

            // Sum of prefix[i+1..=r] = prefix_prefix[r+1] - prefix_prefix[i+1]
            let right_prefix_sum =
                (prefix_prefix[(r + 1) as usize] - prefix_prefix[i + 1] + MOD) % MOD;

            // Sum of prefix[l+1..=i] = prefix_prefix[i+1] - prefix_prefix[(l+1)]
            let left_prefix_sum =
                (prefix_prefix[i + 1] - prefix_prefix[(l + 1) as usize] + MOD) % MOD;

            // Contribution formula:
            // strength[i] * (left_count * right_prefix_sum - right_count * left_prefix_sum)
            let positive_term = (left_count % MOD * right_prefix_sum) % MOD;
            let negative_term = (right_count % MOD * left_prefix_sum) % MOD;

            let subarray_sum_contribution = (positive_term - negative_term % MOD + MOD) % MOD;
            let contribution = (strength[i] as i64 * subarray_sum_contribution) % MOD;

            result = (result + contribution) % MOD;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // [1,3,1,2] => 44
        let strength = vec![1, 3, 1, 2];
        assert_eq!(Solution::total_strength(strength), 44);
    }

    #[test]
    fn test_example_2() {
        // [5,4,6] => 213
        let strength = vec![5, 4, 6];
        assert_eq!(Solution::total_strength(strength), 213);
    }

    #[test]
    fn test_single_element() {
        let strength = vec![7];
        assert_eq!(Solution::total_strength(strength), 49);
    }

    #[test]
    fn test_two_elements_ascending() {
        // [1,2]: [1]=1, [2]=4, [1,2]=1*3=3 => 8
        let strength = vec![1, 2];
        assert_eq!(Solution::total_strength(strength), 8);
    }

    #[test]
    fn test_two_elements_descending() {
        // [2,1]: [2]=4, [1]=1, [2,1]=1*3=3 => 8
        let strength = vec![2, 1];
        assert_eq!(Solution::total_strength(strength), 8);
    }

    #[test]
    fn test_all_same() {
        // [3,3,3]: [3]=9 x3, [3,3]=3*6=18 x2, [3,3,3]=3*9=27 => 27+36+27=90
        let strength = vec![3, 3, 3];
        assert_eq!(Solution::total_strength(strength), 90);
    }

    #[test]
    fn test_increasing() {
        // [1,2,3]: [1]=1, [2]=4, [3]=9, [1,2]=1*3=3, [2,3]=2*5=10, [1,2,3]=1*6=6
        // Total: 1+4+9+3+10+6=33
        let strength = vec![1, 2, 3];
        assert_eq!(Solution::total_strength(strength), 33);
    }

    #[test]
    fn test_decreasing() {
        // [3,2,1]: [3]=9, [2]=4, [1]=1, [3,2]=2*5=10, [2,1]=1*3=3, [3,2,1]=1*6=6
        // Total: 9+4+1+10+3+6=33
        let strength = vec![3, 2, 1];
        assert_eq!(Solution::total_strength(strength), 33);
    }

    #[test]
    fn test_large_values() {
        // Test modular arithmetic with larger values
        let strength = vec![1_000_000_000, 1_000_000_000];
        let result = Solution::total_strength(strength);
        assert!(result >= 0);
    }
}
