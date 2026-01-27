impl Solution {
    /// Two-pointer in-place deduplication allowing at most two occurrences.
    ///
    /// # Intuition
    /// In a sorted array, checking `nums[write - 2]` against the current
    /// element determines whether a third (or more) duplicate would be
    /// introduced. If different, the element is safe to write.
    ///
    /// # Approach
    /// Start the write pointer at 2 (first two elements always kept).
    /// For each element from index 2 onward, write it only if it differs
    /// from `nums[write - 2]`. Return the write pointer as the new length.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — in-place modification
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }

        let mut write = 2;
        for read in 2..n {
            if nums[read] != nums[write - 2] {
                nums[write] = nums[read];
                write += 1;
            }
        }

        write as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_duplicates() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(&nums[..5], &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn four_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
        assert_eq!(&nums[..7], &[0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn no_duplicates() {
        let mut nums = vec![1, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 3);
    }

    #[test]
    fn two_elements() {
        let mut nums = vec![1, 1];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    }
}
