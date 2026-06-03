impl Solution {
    /// Prefix-sum BFS reachability over a binary string.
    ///
    /// # Intuition
    /// A position `j` is reachable if any position `i` in `[j-maxJump, j-minJump]`
    /// is reachable and `s[j] == '0'`. Checking each window naively is O(n × window),
    /// but a prefix sum over reachable positions lets us answer "is there any reachable
    /// index in this range?" in O(1), giving an overall O(n) solution.
    ///
    /// # Approach
    /// 1. Build a boolean `reach` array; mark index 0 as reachable.
    /// 2. Maintain a prefix-sum array `pre` where `pre[i]` = number of reachable
    ///    indices in `reach[0..i]`.
    /// 3. For each index `j` where `s[j] == '0'`, compute the window
    ///    `[j - maxJump, j - minJump]` (clamped to valid range).
    ///    If `pre[hi+1] - pre[lo] > 0`, mark `j` as reachable and update `pre`.
    /// 4. Return `reach[n-1]`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;

        let mut reach = vec![false; n];
        reach[0] = true;

        // pre[i] = number of reachable positions in reach[0..i]
        let mut pre = vec![0i32; n + 1];
        pre[1] = 1;

        for j in 1..n {
            if s[j] == b'0' && j >= min_jump {
                let lo = j - max_jump.min(j); // clamp to 0
                let hi = j - min_jump;
                if pre[hi + 1] - pre[lo] > 0 {
                    reach[j] = true;
                }
            }
            pre[j + 1] = pre[j] + reach[j] as i32;
        }

        reach[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        // 0 -> 3 -> 5
        assert!(Solution::can_reach("011010".to_string(), 2, 3));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::can_reach("01101110".to_string(), 2, 3));
    }

    #[test]
    fn last_index_is_one() {
        // Last index is '1', can never be reached
        assert!(!Solution::can_reach("0101".to_string(), 1, 2));
    }

    #[test]
    fn direct_single_jump() {
        // "00", minJump=1, maxJump=1 -> 0 -> 1
        assert!(Solution::can_reach("00".to_string(), 1, 1));
    }

    #[test]
    fn no_valid_landing_spots() {
        // All intermediate positions are '1'
        assert!(!Solution::can_reach("0110".to_string(), 1, 1));
    }

    #[test]
    fn min_equals_max_exact_steps() {
        // "000000", minJump=maxJump=2 -> 0->2->4 (index 5 unreachable)
        assert!(!Solution::can_reach("000000".to_string(), 2, 2));
    }

    #[test]
    fn min_equals_max_reaches_end() {
        // "00000", minJump=maxJump=2 -> 0->2->4
        assert!(Solution::can_reach("00000".to_string(), 2, 2));
    }

    #[test]
    fn large_window_covers_all() {
        // minJump=1, maxJump=100 on a string of zeros
        let s = "0".repeat(10) + "0";
        assert!(Solution::can_reach(s, 1, 100));
    }

    #[test]
    fn start_equals_end() {
        // Length 2, direct jump
        assert!(Solution::can_reach("00".to_string(), 1, 2));
    }
}
