impl Solution {
    /// Computes the H-Index by sorting citations in descending order.
    ///
    /// # Intuition
    /// After sorting descending, the H-Index is the largest h where
    /// citations[h-1] >= h.
    ///
    /// # Approach
    /// 1. Sort citations in descending order.
    /// 2. Iterate from the largest possible h downward.
    /// 3. Return the first h where the h-th citation >= h.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by(|a, b| b.cmp(a));
        for i in (1..=citations.len()).rev() {
            if citations[i - 1] >= i as i32 {
                return i as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::h_index(vec![1, 1, 1]), 1);
    }

    #[test]
    fn zero_citations() {
        assert_eq!(Solution::h_index(vec![0]), 0);
    }
}
