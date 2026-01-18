use std::collections::HashSet;

impl Solution {
    /// Maximum Square Area by Removing Fences From a Field
    ///
    /// # Intuition
    /// Find the largest gap common to both directions. Build HashSet from shorter
    /// array's gaps, iterate longer array with early pruning.
    ///
    /// # Approach
    /// 1. Add boundaries and sort both fence arrays
    /// 2. Build HashSet of gaps from shorter array
    /// 3. Iterate longer array pairs: outer loop forward, inner loop reverse
    /// 4. Prune inner loop when diff <= max_side (since diff decreases as j decreases)
    ///
    /// # Complexity
    /// - Time: O(h² + v²) worst case, better average with pruning
    /// - Space: O(min(h², v²)) for HashSet
    pub fn maximize_square_area(
        m: i32,
        n: i32,
        mut h_fences: Vec<i32>,
        mut v_fences: Vec<i32>,
    ) -> i32 {
        h_fences.push(1);
        h_fences.push(m);
        h_fences.sort_unstable();

        v_fences.push(1);
        v_fences.push(n);
        v_fences.sort_unstable();

        let (short, long) = if h_fences.len() < v_fences.len() {
            (&h_fences, &v_fences)
        } else {
            (&v_fences, &h_fences)
        };

        let s_len = short.len();
        let mut gaps = HashSet::with_capacity(s_len * (s_len - 1) / 2);
        for i in 0..s_len {
            for j in i + 1..s_len {
                gaps.insert(short[j] - short[i]);
            }
        }

        let mut max_side = 0_i32;
        let l_len = long.len();

        for i in 0..l_len {
            for j in (i + 1..l_len).rev() {
                let diff = long[j] - long[i];

                if diff <= max_side {
                    break;
                }

                if gaps.contains(&diff) {
                    max_side = diff;
                }
            }
        }

        if max_side == 0 {
            -1
        } else {
            let side = max_side as i64;
            ((side * side) % 1_000_000_007) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::maximize_square_area(6, 7, vec![2], vec![4]), -1);
    }

    #[test]
    fn test_single_fence_each() {
        assert_eq!(Solution::maximize_square_area(3, 3, vec![2], vec![2]), 1);
    }

    #[test]
    fn test_large_square_possible() {
        assert_eq!(
            Solution::maximize_square_area(5, 5, vec![2, 3, 4], vec![2, 3, 4]),
            16
        );
    }

    #[test]
    fn test_no_common_gaps() {
        assert_eq!(
            Solution::maximize_square_area(10, 5, vec![5], vec![2, 3]),
            -1
        );
    }

    #[test]
    fn test_failing_case() {
        assert_eq!(
            Solution::maximize_square_area(7, 4, vec![2, 3, 6, 5], vec![3, 2]),
            9
        );
    }
}
