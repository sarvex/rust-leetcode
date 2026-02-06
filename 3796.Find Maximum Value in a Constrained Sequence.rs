impl Solution {
    /// Two-pass bounds: left-to-right maximizes growth; right-to-right propagates
    /// restriction caps backward. Per-index upper bound is the minimum of both;
    /// answer is the max of those bounds.
    ///
    /// # Intuition
    /// Each index has an upper bound from "how high we can get from the left"
    /// and "how high we can be and still satisfy future restrictions." The
    /// tightest valid bound at each index is the minimum of the two; the
    /// optimal sequence achieves the maximum of these bounds at some index.
    ///
    /// # Approach
    /// 1. Build a cap array from restrictions (indices without a restriction use a large sentinel).
    /// 2. Left-to-right: start at 0, set `left_high[i+1] = min(left_high[i] + diff[i], cap[i+1])`.
    /// 3. Right-to-left: start from last index with cap, set
    ///    `right_high[i] = min(right_high[i+1] + diff[i], cap[i])`.
    /// 4. At each index, valid upper bound is `min(left_high[i], right_high[i])`; return its max.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find_max_val(n: i32, restrictions: Vec<Vec<i32>>, diff: Vec<i32>) -> i32 {
        const INF: i32 = 2_000_001;
        let n = n as usize;

        let mut cap: Vec<i32> = vec![INF; n];
        for r in &restrictions {
            let (idx, max_val) = (r[0] as usize, r[1]);
            cap[idx] = cap[idx].min(max_val);
        }

        let mut left_high = vec![0i32; n];
        for i in 0..n.saturating_sub(1) {
            left_high[i + 1] = (left_high[i] + diff[i]).min(cap[i + 1]);
        }

        let mut right_high = vec![INF; n];
        right_high[n - 1] = cap[n - 1];
        for i in (0..n.saturating_sub(1)).rev() {
            right_high[i] = (right_high[i + 1] + diff[i]).min(cap[i]);
        }

        (0..n)
            .map(|i| left_high[i].min(right_high[i]))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::find_max_val(
                10,
                vec![vec![3, 1], vec![8, 1]],
                vec![2, 2, 3, 1, 4, 5, 1, 1, 2]
            ),
            6
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::find_max_val(8, vec![vec![3, 2]], vec![3, 5, 2, 4, 2, 3, 1]),
            12
        );
    }

    #[test]
    fn test_no_restrictions() {
        assert_eq!(Solution::find_max_val(3, vec![], vec![5, 5]), 10);
    }

    #[test]
    fn test_single_restriction_at_end() {
        assert_eq!(
            Solution::find_max_val(4, vec![vec![3, 2]], vec![10, 10, 10]),
            2
        );
    }
}