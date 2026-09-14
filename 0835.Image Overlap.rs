impl Solution {
    /// Row bitmasks with popcount over every translation offset.
    ///
    /// # Intuition
    /// Because `n <= 30`, an entire row fits in a single `u32`, so the overlap of
    /// one row pair collapses to `popcount(a & b)` — a single instruction instead
    /// of a column loop. A translation is then just "pick a row offset, pick a
    /// column bit-shift". Row offsets can be kept non-negative by swapping which
    /// grid is treated as the moving one, but column offsets must stay signed:
    /// translating down-and-left is not the mirror of any down-and-right move.
    ///
    /// # Approach
    /// 1. Pack each grid into `Vec<u32>`, bit `j` of row `r` holding `grid[r][j]`.
    /// 2. For offset `(dr, dc)` with `dr >= 0`, shift each source row left by `dc`
    ///    (right in column space) and clip to `n` bits, or right by `-dc`, then AND
    ///    with target row `row + dr` and sum the popcounts.
    /// 3. Take the max over `dr` in `0..n` and `dc` across the full signed span,
    ///    then repeat with the grids swapped to pick up the upward translations.
    ///
    /// # Complexity
    /// - Time: O(n^3) — O(n^2) offsets, n word-level row operations each
    /// - Space: O(n) — two packed row vectors
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let span = n as i32;
        let mask = (1u32 << n) - 1;

        let pack = |grid: &[Vec<i32>]| -> Vec<u32> {
            grid.iter()
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .fold(0u32, |bits, (col, cell)| bits | ((*cell as u32) << col))
                })
                .collect()
        };

        let (first, second) = (pack(&img1), pack(&img2));

        // Best overlap when `src` slides `dr` rows down and `dc` columns right
        // (`dc < 0` meaning left). Negative `dr` is handled by swapping the pair.
        let best = |src: &[u32], dst: &[u32]| -> u32 {
            (0..span)
                .flat_map(|dr| (1 - span..span).map(move |dc| (dr as usize, dc)))
                .map(|(dr, dc)| {
                    (0..n - dr)
                        .map(|row| {
                            let moved = if dc >= 0 {
                                (src[row] << dc) & mask
                            } else {
                                src[row] >> -dc
                            };
                            (moved & dst[row + dr]).count_ones()
                        })
                        .sum::<u32>()
                })
                .max()
                .unwrap_or(0)
        };

        best(&first, &second).max(best(&second, &first)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_shift_down_right() {
        let img1 = vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        let img2 = vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]];
        assert_eq!(Solution::largest_overlap(img1, img2), 3);
    }

    #[test]
    fn example_two_single_one() {
        assert_eq!(Solution::largest_overlap(vec![vec![1]], vec![vec![1]]), 1);
    }

    #[test]
    fn example_three_single_zero() {
        assert_eq!(Solution::largest_overlap(vec![vec![0]], vec![vec![0]]), 0);
    }

    #[test]
    fn one_grid_empty_yields_zero() {
        let img1 = vec![vec![1, 1], vec![1, 1]];
        let img2 = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::largest_overlap(img1, img2), 0);
    }

    #[test]
    fn shift_up_and_left_is_covered() {
        // img1 sits bottom-right, img2 top-left: needs a negative offset.
        let img1 = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 1, 1]];
        let img2 = vec![vec![1, 1, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::largest_overlap(img1, img2), 2);
    }

    #[test]
    fn identity_alignment_beats_any_shift() {
        let img = vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]];
        assert_eq!(Solution::largest_overlap(img.clone(), img), 5);
    }

    #[test]
    fn full_grid_max_constraints() {
        let img = vec![vec![1; 30]; 30];
        assert_eq!(Solution::largest_overlap(img.clone(), img), 900);
    }

    #[test]
    fn diagonal_translation_wraps_nothing() {
        // A single 1 in opposite corners: exactly one cell can ever align.
        let mut img1 = vec![vec![0; 30]; 30];
        let mut img2 = vec![vec![0; 30]; 30];
        img1[0][0] = 1;
        img2[29][29] = 1;
        assert_eq!(Solution::largest_overlap(img1, img2), 1);
    }

    #[test]
    fn mixed_sign_offset_down_and_left() {
        // Optimal translation is one row down and one column left, so the row and
        // column offsets have opposite signs.
        let img1 = vec![vec![0, 1, 1], vec![0, 1, 1], vec![0, 1, 0]];
        let img2 = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::largest_overlap(img1, img2), 3);
    }

    #[test]
    fn mixed_sign_offset_up_and_right() {
        let img1 = vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 1, 0]];
        let img2 = vec![vec![0, 1, 1], vec![0, 0, 1], vec![0, 0, 0]];
        assert_eq!(Solution::largest_overlap(img1, img2), 3);
    }

    #[test]
    fn partial_row_overlap_prefers_larger_offset() {
        let img1 = vec![
            vec![1, 1, 1, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let img2 = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::largest_overlap(img1, img2), 3);
    }
}
