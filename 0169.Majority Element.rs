/// Finds the majority element using Boyer-Moore Voting Algorithm
/// 
/// # intuition
/// The majority element appears more than n/2 times, so its count will always be positive
/// after canceling out with other elements.
/// 
/// # approach
/// 1. Initialize a candidate element and its count
/// 2. Iterate through the array:
///    - If count is 0, set the current element as the new candidate
///    - If current element matches candidate, increment count
///    - Otherwise, decrement count
/// 3. Return the final candidate (guaranteed to be the majority element)
/// 
/// # complexity
/// - Time complexity: O(n) where n is the length of the array
/// - Space complexity: O(1) as we only use two variables
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate: i32 = 0;
        let mut count: i32 = 0;
        
        for &num in nums.iter() {
            if count == 0 {
                candidate = num;
                count = 1;
            } else if candidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
        assert_eq!(Solution::majority_element(vec![1]), 1);
    }
}
