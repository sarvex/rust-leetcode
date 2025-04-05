/// Rotates an array to the right by k steps using the reverse technique.
///
/// # intuition
/// When we rotate an array by k steps, we're essentially moving the last k elements to the front.
/// This can be achieved efficiently using a three-step reversal technique:
/// 1. Reverse the entire array
/// 2. Reverse the first k elements
/// 3. Reverse the remaining elements
///
/// # approach
/// 1. Calculate effective rotation (k % array length) to handle cases where k > array length
/// 2. Reverse the entire array
/// 3. Reverse the first k elements
/// 4. Reverse the remaining elements
///
/// # complexity
/// Time complexity: O(n) where n is the length of the array
/// Space complexity: O(1) as we modify the array in-place
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k as usize) % n;
        
        // Early return if no rotation needed or empty array
        if k == 0 || n <= 1 {
            return;
        }
        
        // Three-step reversal technique
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut nums1 = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums1, 3);
        assert_eq!(nums1, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut nums2 = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums2, 2);
        assert_eq!(nums2, vec![3, 99, -1, -100]);
    }
}
