impl Solution {
    /// Greedily concentrates available bits into k numbers.
    ///
    /// # Intuition
    /// The AND/OR operation preserves the count of set bits at every position across the array.
    /// With these counts fixed, the sum of squares is maximized by concentrating 1s into as few
    /// numbers as possible (convexity of x^2).
    ///
    /// # Approach
    /// Count set bits for positions 0..30. For each of the k resulting numbers, build a value by
    /// taking one available 1 from every bit position that still has remaining counts. Square and
    /// sum these k values modulo 1e9+7.
    ///
    /// # Complexity
    /// - Time: O(n * B + k * B), where B = 31
    /// - Space: O(B)
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut bit_counts = [0_i32; 31];

        nums.iter().for_each(|&num| {
            (0..31).for_each(|bit| {
                if (num >> bit) & 1 == 1 {
                    bit_counts[bit] += 1;
                }
            });
        });

        let mut result = 0_i64;
        let k = k as usize;
        for _ in 0..k {
            let mut value = 0_i64;
            (0..31).for_each(|bit| {
                if bit_counts[bit] > 0 {
                    bit_counts[bit] -= 1;
                    value |= 1_i64 << bit;
                }
            });
            let square = (value % MOD) * (value % MOD) % MOD;
            result = (result + square) % MOD;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_sum(vec![2, 6, 5, 8], 2), 261);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_sum(vec![4, 5, 4, 7], 3), 90);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_sum(vec![9], 1), 81);
    }

    #[test]
    fn test_all_bits_concentrated() {
        assert_eq!(Solution::max_sum(vec![1, 1, 1], 2), 5);
    }
}
