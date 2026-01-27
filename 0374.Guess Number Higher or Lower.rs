impl Solution {
    /// Finds the guessed number using binary search.
    ///
    /// # Intuition
    /// The guess API provides comparison feedback, enabling binary search
    /// to converge in O(log n) calls.
    ///
    /// # Approach
    /// 1. Binary search between 1 and n.
    /// 2. Use the guess API to determine which half to search.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        loop {
            let mid = left + (right - left) / 2;
            match unsafe { guess(mid) } {
                -1 => right = mid - 1,
                1 => left = mid + 1,
                _ => break mid,
            }
        }
    }
}
