impl Solution {
    /// Finds minimum recolors for k consecutive black blocks via sliding window.
    ///
    /// # Intuition
    /// A sliding window of size k tracks the count of black blocks. The minimum
    /// number of whites in any window equals the required recolors.
    ///
    /// # Approach
    /// 1. Count black blocks in the initial window of size k
    /// 2. Slide the window, adjusting count for entering/leaving elements
    /// 3. Track minimum whites (k - black_count) across all windows
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let s = blocks.as_bytes();

        let black_count = s[..k].iter().filter(|c| **c == b'B').count();
        let mut min_whites = k - black_count;
        let mut blacks = black_count;

        for i in k..s.len() {
            blacks += (s[i] == b'B') as usize;
            blacks -= (s[i - k] == b'B') as usize;
            min_whites = min_whites.min(k - blacks);
        }

        min_whites as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_recolors("WBWBBBW".to_string(), 2), 0);
    }

    #[test]
    fn test_all_white() {
        assert_eq!(Solution::minimum_recolors("WWWW".to_string(), 2), 2);
    }

    #[test]
    fn test_all_black() {
        assert_eq!(Solution::minimum_recolors("BBBB".to_string(), 3), 0);
    }

    #[test]
    fn test_single_block() {
        assert_eq!(Solution::minimum_recolors("W".to_string(), 1), 1);
        assert_eq!(Solution::minimum_recolors("B".to_string(), 1), 0);
    }
}
