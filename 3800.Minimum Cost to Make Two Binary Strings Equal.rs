impl Solution {
    /// Minimum cost to make s and t equal using flip, same-string swap, and cross swap.
    ///
    /// # Intuition
    /// Only positions where s[i] ≠ t[i] matter. Classify each mismatch as type A
    /// (s[i]=0, t[i]=1) or type B (s[i]=1, t[i]=0). An (A,B) pair can be fixed with
    /// one same-string swap; two same-type mismatches need one cross swap then one
    /// same-string swap (or two flips). Single leftover mismatches are fixed by flip.
    ///
    /// # Approach
    /// Count type A and type B mismatches. Pair mixed (A,B) with cost
    /// min(swapCost, 2*flipCost) per pair; pair same-type with cost
    /// min(crossCost+swapCost, 2*flipCost) per pair; fix remainder with flips.
    /// Compare with fixing all mismatches by flips: (a+b)*flipCost.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_cost(
        s: String,
        t: String,
        flip_cost: i32,
        swap_cost: i32,
        cross_cost: i32,
    ) -> i64 {
        let (flip_cost, swap_cost, cross_cost) = (
            flip_cost as i64,
            swap_cost as i64,
            cross_cost as i64,
        );
        let s = s.as_bytes();
        let t = t.as_bytes();

        let (mut type_a, mut type_b) = (0i64, 0i64);
        for (sc, tc) in s.iter().zip(t.iter()) {
            match (sc, tc) {
                (b'0', b'1') => type_a += 1,
                (b'1', b'0') => type_b += 1,
                _ => {}
            }
        }

        let pair_mixed = (swap_cost).min(2 * flip_cost);
        let pair_same = (cross_cost + swap_cost).min(2 * flip_cost);
        let paired = type_a.min(type_b);
        let unpaired = (type_a - type_b).abs();
        let strategy = paired * pair_mixed
            + (unpaired / 2) * pair_same
            + (unpaired % 2) * flip_cost;
        let all_flips = (type_a + type_b) * flip_cost;
        strategy.min(all_flips)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::minimum_cost(
                "01000".into(),
                "10111".into(),
                10,
                2,
                2
            ),
            16
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::minimum_cost("001".into(), "110".into(), 2, 100, 100),
            6
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::minimum_cost("1010".into(), "1010".into(), 5, 5, 5),
            0
        );
    }

    #[test]
    fn test_single_mismatch_flip() {
        assert_eq!(
            Solution::minimum_cost("0".into(), "1".into(), 1, 10, 10),
            1
        );
    }

    #[test]
    fn test_one_a_one_b_swap() {
        assert_eq!(
            Solution::minimum_cost("01".into(), "10".into(), 100, 1, 100),
            1
        );
    }
}
