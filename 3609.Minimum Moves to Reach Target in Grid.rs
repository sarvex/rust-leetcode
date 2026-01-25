use std::collections::HashMap;

impl Solution {
    /// Find minimum moves by working backwards from target to source.
    ///
    /// # Intuition
    /// Forward search is exponential. Instead, work backwards from (tx, ty) to (sx, sy).
    /// At each step, determine which coordinate was increased in the previous move by
    /// comparing tx and ty. If tx > ty, the last move increased x; if ty > tx, it increased y.
    ///
    /// # Approach
    /// 1. Base case: if (tx, ty) == (sx, sy), return 0
    /// 2. If tx < sx or ty < sy, target is unreachable, return -1
    /// 3. If tx == ty, check if we can reach (sx, sy) from this state
    /// 4. If tx > ty, the last move increased x, so previous position was (tx - m, ty) where m = max(prev_x, prev_y)
    ///    - If prev_x >= prev_y: m = prev_x, so prev_x = tx/2
    ///    - If prev_x < prev_y: m = prev_y = ty, so prev_x = tx - ty
    /// 5. Similarly for ty > tx
    /// 6. Use memoization to avoid recomputing states
    ///
    /// # Complexity
    /// - Time: O(log(max(tx, ty))) - each step reduces coordinates significantly
    /// - Space: O(log(max(tx, ty))) - memoization depth
    pub fn min_moves(sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::dfs(sx, sy, tx, ty, &mut memo)
    }

    fn dfs(
        sx: i32,
        sy: i32,
        tx: i32,
        ty: i32,
        memo: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        // Base case: reached source
        if tx == sx && ty == sy {
            return 0;
        }

        // Target coordinates must be >= source coordinates
        if tx < sx || ty < sy {
            return -1;
        }

        // Check memoization
        if let Some(&result) = memo.get(&(tx, ty)) {
            return result;
        }

        let result = if tx == ty {
            // Special case: both coordinates equal
            // From (x, x) where x > 0, we can only have come from (0, x) or (x, 0) in one move
            // Because: (0, x) -> (0 + max(0, x), x) = (x, x) or (x, 0) -> (x, 0 + max(x, 0)) = (x, x)
            if tx == 0 {
                -1
            } else {
                let mut best = i32::MAX;
                
                // Try (0, tx) -> (tx, tx)
                let sub1 = Self::dfs(sx, sy, 0, ty, memo);
                if sub1 != -1 {
                    best = best.min(sub1 + 1);
                }
                
                // Try (tx, 0) -> (tx, tx)
                let sub2 = Self::dfs(sx, sy, tx, 0, memo);
                if sub2 != -1 {
                    best = best.min(sub2 + 1);
                }
                
                if best == i32::MAX {
                    -1
                } else {
                    best
                }
            }
        } else if tx > ty {
            // Last move increased x
            // Optimization: if tx >= 2*ty, we can use modulo to skip steps
            if tx >= 2 * ty {
                // We must have come from (tx/2, ty) repeatedly
                // Count how many times we can divide by 2
                let mut count = 0;
                let mut curr_tx = tx;
                while curr_tx >= 2 * ty && curr_tx % 2 == 0 {
                    curr_tx /= 2;
                    count += 1;
                }
                // Now try from (curr_tx, ty)
                let sub = Self::dfs(sx, sy, curr_tx, ty, memo);
                if sub != -1 {
                    return sub + count;
                }
                return -1;
            }
            
            // Regular case: try both possibilities
            let mut best = i32::MAX;
            
            // Case 1: previous position had prev_x >= prev_y, so m = prev_x
            // prev_x + prev_x = tx, so prev_x = tx / 2, prev_y = ty
            if tx % 2 == 0 {
                let prev_x = tx / 2;
                if prev_x >= ty {
                    let sub = Self::dfs(sx, sy, prev_x, ty, memo);
                    if sub != -1 {
                        best = best.min(sub + 1);
                    }
                }
            }
            
            // Case 2: previous position had prev_x < prev_y, so m = prev_y = ty
            // prev_x + ty = tx, so prev_x = tx - ty, prev_y = ty
            let prev_x = tx - ty;
            if prev_x < ty && prev_x >= sx {
                let sub = Self::dfs(sx, sy, prev_x, ty, memo);
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
            // Optimization: if ty >= 2*tx, we can use modulo to skip steps
            if ty >= 2 * tx {
                // We must have come from (tx, ty/2) repeatedly
                // Count how many times we can divide by 2
                let mut count = 0;
                let mut curr_ty = ty;
                while curr_ty >= 2 * tx && curr_ty % 2 == 0 {
                    curr_ty /= 2;
                    count += 1;
                }
                // Now try from (tx, curr_ty)
                let sub = Self::dfs(sx, sy, tx, curr_ty, memo);
                if sub != -1 {
                    return sub + count;
                }
                return -1;
            }
            
            // Regular case: try both possibilities
            let mut best = i32::MAX;
            
            // Case 1: previous position had prev_y >= prev_x, so m = prev_y
            // prev_y + prev_y = ty, so prev_y = ty / 2, prev_x = tx
            if ty % 2 == 0 {
                let prev_y = ty / 2;
                if prev_y >= tx {
                    let sub = Self::dfs(sx, sy, tx, prev_y, memo);
                    if sub != -1 {
                        best = best.min(sub + 1);
                    }
                }
            }
            
            // Case 2: previous position had prev_y < prev_x, so m = prev_x = tx
            // prev_y + tx = ty, so prev_y = ty - tx, prev_x = tx
            let prev_y = ty - tx;
            if prev_y < tx && prev_y >= sy {
                let sub = Self::dfs(sx, sy, tx, prev_y, memo);
                if sub != -1 {
                    best = best.min(sub + 1);
                }
            }
            
            if best == i32::MAX {
                -1
            } else {
                best
            }
        };

        memo.insert((tx, ty), result);
        result
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
