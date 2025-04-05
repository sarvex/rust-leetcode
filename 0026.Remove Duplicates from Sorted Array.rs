/// Remove duplicates in-place from a sorted array and return the number of unique elements.
///
/// # intuition
/// Since the array is already sorted, duplicates will be adjacent to each other.
/// We can use a two-pointer approach where one pointer tracks the position for the next unique element,
/// and the other scans through the array.
///
/// # approach
/// 1. Use an index to track where the next unique element should be placed
/// 2. Iterate through the array with a read pointer
/// 3. If the current element is different from the last written element, write it to the index position
/// 4. Increment index after each write
/// 5. Return index as the count of unique elements
///
/// # complexity
/// - Time complexity: O(n) where n is the length of the array
/// - Space complexity: O(1) as we modify the array in-place
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // Handle empty array edge case
        if nums.is_empty() {
            return 0;
        }

        let len: usize = nums.len();
        let mut index: usize = 1;

        for read in 1..len {
            if nums[read] != nums[read - 1] {
                nums[index] = nums[read];
                index += 1;
            }
        }

        index as i32
    }
}

#[cfg(test)]
mod tests {1
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums[0..2], [1, 2]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[0..5], [0, 1, 2, 3, 4]);
    }
}
