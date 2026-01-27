impl Solution {
    /// Maximum gap product after sorting cuts.
    ///
    /// # Intuition
    /// The largest cake piece is the product of the largest horizontal gap
    /// and the largest vertical gap. Sorting cuts and computing consecutive
    /// differences (including edges) yields these gaps.
    ///
    /// # Approach
    /// 1. Sort horizontal and vertical cuts
    /// 2. Compute max gap including borders (0 and h/w)
    /// 3. Return `(max_h_gap * max_v_gap) % MOD`
    ///
    /// # Complexity
    /// - Time: O(m log m + n log n)
    /// - Space: O(1) auxiliary
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        const MOD: i64 = 1_000_000_007;

        horizontal_cuts.sort_unstable();
        vertical_cuts.sort_unstable();

        let max_h = horizontal_cuts
            .windows(2)
            .map(|w| (w[1] - w[0]) as i64)
            .chain(std::iter::once(horizontal_cuts[0] as i64))
            .chain(std::iter::once(
                (h - horizontal_cuts.last().unwrap()) as i64,
            ))
            .max()
            .unwrap();

        let max_v = vertical_cuts
            .windows(2)
            .map(|w| (w[1] - w[0]) as i64)
            .chain(std::iter::once(vertical_cuts[0] as i64))
            .chain(std::iter::once((w - vertical_cuts.last().unwrap()) as i64))
            .max()
            .unwrap();

        ((max_h * max_v) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_cuts() {
        assert_eq!(Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
    }

    #[test]
    fn single_cuts() {
        assert_eq!(Solution::max_area(5, 4, vec![3], vec![3]), 9);
    }
}
