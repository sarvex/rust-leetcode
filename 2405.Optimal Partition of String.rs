impl Solution {
    /// Partitions string into minimum substrings with unique characters.
    ///
    /// # Intuition
    /// Greedily extend the current partition until a duplicate character appears,
    /// then start a new partition. A bitmask efficiently tracks seen characters.
    ///
    /// # Approach
    /// 1. Use a 26-bit mask to track characters in the current partition
    /// 2. For each character, check if its bit is already set
    /// 3. If duplicate found, increment partition count and reset mask
    /// 4. Set the current character's bit in the mask
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn partition_string(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .map(|&c| (c - b'a') as u32)
            .fold((1, 0u32), |(ans, mask), x| {
                if mask >> x & 1 == 1 {
                    (ans + 1, 1 << x)
                } else {
                    (ans, mask | (1 << x))
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::partition_string("abacaba".to_string()), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::partition_string("ssssss".to_string()), 6);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(Solution::partition_string("abcdef".to_string()), 1);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::partition_string("a".to_string()), 1);
    }

    #[test]
    fn test_two_same() {
        assert_eq!(Solution::partition_string("aa".to_string()), 2);
    }
}
