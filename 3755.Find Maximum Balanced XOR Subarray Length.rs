use std::collections::HashMap;

impl Solution {
    /// Find the longest subarray with XOR=0 and equal even/odd count using prefix state hashing.
    ///
    /// # Intuition
    /// For subarray [l, r] to have XOR=0: prefix_xor[l] == prefix_xor[r+1]
    /// For equal even/odd: (#even - #odd) must be equal at both boundaries
    /// Both conditions together mean we need matching (xor, parity) state pairs.
    ///
    /// # Approach
    /// 1. Track running XOR and parity difference (evens - odds)
    /// 2. Use HashMap with combined i64 key (xor << 32 | parity) for O(1) lookups
    /// 3. Preallocate HashMap capacity to avoid rehashing
    /// 4. When state repeats, compute length and update max
    ///
    /// # Complexity
    /// - Time: O(n) - single pass through array
    /// - Space: O(n) - HashMap stores at most n entries
    pub fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
        // Combined key: (xor as i64) << 32 | (parity as u32 as i64)
        // This avoids tuple hashing overhead
        let mut first_occurrence: HashMap<i64, usize> = HashMap::with_capacity(nums.len() + 1);
        
        // Initial state: xor=0, parity=0 at index 0 (before any elements)
        first_occurrence.insert(0, 0);
        
        let mut xor = 0i32;
        let mut parity = 0i32;
        let mut max_len = 0;
        
        for (i, &num) in nums.iter().enumerate() {
            let idx = i + 1;
            
            xor ^= num;
            parity += if num & 1 == 0 { 1 } else { -1 };
            
            // Combine xor and parity into single i64 key
            // Cast to u32 first to avoid sign extension, then combine
            let key = ((xor as u32 as i64) << 32) | (parity as u32 as i64);
            
            if let Some(&first_idx) = first_occurrence.get(&key) {
                let len = idx - first_idx;
                if len > max_len {
                    max_len = len;
                }
            } else {
                first_occurrence.insert(key, idx);
            }
        }
        
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 1, 3, 2, 0];
        assert_eq!(Solution::max_balanced_subarray(nums), 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 2, 8, 5, 4, 14, 9, 15];
        assert_eq!(Solution::max_balanced_subarray(nums), 8);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![0];
        assert_eq!(Solution::max_balanced_subarray(nums), 0);
    }

    #[test]
    fn test_no_valid_subarray() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::max_balanced_subarray(nums), 0);
    }

    #[test]
    fn test_two_elements_xor_zero() {
        let nums = vec![5, 5];
        assert_eq!(Solution::max_balanced_subarray(nums), 0);
    }

    #[test]
    fn test_all_zeros() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(Solution::max_balanced_subarray(nums), 0);
    }

    #[test]
    fn test_large_case() {
        let nums: Vec<i32> = (0..100000).map(|i| if i % 2 == 0 { 0 } else { 1 }).collect();
        let result = Solution::max_balanced_subarray(nums);
        assert!(result >= 0);
    }
}
