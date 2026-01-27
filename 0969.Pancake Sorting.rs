impl Solution {
    /// Sorts array using pancake flips (reverse prefixes).
    ///
    /// # Intuition
    /// Place each element at its correct position from largest to smallest.
    /// Flip the max element to the front, then flip it to its target position.
    ///
    /// # Approach
    /// For each position from end to start, find the max in the unsorted
    /// prefix, flip it to the front, then flip it to the target position.
    ///
    /// # Complexity
    /// - Time: O(n^2) — n iterations with O(n) search each
    /// - Space: O(n) for the flip indices
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut flips = Vec::new();
        for n in (1..arr.len()).rev() {
            let max_idx = (0..=n).max_by_key(|&i| arr[i]).unwrap();
            if max_idx != n {
                if max_idx != 0 {
                    arr[..=max_idx].reverse();
                    flips.push(max_idx as i32 + 1);
                }
                arr[..=n].reverse();
                flips.push(n as i32 + 1);
            }
        }
        flips
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let arr = vec![3, 2, 4, 1];
        let flips = Solution::pancake_sort(arr.clone());
        assert!(!flips.is_empty()); // Verify flips were produced
    }

    #[test]
    fn test_already_sorted() {
        assert!(Solution::pancake_sort(vec![1, 2, 3]).is_empty());
    }
}
