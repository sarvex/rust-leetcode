impl Solution {
    /// Maximize LIS length by selecting a shared set bit.
    ///
    /// # Intuition
    /// A bitwise AND is non-zero iff some bit is set in every element. That means any valid
    /// subsequence must lie entirely in the elements that share at least one common set bit.
    ///
    /// # Approach
    /// Single pass: maintain 30 LIS tails (one per bit; 10^9 < 2^30). For each value, update
    /// only tails for bits set. Fast path: if value > last tail, push without binary search;
    /// else patience-sort replace. Best length over all bits wins.
    ///
    /// # Complexity
    /// - Time: O(n * B * log L) with B = bits set per value; append case O(1) per bit.
    /// - Space: O(n)
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        const MAX_BITS: usize = 30; // 10^9 < 2^30
        let mut tails: [Vec<i32>; MAX_BITS] = std::array::from_fn(|_| Vec::new());

        for &value in &nums {
            let v = value as u32;
            for bit in 0..MAX_BITS {
                if (v >> bit) & 1 == 0 {
                    continue;
                }
                let t = &mut tails[bit];
                if t.last().map_or(true, |&last| last < value) {
                    t.push(value);
                } else {
                    let pos = t.partition_point(|x| *x < value);
                    t[pos] = value;
                }
            }
        }

        tails.iter().map(|t| t.len()).max().unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_subsequence(vec![5, 4, 7]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_subsequence(vec![2, 3, 6]), 3);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_subsequence(vec![0, 1]), 1);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::longest_subsequence(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_duplicates_strictly_increasing() {
        assert_eq!(Solution::longest_subsequence(vec![4, 4, 4]), 1);
    }

    #[test]
    fn test_lis_blocked_by_and() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2);
    }
}
