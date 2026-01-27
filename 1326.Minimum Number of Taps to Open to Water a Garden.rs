impl Solution {
    /// Greedy interval covering via jump game transformation.
    ///
    /// # Intuition
    /// Each tap at position `i` with range `r` covers `[i-r, i+r]`. Transform
    /// this into a jump game: for each starting position, record the farthest
    /// reachable endpoint. Then greedily count the minimum jumps to reach `n`.
    ///
    /// # Approach
    /// 1. Build `reach[i]` = max right endpoint achievable from position `i`
    /// 2. Sweep left to right, tracking current reach and previous boundary
    /// 3. When hitting the previous boundary, increment tap count and extend
    /// 4. If stuck (reach ≤ current position), return -1
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the reach array
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut reach = vec![0i32; n + 1];

        for (i, &r) in ranges.iter().enumerate() {
            let left = (i as i32 - r).max(0) as usize;
            reach[left] = reach[left].max(i as i32 + r);
        }

        let (mut taps, mut max_reach, mut prev_end) = (0, 0i32, 0i32);

        for i in 0..n {
            max_reach = max_reach.max(reach[i]);
            if max_reach <= i as i32 {
                return -1;
            }
            if prev_end == i as i32 {
                taps += 1;
                prev_end = max_reach;
            }
        }

        taps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_coverage() {
        assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
    }

    #[test]
    fn impossible() {
        assert_eq!(Solution::min_taps(3, vec![0, 0, 0, 0]), -1);
    }

    #[test]
    fn minimal_taps() {
        assert_eq!(Solution::min_taps(7, vec![1, 2, 1, 0, 2, 1, 0, 1]), 3);
    }
}
