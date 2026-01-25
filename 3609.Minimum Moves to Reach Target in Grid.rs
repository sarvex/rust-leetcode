impl Solution {
    /// Find minimum moves by working backwards from target to source.
    ///
    /// # Intuition
    /// Forward search is exponential. Instead, work backwards from (tx, ty) to (sx, sy).
    /// At each step, determine which coordinate was increased in the previous move by
    /// comparing tx and ty. When one coordinate is much larger, use modulo to skip steps.
    ///
    /// # Approach
    /// 1. Base case: if (tx, ty) == (sx, sy), return 0
    /// 2. If tx < sx or ty < sy, target is unreachable, return -1
    /// 3. If tx == ty and tx > 0, we can only have come from (0, tx) or (tx, 0)
    /// 4. If tx > ty, the last move increased x. Use modulo when tx >> ty to reduce quickly
    /// 5. If ty > tx, the last move increased y. Use modulo when ty >> tx to reduce quickly
    ///
    /// # Complexity
    /// - Time: O(log(max(tx, ty))) - each step reduces coordinates significantly
    /// - Space: O(log(max(tx, ty))) - recursion depth
    pub fn min_moves(sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
        Self::dfs(sx as i64, sy as i64, tx as i64, ty as i64)
    }

    fn dfs(sx: i64, sy: i64, tx: i64, ty: i64) -> i32 {
        // Base case: reached source
        if tx == sx && ty == sy {
            return 0;
        }

        // Target coordinates must be >= source coordinates
        if tx < sx || ty < sy {
            return -1;
        }

        if tx == ty {
            // From (x, x) where x > 0, we can only have come from (0, x) or (x, 0)
            if tx == 0 {
                return -1;
            }
            // Try both possibilities
            let from_zero_x = Self::dfs(sx, sy, 0, ty);
            let from_zero_y = Self::dfs(sx, sy, tx, 0);
            if from_zero_x != -1 && from_zero_y != -1 {
                return 1 + from_zero_x.min(from_zero_y);
            }
            if from_zero_x != -1 {
                return 1 + from_zero_x;
            }
            if from_zero_y != -1 {
                return 1 + from_zero_y;
            }
            return -1;
        }

        if tx > ty {
            // Last move increased x
            // Try both possibilities for previous position
            let mut best = i32::MAX;

            // Optimization: when tx >> ty, use modulo to reduce tx quickly
            // Only use modulo when remainder >= sx, otherwise it might skip unreachable states
            if tx > ty && ty > 0 && tx >= 2 * ty {
                let remainder = tx % ty;
                if remainder >= sx {
                    // We can reduce directly to remainder
                    let steps = (tx - remainder) / ty;
                    let sub = Self::dfs(sx, sy, remainder, ty);
                    if sub != -1 {
                        best = best.min(steps as i32 + sub);
                    }
                }
                // If remainder < sx, don't use modulo - let regular cases handle it
            }

            // Case 1: prev_x >= prev_y, so m = prev_x, prev_x = tx/2
            if tx % 2 == 0 {
                let prev_x = tx / 2;
                if prev_x >= ty && prev_x >= sx {
                    let sub = Self::dfs(sx, sy, prev_x, ty);
                    if sub != -1 {
                        best = best.min(sub + 1);
                    }
                }
            }

            // Case 2: prev_x < prev_y, so m = ty, prev_x = tx - ty
            let prev_x = tx - ty;
            if prev_x < ty && prev_x >= sx {
                let sub = Self::dfs(sx, sy, prev_x, ty);
                if sub != -1 {
                    best = best.min(sub + 1);
                }
            }

            if best == i32::MAX {
                -1
            } else {
                best
            }
        } else {
            // ty > tx: last move increased y
            // Try both possibilities for previous position
            let mut best = i32::MAX;

            // Optimization: when ty >> tx, use modulo to reduce ty quickly
            // Only use modulo when remainder >= sy, otherwise it might skip unreachable states
            if ty > tx && tx > 0 && ty >= 2 * tx {
                let remainder = ty % tx;
                if remainder >= sy {
                    // We can reduce directly to remainder
                    let steps = (ty - remainder) / tx;
                    let sub = Self::dfs(sx, sy, tx, remainder);
                    if sub != -1 {
                        best = best.min(steps as i32 + sub);
                    }
                }
                // If remainder < sy, don't use modulo - let regular cases handle it
            }

            // Case 1: prev_y >= prev_x, so m = prev_y, prev_y = ty/2
            if ty % 2 == 0 {
                let prev_y = ty / 2;
                if prev_y >= tx && prev_y >= sy {
                    let sub = Self::dfs(sx, sy, tx, prev_y);
                    if sub != -1 {
                        best = best.min(sub + 1);
                    }
                }
            }

            // Case 2: prev_y < prev_x, so m = tx, prev_y = ty - tx
            let prev_y = ty - tx;
            if prev_y < tx && prev_y >= sy {
                let sub = Self::dfs(sx, sy, tx, prev_y);
                if sub != -1 {
                    best = best.min(sub + 1);
                }
            }

            if best == i32::MAX {
                -1
            } else {
                best
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_moves(1, 2, 5, 4), 2);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_moves(0, 1, 2, 3), 3);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::min_moves(1, 1, 2, 2), -1);
    }

    #[test]
    fn test_same_point() {
        assert_eq!(Solution::min_moves(5, 4, 5, 4), 0);
    }

    #[test]
    fn test_single_move() {
        assert_eq!(Solution::min_moves(1, 2, 3, 2), 1); // (1,2) -> (1+2, 2) = (3,2)
    }
}
