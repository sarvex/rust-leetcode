impl Solution {
    /// Minimum Swaps to Avoid Forbidden Values
    ///
    /// # Intuition
    /// Bad indices can pair to fix both if they have different values. The bottleneck
    /// is the most frequent value among bad indices - it needs positions that don't
    /// forbid it.
    ///
    /// # Approach
    /// 1. Use Boyer-Moore to find candidate majority value among bad indices
    /// 2. Count positions where this value can go (value differs AND not forbidden)
    /// 3. If not enough positions, return -1
    /// 4. Answer is max(majority_count, ceil(total_bad / 2))
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_swaps(nums: Vec<i32>, forbidden: Vec<i32>) -> i32 {
        use std::cmp::max;

        let mut cand_n: i32 = 0;
        let mut n_cnt: i32 = 0;
        let mut all_forb: i32 = 0;

        // Boyer-Moore majority vote among bad indices
        for (&n, &f) in nums.iter().zip(forbidden.iter()) {
            if n == f {
                all_forb += 1;

                if n == cand_n {
                    n_cnt += 1;
                } else if n_cnt != 0 {
                    n_cnt -= 1;
                } else {
                    cand_n = n;
                    n_cnt = 1;
                }
            }
        }

        if all_forb == 0 {
            return 0;
        }

        // Count actual frequency and available positions for candidate
        n_cnt = 0;
        let mut not_n: i32 = 0;
        for (&n, &f) in nums.iter().zip(forbidden.iter()) {
            if n == cand_n && n == f {
                n_cnt += 1;
            } else if n != cand_n && cand_n != f {
                not_n += 1;
            }
        }

        if n_cnt > not_n {
            -1
        } else {
            max(n_cnt, (all_forb + 1) / 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_swaps(vec![1, 2, 3], vec![3, 2, 1]), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_swaps(vec![4, 6, 6, 5], vec![4, 6, 5, 5]), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_swaps(vec![7, 7], vec![8, 7]), -1);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(Solution::min_swaps(vec![1, 2], vec![2, 1]), 0);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(Solution::min_swaps(vec![9, 1], vec![1, 1]), -1);
    }

    #[test]
    fn test_example_6() {
        assert_eq!(Solution::min_swaps(vec![5, 12, 12], vec![5, 12, 12]), -1);
    }

    #[test]
    fn test_example_7() {
        assert_eq!(
            Solution::min_swaps(
                vec![104, 13, 82, 116, 72, 43, 101, 45, 6, 6, 31],
                vec![10, 13, 82, 116, 14, 73, 85, 45, 6, 6, 36]
            ),
            3
        );
    }
}
