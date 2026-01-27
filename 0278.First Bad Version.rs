// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    /// Finds the first bad version using binary search.
    ///
    /// # Intuition
    /// All versions after the first bad one are also bad. Binary search
    /// efficiently finds the transition point.
    ///
    /// # Approach
    /// 1. Binary search between 1 and n.
    /// 2. If mid is bad, search left half.
    /// 3. Otherwise, search right half.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
