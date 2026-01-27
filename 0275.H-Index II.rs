impl Solution {
    /// Computes the H-Index from a sorted array using binary search.
    ///
    /// # Intuition
    /// In a sorted array, binary search for the largest h where
    /// citations[n - h] >= h.
    ///
    /// # Approach
    /// 1. Binary search on h in range [0, n].
    /// 2. If citations[n - mid] >= mid, try a larger h.
    /// 3. Otherwise, try smaller h.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let (mut left, mut right) = (0, n);
        while left < right {
            let mid = (left + right + 1) >> 1;
            if citations[n - mid] >= mid as i32 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_sorted() {
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(Solution::h_index(vec![0, 0, 0]), 0);
    }

    #[test]
    fn single_high() {
        assert_eq!(Solution::h_index(vec![100]), 1);
    }
}
