use std::collections::HashMap;

impl Solution {
    /// Groups indices by square-free kernel and picks the largest sum.
    ///
    /// # Intuition
    /// For indices `i` and `j`, `i * j` is a perfect square iff their prime exponents have the
    /// same parity. This holds exactly when `i` and `j` share the same square-free kernel.
    ///
    /// # Approach
    /// - Precompute the smallest prime factor for each index.
    /// - For every index, strip square factors to obtain its square-free kernel.
    /// - Sum values by kernel and return the maximum sum.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn maximum_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let smallest_prime_factor = Self::smallest_prime_factors(n);
        let mut sums: HashMap<usize, i64> = HashMap::with_capacity(n);

        for (idx, &value) in nums.iter().enumerate() {
            let index = idx + 1;
            let kernel = Self::square_free_kernel(index, &smallest_prime_factor);
            *sums.entry(kernel).or_insert(0) += value as i64;
        }

        sums.values().copied().max().unwrap_or(0)
    }

    fn smallest_prime_factors(limit: usize) -> Vec<usize> {
        let mut spf: Vec<usize> = (0..=limit).collect();
        if limit >= 1 {
            spf[1] = 1;
        }

        let mut p = 2_usize;
        while p * p <= limit {
            if spf[p] == p {
                let mut multiple = p * p;
                while multiple <= limit {
                    if spf[multiple] == multiple {
                        spf[multiple] = p;
                    }
                    multiple += p;
                }
            }
            p += 1;
        }

        spf
    }

    fn square_free_kernel(mut value: usize, spf: &[usize]) -> usize {
        let mut kernel = 1_usize;
        while value > 1 {
            let prime = spf[value];
            let mut is_odd = false;
            while value % prime == 0 {
                value /= prime;
                is_odd = !is_odd;
            }
            if is_odd {
                kernel *= prime;
            }
        }
        kernel
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let nums = vec![8, 7, 3, 5, 7, 2, 4, 9];
        assert_eq!(Solution::maximum_sum(nums), 16);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![8, 10, 3, 8, 1, 13, 7, 9, 4];
        assert_eq!(Solution::maximum_sum(nums), 20);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        assert_eq!(Solution::maximum_sum(nums), 42);
    }

    #[test]
    fn test_square_index_group() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::maximum_sum(nums), 5);
    }
}
