impl Solution {
    /// Finds the maximum area of a square hole by identifying the largest consecutive gap in grid bars.
    ///
    /// # intuition
    /// A square hole is formed by the intersection of horizontal and vertical gaps. To maximize the area,
    /// we need to find the longest consecutive sequence of removed bars in both dimensions.
    /// The maximum possible side of a square is the minimum of these two maximum gap lengths.
    ///
    /// # approach
    /// 1. Sort both horizontal and vertical bar arrays using `sort_unstable()`.
    /// 2. Use a closure with an iterator chain (`windows` and `fold`) to find the longest consecutive sequence.
    /// 3. The gap size is `consecutive_count + 1`.
    /// 4. Calculate the side as `min(h_gap, v_gap)` and return `side * side`.
    ///
    /// # complexity
    /// - time: O(H log H + V log V) where H and V are the lengths of `h_bars` and `v_bars`.
    /// - space: O(1) ignoring the space used by the sorting algorithm.
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let _ = (n, m);
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;

        let max_consecutive = |bars: &mut Vec<i32>| -> i32 {
            bars.sort_unstable();
            bars.windows(2)
                .fold((1, 1), |(max_gap, curr_gap), w| {
                    if w[1] == w[0] + 1 {
                        let new_gap = curr_gap + 1;
                        (max_gap.max(new_gap), new_gap)
                    } else {
                        (max_gap, 1)
                    }
                })
                .0
                + 1
        };

        let h_max = max_consecutive(&mut h_bars);
        let v_max = max_consecutive(&mut v_bars);
        let side = h_max.min(v_max);
        side * side
    }
}
