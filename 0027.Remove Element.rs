/// Two-pointer approach that shifts valid elements to the front of the array
/// 
/// # intuition
/// We can use a two-pointer technique where one pointer (k) tracks the position
/// for the next valid element, while another pointer (i) scans through the array.
/// When we find elements that are not equal to the target value, we place them
/// at the position indicated by the first pointer.
/// 
/// # approach
/// 1. Initialize a pointer k to track the position for the next valid element
/// 2. Iterate through the array with pointer i
/// 3. When nums[i] != val, copy the element to position k and increment k
/// 4. Return k as the new length of the array
/// 
/// # complexity
/// - Time complexity: O(n), where n is the length of the array
/// - Space complexity: O(1), only constant extra space is used
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k: usize = 0;
        
        for i in 0..nums.len() {
            if nums[i] != val {
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
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(Solution::remove_element(&mut nums, val), 2);
        assert_eq!(nums[0..2], [2, 2]);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        assert_eq!(Solution::remove_element(&mut nums, val), 5);
        assert_eq!(nums[0..5], [0, 1, 3, 0, 4]);
    }
}
