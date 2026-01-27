impl Solution {
    /// Finds the longest turbulent subarray using alternating comparison DP.
    ///
    /// # Intuition
    /// A turbulent subarray alternates between increasing and decreasing.
    /// Track two counters: one for ending on an increase, one for a decrease.
    ///
    /// # Approach
    /// For each pair, if ascending, the "up" counter extends the previous
    /// "down" counter. If descending, vice versa. Otherwise both reset.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut max_len = 1;
        let (mut up, mut down) = (1, 1);
        for i in 1..arr.len() {
            let (new_up, new_down) = match arr[i - 1].cmp(&arr[i]) {
                std::cmp::Ordering::Less => (down + 1, 1),
                std::cmp::Ordering::Greater => (1, up + 1),
                std::cmp::Ordering::Equal => (1, 1),
            };
            up = new_up;
            down = new_down;
            max_len = max_len.max(up.max(down));
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::max_turbulence_size(vec![4, 4, 4]), 1);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
    }
}
