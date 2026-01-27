impl Solution {
    /// Finds the maximum ribbon length to cut at least k pieces via binary search.
    ///
    /// # Intuition
    /// The number of pieces obtainable is monotonically decreasing with
    /// piece length. Binary search finds the maximum feasible length.
    ///
    /// # Approach
    /// 1. Binary search over possible lengths [0, max_ribbon].
    /// 2. For each candidate length, count total pieces.
    /// 3. Use upper-bound binary search to find the maximum valid length.
    ///
    /// # Complexity
    /// - Time: O(n * log(max_val))
    /// - Space: O(1)
    pub fn max_length(ribbons: Vec<i32>, k: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, *ribbons.iter().max().unwrap());

        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            let count: i32 = ribbons.iter().map(|&r| r / mid).sum();
            if count >= k {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::max_length(vec![9, 7, 5], 3), 5);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::max_length(vec![5, 7, 9], 22), 0);
    }

    #[test]
    fn test_exact_fit() {
        assert_eq!(Solution::max_length(vec![7, 7, 7], 3), 7);
    }
}
