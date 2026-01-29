impl Solution {
    /// Finds the duplicate number using binary search on value range.
    ///
    /// # Intuition
    /// For a value mid, if count of numbers <= mid exceeds mid, the duplicate
    /// is in [1, mid]. Otherwise, it is in [mid+1, n].
    ///
    /// # Approach
    /// 1. Binary search on the value range [1, n].
    /// 2. Count elements <= mid.
    /// 3. If count > mid, duplicate is at or below mid.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (1, nums.len() - 1);
        while left < right {
            let mid = (left + right) >> 1;
            let count = nums.iter().filter(|x| **x <= mid as i32).count();
            if count > mid {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_one() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }

    #[test]
    fn duplicate_three() {
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

    #[test]
    fn duplicate_at_start() {
        assert_eq!(Solution::find_duplicate(vec![1, 1]), 1);
    }
}
