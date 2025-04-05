/// Remove duplicates from a sorted array, allowing at most two occurrences of each element
///
/// # intuition
/// Since the array is sorted, we can use a two-pointer approach where one pointer (k) tracks
/// the position for the next valid element, and another pointer (i) scans through the array.
/// We can maintain the invariant that the first k elements are our result.
///
/// # approach
/// 1. Initialize a pointer k to track the position for the next valid element
/// 2. Iterate through the array with pointer i
/// 3. For each element, if it's valid (either k < 2 or the current element is different from
///    the element at k-2), copy it to position k and increment k
/// 4. The valid element condition ensures we have at most 2 of each value
/// 5. Return k as the new length
///
/// # complexity
/// - Time complexity: O(n), where n is the length of the array
/// - Space complexity: O(1), only constant extra space is used
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }

        let mut k = 2;
        for i in 2..n {
            if nums[i] != nums[k - 2] {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[0..5], [1, 1, 2, 2, 3]);

        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
        assert_eq!(nums[0..7], [0, 0, 1, 1, 2, 3, 3]);
    }
}
