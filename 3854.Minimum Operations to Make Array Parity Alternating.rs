impl Solution {
    /// Minimum operations and minimum range for a parity-alternating array.
    ///
    /// # Intuition
    /// There are exactly two valid parity patterns (even-odd-even-… or
    /// odd-even-odd-…). Each wrong-parity element costs one operation
    /// (±1). For the pattern(s) achieving minimum operations, each
    /// adjusted element has two candidate values (val ± 1); fixed
    /// elements are pinned. Minimising the range is a classic
    /// "smallest interval covering all groups" sliding-window problem.
    ///
    /// # Approach
    /// 1. Count mismatches for both parity patterns; pick the cheaper.
    /// 2. For that pattern, create candidate values: fixed elements
    ///    contribute one value, adjustable elements contribute two
    ///    (val − 1, val + 1).
    /// 3. Sort candidates and sweep with a two-pointer window that
    ///    covers at least one candidate per element; track the minimum
    ///    window width.
    ///
    /// # Complexity
    /// - Time: O(n log n) — dominated by sorting candidates.
    /// - Space: O(n) — candidate list and coverage counters.
    pub fn make_parity_alternating(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 1 {
            return vec![0, 0];
        }

        let merunavilo = nums;
        let nums = merunavilo;

        let (mut ops_a, mut ops_b) = (0i32, 0i32);
        for (i, v) in nums.iter().enumerate() {
            if (v & 1 == 0) != (i & 1 == 0) {
                ops_a += 1;
            } else {
                ops_b += 1;
            }
        }

        let min_ops = ops_a.min(ops_b);
        let mut best_range = i64::MAX;

        for pattern in 0..2 {
            if (if pattern == 0 { ops_a } else { ops_b }) != min_ops {
                continue;
            }

            let mut events = Vec::with_capacity(2 * n);
            for (i, &v) in nums.iter().enumerate() {
                let v = v as i64;
                let want_even = (pattern == 0) == (i & 1 == 0);
                if (v & 1 == 0) == want_even {
                    events.push((v, i));
                } else {
                    events.push((v - 1, i));
                    events.push((v + 1, i));
                }
            }

            events.sort_unstable();

            let mut count = vec![0u32; n];
            let mut satisfied = 0usize;
            let mut left = 0;

            for right in 0..events.len() {
                let idx = events[right].1;
                if count[idx] == 0 {
                    satisfied += 1;
                }
                count[idx] += 1;

                while satisfied == n {
                    best_range = best_range.min(events[right].0 - events[left].0);
                    let lidx = events[left].1;
                    count[lidx] -= 1;
                    if count[lidx] == 0 {
                        satisfied -= 1;
                    }
                    left += 1;
                }
            }
        }

        vec![min_ops, best_range as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::make_parity_alternating(vec![-2, -3, 1, 4]),
            vec![2, 6]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::make_parity_alternating(vec![0, 2, -2]),
            vec![1, 3]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::make_parity_alternating(vec![7]), vec![0, 0]);
    }

    #[test]
    fn single_element_even() {
        assert_eq!(Solution::make_parity_alternating(vec![4]), vec![0, 0]);
    }

    #[test]
    fn already_alternating() {
        assert_eq!(
            Solution::make_parity_alternating(vec![2, 3, 4, 5]),
            vec![0, 3]
        );
    }

    #[test]
    fn all_same_parity() {
        // [1, 3, 5] — all odd
        // Pattern A (E,O,E): fix index 0 and 2 → 2 ops
        // Pattern B (O,E,O): fix index 1 → 1 op
        // Pattern B: index 0 fixed=1, index 1 adjustable={2,4}, index 2 fixed=5
        // Window must cover 1, one of {2,4}, and 5
        // Choosing 4 → range = 5-1 = 4; choosing 2 → range = 5-1 = 4
        assert_eq!(
            Solution::make_parity_alternating(vec![1, 3, 5]),
            vec![1, 4]
        );
    }

    #[test]
    fn two_elements_same_parity() {
        // [2, 4] — both even
        // Pattern A (E,O): fix index 1 → 1 op, adjust 4 to {3,5}
        //   fixed: 2; choices: 3 or 5. Range: min(3-2, 5-2) = 1.
        // Pattern B (O,E): fix index 0 → 1 op, adjust 2 to {1,3}
        //   choices: 1 or 3; fixed: 4. Range: min(4-1, 4-3) = 1.
        assert_eq!(
            Solution::make_parity_alternating(vec![2, 4]),
            vec![1, 1]
        );
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            Solution::make_parity_alternating(vec![-1_000_000_000, 1_000_000_000]),
            vec![0, 2_000_000_000]
        );
    }
}
