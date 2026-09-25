impl Solution {
    /// Count subarray products by remainder modulo `k`.
    ///
    /// # Intuition
    /// Each allowed removal of a non-overlapping prefix and suffix leaves one
    /// contiguous subarray. Extending every subarray that ends at the previous
    /// index by `nums[i]` produces every subarray that ends at `i`, so product
    /// remainders can be updated from the previous index alone. `k` is at most 5,
    /// so those remainders fit in a tiny frequency table.
    ///
    /// # Approach
    /// Keep the frequency of product remainders for subarrays ending at the
    /// previous index. For each value, let `m = nums[i] % k`. Map every previous
    /// remainder `r` to `(r * m) % k`, then add the singleton subarray `[nums[i]]`
    /// at remainder `m`. Fold those frequencies into the answer and advance.
    ///
    /// # Complexity
    /// - Time: O(n · k)
    /// - Space: O(k)
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;
        let mut result = vec![0_i64; k];
        let mut ending = vec![0_i64; k];
        let mut next = vec![0_i64; k];

        for num in nums {
            let residue = (num as usize) % k;
            next.fill(0);
            for (prev, count) in ending.iter().enumerate() {
                next[(prev * residue) % k] += count;
            }
            next[residue] += 1;
            result
                .iter_mut()
                .zip(next.iter())
                .for_each(|(total, count)| *total += count);
            std::mem::swap(&mut ending, &mut next);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute(nums: &[i32], k: i32) -> Vec<i64> {
        let modulus = k as i64;
        let mut result = vec![0_i64; k as usize];
        for start in 0..nums.len() {
            let mut product = 1_i64;
            for &num in &nums[start..] {
                product = product * (num as i64 % modulus) % modulus;
                result[product as usize] += 1;
            }
        }
        result
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::result_array(vec![1, 2, 3, 4, 5], 3),
            vec![9, 2, 4]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4),
            vec![18, 1, 2, 0]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::result_array(vec![1, 1, 2, 1, 1], 2), vec![9, 6]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::result_array(vec![7], 5), vec![0, 0, 1, 0, 0]);
        assert_eq!(Solution::result_array(vec![10], 5), vec![1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_k_is_one() {
        let nums = vec![1, 2, 3, 4];
        let n = nums.len() as i64;
        assert_eq!(Solution::result_array(nums, 1), vec![n * (n + 1) / 2]);
    }

    #[test]
    fn test_all_multiples_of_k() {
        assert_eq!(Solution::result_array(vec![3, 6, 9], 3), vec![6, 0, 0]);
    }

    #[test]
    fn test_matches_brute_force() {
        let cases = [
            (vec![1, 2, 3, 4, 5], 3),
            (vec![1, 2, 4, 8, 16, 32], 4),
            (vec![1, 1, 2, 1, 1], 2),
            (vec![5], 5),
            (vec![1, 1, 1, 1], 5),
            (vec![2, 3, 5, 7, 11], 4),
            (vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 5),
            (vec![1_000_000_000, 999_999_937, 4], 5),
        ];
        for (nums, k) in cases {
            assert_eq!(Solution::result_array(nums.clone(), k), brute(&nums, k));
        }
    }

    #[test]
    fn test_large_all_ones() {
        let n = 100_000_i64;
        let mut expected = vec![0_i64; 5];
        expected[1] = n * (n + 1) / 2;
        assert_eq!(Solution::result_array(vec![1; n as usize], 5), expected);
    }
}
