impl Solution {
    /// Finds the maximum removals keeping p as a subsequence of s via binary search.
    ///
    /// # Intuition
    /// The feasibility of removing k characters is monotonic — if k works,
    /// so does k-1. Binary search identifies the maximum feasible k.
    ///
    /// # Approach
    /// 1. Binary search over the number of removals [0, removable.len()].
    /// 2. For each candidate k, mark the first k removable indices.
    /// 3. Check if p is still a subsequence of the remaining characters.
    ///
    /// # Complexity
    /// - Time: O((m + n) * log(removable.len()))
    /// - Space: O(m)
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let m = s.len();
        let n = p.len();

        let is_subsequence = |k: usize| -> bool {
            let mut removed = vec![false; m];
            for &idx in &removable[..k] {
                removed[idx as usize] = true;
            }
            let mut j = 0;
            for (i, &b) in s.iter().enumerate() {
                if j < n && !removed[i] && b == p[j] {
                    j += 1;
                }
            }
            j == n
        };

        let (mut lo, mut hi) = (0, removable.len());
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if is_subsequence(mid) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::maximum_removals("abcacb".to_string(), "ab".to_string(), vec![3, 1, 0]),
            2
        );
    }

    #[test]
    fn test_no_removals() {
        assert_eq!(
            Solution::maximum_removals("ab".to_string(), "ab".to_string(), vec![0, 1]),
            0
        );
    }

    #[test]
    fn test_all_removable() {
        assert_eq!(
            Solution::maximum_removals(
                "abcbddddd".to_string(),
                "abcd".to_string(),
                vec![3, 2, 1, 4, 5, 6]
            ),
            1
        );
    }
}
