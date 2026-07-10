use std::collections::HashMap;

impl Solution {
    /// Frequency map + chain extension: for each base x walk x → x² → x⁴ → …
    ///
    /// # Intuition
    /// A valid subset forms the palindrome [x, x², x⁴, …, xᵏ, …, x⁴, x², x].
    /// Every level except the topmost centre appears on both sides (2 copies needed).
    /// The centre appears exactly once.  Total length is always odd.
    ///
    /// For a starting value `x` walk the squaring chain while `freq[cur] ≥ 2`,
    /// adding 2 per level.  When the chain can no longer grow:
    /// - If `freq[cur] ≥ 1` → `cur` serves as the centre, add 1 (length becomes odd).
    /// - If `freq[cur] = 0` → no centre; drop the last pair to restore odd length
    ///   (`max(chain_len - 1, 1)`).
    ///
    /// When `cur²` overflows or exceeds 10⁹, the next level cannot exist in nums.
    /// `cur` has already been added as a pair (`+2`); the chain simply ends and the
    /// dropping rule above restores a valid odd length.
    ///
    /// Special case `x = 1`: squaring stays at 1; best valid length = largest odd ≤ freq[1].
    ///
    /// # Approach
    /// 1. Build frequency map with `with_capacity`.
    /// 2. Handle `x = 1` separately.
    /// 3. For each other unique value, walk the chain with the logic above.
    /// 4. Return the global maximum (at least 1).
    ///
    /// # Complexity
    /// - Time: O(n · log(max_val))  — chain depth ≤ log₂(10⁹) ≈ 30
    /// - Space: O(n)
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut freq: HashMap<i64, i64> = HashMap::with_capacity(nums.len());
        for &n in &nums {
            *freq.entry(n as i64).or_insert(0) += 1;
        }

        let mut best: i64 = 1;

        for (&base, &cnt) in &freq {
            if base == 1 {
                // Squaring 1 stays at 1; pick the largest odd count ≤ freq[1].
                let len = if cnt % 2 == 0 { cnt - 1 } else { cnt };
                best = best.max(len);
                continue;
            }

            let mut chain_len: i64 = 0;
            let mut cur = base;

            loop {
                let c = freq.get(&cur).copied().unwrap_or(0);

                if c < 2 {
                    // cur cannot form a pair; use it as the centre if present.
                    if c >= 1 {
                        chain_len += 1;
                    } else {
                        // No centre available — drop the outermost pair.
                        chain_len = (chain_len - 1).max(1);
                    }
                    break;
                }

                // cur has freq ≥ 2: pair level (contributes to both sides).
                chain_len += 2;

                // Advance to cur². If unreachable, the pair is already counted;
                // the loop will exit next iteration when c=0 triggers the drop rule.
                match cur.checked_mul(cur) {
                    Some(next) if next <= 1_000_000_000 => cur = next,
                    _ => {
                        // cur² unreachable — no next level. Drop outermost pair.
                        chain_len = (chain_len - 1).max(1);
                        break;
                    }
                }
            }

            best = best.max(chain_len);
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // [2,4,2] → length 3
        assert_eq!(Solution::maximum_length(vec![5, 4, 1, 2, 2]), 3);
    }

    #[test]
    fn test_example_2() {
        // No valid multi-element chain → length 1
        assert_eq!(Solution::maximum_length(vec![1, 3, 2, 4]), 1);
    }

    #[test]
    fn test_pair_even_no_centre() {
        // freq[16]=2, 256 absent → drop pair → 1
        assert_eq!(Solution::maximum_length(vec![1, 16, 49, 16, 121]), 1);
    }

    #[test]
    fn test_all_ones_odd() {
        assert_eq!(Solution::maximum_length(vec![1, 1, 1, 1, 1]), 5);
    }

    #[test]
    fn test_all_ones_even() {
        assert_eq!(Solution::maximum_length(vec![1, 1, 1, 1]), 3);
    }

    #[test]
    fn test_chain_no_next_level() {
        // [2,4,16,4,2] = 5; 16 used as pair then 256 absent → drop → 5
        assert_eq!(Solution::maximum_length(vec![2, 2, 4, 4, 16, 16, 16]), 5);
    }

    #[test]
    fn test_chain_with_centre() {
        // [2,4,16,256,65536,256,16,4,2] = 9
        assert_eq!(
            Solution::maximum_length(vec![2, 2, 4, 4, 16, 16, 256, 256, 65536]),
            9
        );
    }

    #[test]
    fn test_overflow_chain_drop() {
        // Chain ends at 43046721 (its square overflows) → drop outermost pair → 9
        assert_eq!(
            Solution::maximum_length(vec![
                3, 3, 3, 3, 9, 9, 9, 9, 9, 9, 9, 9, 9, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81,
                81, 81, 81, 6561, 6561, 6561, 6561, 6561, 6561, 6561, 6561, 6561, 6561, 6561, 6561,
                6561, 6561, 43046721, 43046721, 43046721, 43046721, 43046721, 43046721, 43046721,
                43046721, 43046721, 43046721, 43046721, 43046721, 43046721, 43046721
            ]),
            9
        );
    }

    #[test]
    fn test_pair_then_single_centre() {
        // 2(×2), 4(×1): [2,4,2] = 3
        assert_eq!(Solution::maximum_length(vec![2, 2, 4]), 3);
    }

    #[test]
    fn test_pair_only_no_square() {
        // 3(×3): 9 absent → drop pair → 1
        assert_eq!(Solution::maximum_length(vec![3, 3, 3]), 1);
    }
}
