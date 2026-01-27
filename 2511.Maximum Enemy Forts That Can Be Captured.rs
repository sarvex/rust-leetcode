impl Solution {
    /// Finds the maximum number of enemy forts captured between a friendly fort and empty space.
    ///
    /// # Intuition
    /// We need the longest contiguous run of zeros (enemy forts) bounded by a 1 and a -1
    /// on either side. Scan for non-zero anchors and check spans between them.
    ///
    /// # Approach
    /// 1. Track the last non-zero position
    /// 2. When encountering another non-zero value, if their signs differ,
    ///    the gap between them contains capturable forts
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut last_nonzero = usize::MAX;

        for (i, &f) in forts.iter().enumerate() {
            if f != 0 {
                if last_nonzero != usize::MAX && forts[last_nonzero] + f == 0 {
                    ans = ans.max(i - last_nonzero - 1);
                }
                last_nonzero = i;
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::capture_forts(vec![0, 0, 1, -1]), 0);
    }

    #[test]
    fn test_no_forts() {
        assert_eq!(Solution::capture_forts(vec![1, -1]), 0);
    }

    #[test]
    fn test_same_sign_adjacent() {
        assert_eq!(Solution::capture_forts(vec![1, 0, 0, 1]), 0);
    }
}
