impl Solution {
    /// Finds the maximum area of a square hole by identifying the largest consecutive gap.
    ///
    /// # Intuition
    /// A square hole is formed by the intersection of horizontal and vertical gaps.
    /// To maximize the area, find the longest consecutive sequence of removed bars
    /// in both dimensions. The side length is the minimum of these two gaps.
    ///
    /// # Approach
    /// 1. Sort both bar arrays using `sort_unstable`.
    /// 2. Use a closure with `windows(2)` and `fold` to find the longest consecutive
    ///    sequence in each direction.
    /// 3. Compute the side as `min(h_gap, v_gap)` and return `side * side`.
    ///
    /// # Complexity
    /// - Time: O(H log H + V log V) where H and V are the bar array lengths
    /// - Space: O(1) ignoring sorting overhead
    pub fn maximize_square_hole_area(
        _n: i32,
        _m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_grid() {
        assert_eq!(
            Solution::maximize_square_hole_area(2, 1, vec![2, 3], vec![2]),
            4
        );
    }

    #[test]
    fn test_single_bar_each() {
        assert_eq!(
            Solution::maximize_square_hole_area(1, 1, vec![2], vec![2]),
            4
        );
    }

    #[test]
    fn test_non_consecutive_bars() {
        assert_eq!(
            Solution::maximize_square_hole_area(3, 3, vec![2, 4], vec![2, 4]),
            4
        );
    }
}
