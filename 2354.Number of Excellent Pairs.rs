impl Solution {
    /// Counts excellent pairs where popcount(a|b) + popcount(a&b) >= k.
    ///
    /// # Intuition
    /// `popcount(a | b) + popcount(a & b) = popcount(a) + popcount(b)`.
    /// This simplifies to counting pairs of distinct values whose combined
    /// popcounts meet the threshold.
    ///
    /// # Approach
    /// 1. Deduplicate using HashSet
    /// 2. Count frequency of each popcount value (max 30 for 10^9)
    /// 3. Double loop over popcount frequencies to count valid pairs
    ///
    /// # Complexity
    /// - Time: O(n) for dedup, O(1) for the 31×31 pair counting
    /// - Space: O(n) for the HashSet
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashSet;

        let unique: HashSet<i32> = nums.into_iter().collect();
        let mut cnt = [0i64; 31];

        for num in unique {
            cnt[num.count_ones() as usize] += 1;
        }

        let k = k as usize;
        let mut ans = 0i64;
        for i in 0..31 {
            for j in 0..31 {
                if i + j >= k {
                    ans += cnt[i] * cnt[j];
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 3, 1], 3), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_excellent_pairs(vec![5, 1, 1], 10), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::count_excellent_pairs(vec![7], 6), 1);
    }

    #[test]
    fn test_all_duplicates() {
        assert_eq!(Solution::count_excellent_pairs(vec![3, 3, 3, 3], 4), 1);
    }

    #[test]
    fn test_minimum_k() {
        assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 4, 8], 1), 16);
    }

    #[test]
    fn test_large_k_no_pairs() {
        assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 3, 4, 5], 60), 0);
    }
}
