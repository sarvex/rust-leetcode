impl Solution {
    /// Greedy bit processing with targeted splits.
    ///
    /// # Intuition
    /// To sum to `target`, each bit position needs a sufficient number of `2^i` pieces.
    /// The only way to create missing smaller pieces is to split a larger power of two.
    ///
    /// # Approach
    /// Count how many of each power of two are present. Process bits from low to high,
    /// tracking how many `2^i` pieces are available after accounting for lower-bit
    /// surplus. If a required bit is missing, split the nearest higher power down
    /// to `i`, counting each split as one operation. Any remaining `2^i` pieces can
    /// be paired to contribute to the next bit.
    ///
    /// # Complexity
    /// - Time: O(n + log C) where C is the max value (<= 2^30)
    /// - Space: O(log C) for the counts array
    pub fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
        let total_sum: i64 = nums.iter().map(|&value| value as i64).sum();
        if total_sum < target as i64 {
            return -1;
        }

        let mut counts = vec![0i64; 31];
        for value in nums {
            let bit = (value as u32).trailing_zeros() as usize;
            counts[bit] += 1;
        }

        let mut operations = 0i64;
        let mut carry = 0i64;
        let target_bits = target as u64;

        for i in 0..=30 {
            let mut available = carry + counts[i];
            if ((target_bits >> i) & 1) == 1 {
                if available == 0 {
                    let mut j = i + 1;
                    while j <= 30 && counts[j] == 0 {
                        j += 1;
                    }
                    if j > 30 {
                        return -1;
                    }
                    while j > i {
                        counts[j] -= 1;
                        counts[j - 1] += 2;
                        operations += 1;
                        j -= 1;
                    }
                    available = carry + counts[i];
                }
                available -= 1;
            }
            carry = available / 2;
        }

        operations as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::min_operations(vec![1, 2, 8], 7), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::min_operations(vec![1, 32, 1, 2], 12), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::min_operations(vec![1, 32, 1], 35), -1);
    }

    #[test]
    fn already_possible_no_ops() {
        assert_eq!(Solution::min_operations(vec![1, 2, 4, 8], 11), 0);
    }

    #[test]
    fn needs_multiple_splits() {
        assert_eq!(Solution::min_operations(vec![8, 1], 7), 2);
    }
}
