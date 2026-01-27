impl Solution {
    /// Determines if n flowers can be planted without adjacent violations.
    ///
    /// # Intuition
    /// Greedily plant at each valid empty position. A position is valid if it
    /// and both neighbors are empty (or boundaries).
    ///
    /// # Approach
    /// 1. Scan left to right, checking each empty spot and its neighbors.
    /// 2. Plant and skip the next position (guaranteed empty after planting).
    /// 3. Return true once enough flowers are planted.
    ///
    /// # Complexity
    /// - Time: O(m) where m = flowerbed length
    /// - Space: O(1)
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let len = flowerbed.len();
        let mut count = 0;
        let mut i = 0;
        while i < len {
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i == len - 1 || flowerbed[i + 1] == 0)
            {
                count += 1;
                if count >= n {
                    return true;
                }
                i += 1;
            }
            i += 1;
        }
        n <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_place() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn test_cannot_place() {
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn test_all_empty() {
        assert!(Solution::can_place_flowers(vec![0, 0, 0, 0, 0], 3));
    }

    #[test]
    fn test_single_empty() {
        assert!(Solution::can_place_flowers(vec![0], 1));
    }

    #[test]
    fn test_zero_needed() {
        assert!(Solution::can_place_flowers(vec![], 0));
    }
}
