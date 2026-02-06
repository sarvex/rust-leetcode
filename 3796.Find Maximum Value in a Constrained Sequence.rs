impl Solution {
    /// Single-array two-pass: left-to-right greedy, then right-to-left tighten
    /// in place; max of the tightened values is the answer.
    ///
    /// # Intuition
    /// Each index's upper bound is the minimum of "max from left" and "max
    /// consistent with future restrictions." One array holds the left pass,
    /// then the right pass tightens it in place; the maximum over indices is
    /// the optimal largest value.
    ///
    /// # Approach
    /// 1. Build `max_vals` from restrictions (i32::MAX where no restriction).
    /// 2. Left-to-right: `vals[i] = (vals[i-1] + diff[i-1]).min(max_vals[i])`.
    /// 3. Right-to-left: tighten `vals[i] = vals[i].min(vals[i+1] + diff[i])`
    ///    and track the maximum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find_max_val(n: i32, restrictions: Vec<Vec<i32>>, diff: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut max_vals = vec![i32::MAX; n];
        for r in restrictions {
            max_vals[r[0] as usize] = r[1];
        }
        let mut vals = vec![0; n];
        for i in 1..n {
            vals[i] = (vals[i - 1] + diff[i - 1]).min(max_vals[i]);
        }
        let mut res = vals[n - 1];
        for i in (0..n - 1).rev() {
            vals[i] = vals[i].min(vals[i + 1] + diff[i]);
            res = res.max(vals[i]);
        }
        res
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
        // Sequence [0, 10, 12, 2] achieves max value 12 at index 2
        assert_eq!(
            Solution::find_max_val(4, vec![vec![3, 2]], vec![10, 10, 10]),
            12
        );
    }
}