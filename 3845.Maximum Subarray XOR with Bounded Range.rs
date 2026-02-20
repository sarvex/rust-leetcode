use std::collections::VecDeque;

impl Solution {
    /// Complete binary trie with implicit indexing for maximum XOR subarray where max-min <= k.
    ///
    /// # Intuition
    /// Given a non-negative integer array nums and an integer k,
    /// select a subarray such that the difference between its maximum and minimum elements is at most k.
    /// The value of this subarray is the bitwise XOR of all elements in the subarray.
    /// Return the maximum possible value of the selected subarray.
    ///
    /// The problem combines two constraints: (1) max-min <= k for valid subarrays, and (2) maximize XOR value.
    /// For XOR maximization with dynamic elements, a Trie is ideal. For the max-min constraint,
    /// sliding window with monotonic deques efficiently tracks min/max as the window expands/contracts.
    ///
    /// # Approach
    /// 1. Compute prefix XOR array where prefix[i] = nums[0] ^ nums[1] ^ ... ^ nums[i-1]
    ///    XOR of subarray [l, r] = prefix[r+1] ^ prefix[l]
    ///
    /// 2. Use sliding window [left, right] maintained by two deques:
    ///    - Min deque: monotonically increasing to track minimum
    ///    - Max deque: monotonically decreasing to track maximum
    ///    Shrink left when max - min > k
    ///
    /// 3. Maintain a complete binary trie of prefix[l] values for all valid start positions l.
    ///    The trie uses implicit indexing: node at index i has children at 2*i and 2*i+1.
    ///    Each node stores a reference count of how many numbers pass through it.
    ///
    /// 4. For each right, query the trie with prefix[right+1] to find the maximum XOR.
    ///    To maximize XOR, try to take the opposite bit at each level if that child has count > 0.
    ///
    /// # Complexity
    /// - Time: O(n * 15) = O(n) where n is the length of nums. Each element is added/removed from
    ///         deques and trie at most once, and trie operations are O(15) for 15-bit numbers.
    /// - Space: O(2^16) for the fixed-size complete binary trie.
    pub fn max_xor(nums: Vec<i32>, k: i32) -> i32 {
        // Complete binary trie with implicit indexing (like binary heap)
        // For 15-bit numbers, we need a trie of height 15, total nodes = 2^16 - 1
        // trie[i] = count of numbers passing through node i
        let mut trie = vec![0; 1 << 16];

        // Monotonic deques storing (value, index) pairs
        let mut max_deque: VecDeque<(i32, usize)> = VecDeque::new();
        let mut min_deque: VecDeque<(i32, usize)> = VecDeque::new();

        // Prefix XOR array: prefix[i] = nums[0] ^ ... ^ nums[i-1]
        let mut prefix = vec![0; nums.len() + 1];

        let mut left = 0; // Left boundary of sliding window
        let mut ans = 0;

        // Insert initial prefix[0] = 0 into trie
        let mut idx = 1;
        for _ in (0..15).rev() {
            idx = 2 * idx;
            trie[idx] += 1;
        }

        for (right, &val) in nums.iter().enumerate() {
            // Update prefix XOR
            prefix[right + 1] = prefix[right] ^ val;

            // Insert prefix[right+1] into trie
            idx = 1;
            for bit_pos in (0..15).rev() {
                let bit = ((prefix[right + 1] >> bit_pos) & 1) as usize;
                idx = 2 * idx + bit;
                trie[idx] += 1;
            }

            // Update max deque (monotonically decreasing)
            while let Some(&(max_val, _)) = max_deque.back() {
                if max_val < val {
                    max_deque.pop_back();
                } else {
                    break;
                }
            }
            max_deque.push_back((val, right));

            // Update min deque (monotonically increasing)
            while let Some(&(min_val, _)) = min_deque.back() {
                if min_val > val {
                    min_deque.pop_back();
                } else {
                    break;
                }
            }
            min_deque.push_back((val, right));

            // Shrink window while constraint is violated
            while max_deque.front().unwrap().0 - min_deque.front().unwrap().0 > k {
                // Remove prefix[left] from trie
                let remove_val = prefix[left];
                idx = 1;
                for bit_pos in (0..15).rev() {
                    let bit = ((remove_val >> bit_pos) & 1) as usize;
                    idx = 2 * idx + bit;
                    trie[idx] -= 1;
                }

                // Remove elements that are no longer in window from deques
                while let Some(&(_, idx)) = max_deque.front() {
                    if left >= idx {
                        max_deque.pop_front();
                    } else {
                        break;
                    }
                }
                while let Some(&(_, idx)) = min_deque.front() {
                    if left >= idx {
                        min_deque.pop_front();
                    } else {
                        break;
                    }
                }

                left += 1;
            }

            // Query maximum XOR for subarrays ending at right
            // XOR of subarray [l, right] = prefix[right+1] ^ prefix[l]
            let mut max_xor = 0;
            idx = 1;
            let query_val = prefix[right + 1];
            for bit_pos in (0..15).rev() {
                let bit = ((query_val >> bit_pos) & 1) as usize;
                // Try to go to opposite bit for maximum XOR
                if trie[2 * idx + (1 - bit)] > 0 {
                    max_xor |= 1 << bit_pos;
                    idx = 2 * idx + (1 - bit);
                } else {
                    idx = 2 * idx + bit;
                }
            }
            ans = ans.max(max_xor);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![5, 4, 5, 6];
        let k = 2;
        assert_eq!(Solution::max_xor(nums, k), 7);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![5, 4, 5, 6];
        let k = 1;
        assert_eq!(Solution::max_xor(nums, k), 6);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![7];
        let k = 0;
        assert_eq!(Solution::max_xor(nums, k), 7);
    }

    #[test]
    fn test_all_same_elements() {
        let nums = vec![5, 5, 5, 5];
        let k = 0;
        assert_eq!(Solution::max_xor(nums, k), 5);
    }

    #[test]
    fn test_large_k() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 100;
        assert_eq!(Solution::max_xor(nums, k), 7);
    }

    #[test]
    fn test_k_zero() {
        let nums = vec![1, 2, 1, 3, 1];
        let k = 0;
        assert_eq!(Solution::max_xor(nums, k), 3);
    }

    #[test]
    fn test_empty_subarray_not_allowed() {
        let nums = vec![0, 0, 0];
        let k = 0;
        assert_eq!(Solution::max_xor(nums, k), 0);
    }

    #[test]
    fn test_xor_properties() {
        let nums = vec![7, 7];
        let k = 0;
        assert_eq!(Solution::max_xor(nums, k), 7);
    }

    #[test]
    fn test_high_bits() {
        let nums = vec![16383, 16384];
        let k = 1;
        assert_eq!(Solution::max_xor(nums, k), 32767);
    }

    #[test]
    fn test_bug_3_3_4_k0() {
        let nums = vec![3, 3, 4];
        let k = 0;
        assert_eq!(Solution::max_xor(nums, k), 4);
    }
}
