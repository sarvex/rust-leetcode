impl Solution {
    /// Two-pointer in-place deduplication of a sorted array.
    ///
    /// # Intuition
    /// In a sorted array, duplicates are adjacent. A write pointer tracks
    /// the position for the next unique element while a read pointer scans
    /// forward, copying only distinct values.
    ///
    /// # Approach
    /// Start the write pointer at index 1. For each element from index 1
    /// onward, if it differs from its predecessor, copy it to the write
    /// position and advance. Return the write pointer as the unique count.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the array
    /// - Space: O(1) — in-place modification
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut write = 1;
        for read in 1..nums.len() {
            if nums[read] != nums[read - 1] {
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
    fn short_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(&nums[..2], &[1, 2]);
    }

    #[test]
    fn longer_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(&nums[..5], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn no_duplicates() {
        let mut nums = vec![1, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 3);
    }

    #[test]
    fn empty_array() {
        let mut nums: Vec<i32> = vec![];
        assert_eq!(Solution::remove_duplicates(&mut nums), 0);
    }

    #[test]
    fn single_element() {
        let mut nums = vec![1];
        assert_eq!(Solution::remove_duplicates(&mut nums), 1);
    }
}
