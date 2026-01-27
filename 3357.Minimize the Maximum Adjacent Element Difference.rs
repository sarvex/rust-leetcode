use std::cmp::{max, min};

impl Solution {
    /// Minimizes maximum adjacent difference via binary search with two-value assignment.
    ///
    /// # Intuition
    /// The answer is monotonic: if difference d is achievable, so is any d' > d.
    /// Binary search on d, then greedily check whether two candidate values
    /// (min_bound + d and max_bound − d) can fill every −1 position.
    ///
    /// # Approach
    /// 1. Collect min/max of known values adjacent to −1 positions.
    /// 2. Binary search on d in [0, 10^9].
    /// 3. For each candidate d, simulate a state machine tracking reachability
    ///    with candidate values x = min_b + d and y = max_b − d.
    ///
    /// # Complexity
    /// - Time: O(n log(max_diff))
    /// - Space: O(1)
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let neg_count = nums.iter().filter(|&&v| v == -1).count();

        match neg_count {
            c if c == n => return 0,
            0 => {
                return nums
                    .windows(2)
                    .map(|w| (w[1] - w[0]).abs())
                    .max()
                    .unwrap_or(0);
            }
            _ => {}
        }

        let (mut min_b, mut max_b) = (i32::MAX, i32::MIN);
        for i in 0..n {
            if nums[i] != -1 && ((i > 0 && nums[i - 1] == -1) || (i + 1 < n && nums[i + 1] == -1)) {
                min_b = min_b.min(nums[i]);
                max_b = max_b.max(nums[i]);
            }
        }

        let (mut lo, mut hi) = (0i32, 1_000_000_000i32);

        while lo < hi {
            let d = lo + (hi - lo) / 2;
            let (x, y) = (min_b + d, max_b - d);
            let (mut ok_x, mut ok_y) = (true, true);

            for i in 1..n {
                let (mut nx, mut ny) = (false, false);
                let (p, c) = (nums[i - 1], nums[i]);

                match (p == -1, c == -1) {
                    (true, true) => {
                        nx = ok_x || (ok_y && (x - y).abs() <= d);
                        ny = ok_y || (ok_x && (y - x).abs() <= d);
                    }
                    (true, false) => {
                        let from_x = ok_x && (c - x).abs() <= d;
                        let from_y = ok_y && (c - y).abs() <= d;
                        nx = from_x || from_y;
                        ny = nx;
                    }
                    (false, true) => {
                        let any = ok_x || ok_y;
                        nx = any && (x - p).abs() <= d;
                        ny = any && (y - p).abs() <= d;
                    }
                    (false, false) => {
                        let ok = (ok_x || ok_y) && (c - p).abs() <= d;
                        nx = ok;
                        ny = ok;
                    }
                }

                ok_x = nx;
                ok_y = ny;
                if !ok_x && !ok_y {
                    break;
                }
            }

            if ok_x || ok_y {
                hi = d;
            } else {
                lo = d + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_known_and_unknown_values() {
        assert_eq!(Solution::min_difference(vec![1, 2, -1, 10, 8]), 4);
    }

    #[test]
    fn all_unknown_returns_zero() {
        assert_eq!(Solution::min_difference(vec![-1, -1, -1]), 0);
    }

    #[test]
    fn alternating_unknown_known() {
        assert_eq!(Solution::min_difference(vec![-1, 10, -1, 8]), 1);
    }

    #[test]
    fn multiple_consecutive_unknowns() {
        assert_eq!(Solution::min_difference(vec![2, -1, 4, -1, -1, 6]), 1);
    }

    #[test]
    fn large_values_with_gaps() {
        let nums = vec![
            4147, 11098, 10014, -1, -1, 1329, -1, 8870, 13649, 16392, 25035, 1685, -1, 9754, -1,
            24948, 26665, 546, -1, -1, -1, 23035, 23387, 13465, -1, 5354, 23367, -1, 22966, 12767,
        ];
        assert_eq!(Solution::min_difference(nums), 26119);
    }
}
