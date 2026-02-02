impl Solution {
    /// Greedily concentrates available bits into k numbers.
    ///
    /// # Intuition
    /// The AND/OR operation preserves the count of set bits at every position across the array.
    /// With these counts fixed, the sum of squares is maximized by concentrating 1s into as few
    /// numbers as possible (convexity of x^2).
    ///
    /// # Approach
    /// Count set bits for positions 0..30 using trailing-zeros iteration. Then, for each of the
    /// k resulting numbers, build a value by taking one available 1 from every bit position that
    /// still has remaining counts. Square and sum these k values modulo 1e9+7.
    ///
    /// # Complexity
    /// - Time: O(n * popcount + k * B), where B = 31
    /// - Space: O(B)
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut bit_counts = [0_u32; 31];
        for num in nums {
            let mut value = num as u32;
            while value != 0 {
                let bit = value.trailing_zeros() as usize;
                bit_counts[bit] += 1;
                value &= value - 1;
            }
        }

        let mut result = 0_i64;
        let k = k as usize;
        let mut active_bits: Vec<usize> = (0..31).filter(|&bit| bit_counts[bit] > 0).collect();
        let mut next_bits = Vec::with_capacity(active_bits.len());
        for _ in 0..k {
            if active_bits.is_empty() {
                break;
            }
            let mut value = 0_i64;
            next_bits.clear();
            for &bit in &active_bits {
                bit_counts[bit] -= 1;
                value |= 1_i64 << bit;
                if bit_counts[bit] > 0 {
                    next_bits.push(bit);
                }
            }
            std::mem::swap(&mut active_bits, &mut next_bits);
            result = (result + (value * value) % MOD) % MOD;
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
