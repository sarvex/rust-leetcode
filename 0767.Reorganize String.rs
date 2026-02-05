use std::collections::BinaryHeap;

impl Solution {
    /// Reorganizes a string so no two adjacent characters are the same.
    ///
    /// # Intuition
    /// Greedily place the most frequent character, then delay it by one step
    /// to avoid adjacency. A max-heap ensures the most frequent character
    /// is always chosen next.
    ///
    /// # Approach
    /// Count character frequencies. If any frequency exceeds `(n+1)/2`, return
    /// empty. Use a max-heap with a cooldown queue of size 1 to alternate
    /// characters.
    ///
    /// # Complexity
    /// - Time: O(n log k) where k is the number of distinct characters
    /// - Space: O(k) for the heap and counts
    pub fn reorganize_string(s: String) -> String {
        let n = s.len();
        let freq = s.bytes().fold([0usize; 26], |mut freq, b| {
            freq[(b - b'a') as usize] += 1;
            freq
        });

        if freq.iter().any(|&f| 2 * f - 1 > n) {
            return String::new();
        }

        let mut heap: BinaryHeap<(usize, u8)> = freq
            .iter()
            .enumerate()
            .filter(|(_, f)| **f > 0)
            .map(|(i, f)| (*f, i as u8 + b'a'))
            .collect();

        let mut result = String::with_capacity(n);
        let mut prev: Option<(usize, u8)> = None;

        while let Some((count, ch)) = heap.pop() {
            result.push(ch as char);
            if let Some(p) = prev.take() {
                if p.0 > 0 {
                    heap.push(p);
                }
            }
            prev = Some((count - 1, ch));
        }

        if result.len() == n {
            result
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(s: &str) -> bool {
        s.as_bytes().windows(2).all(|w| w[0] != w[1])
    }

    #[test]
    fn test_possible() {
        let result = Solution::reorganize_string("aab".to_string());
        assert_eq!(result.len(), 3);
        assert!(is_valid(&result));
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::reorganize_string("aaab".to_string()), "");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::reorganize_string("a".to_string()), "a");
    }

    #[test]
    fn test_two_chars() {
        let result = Solution::reorganize_string("ab".to_string());
        assert!(is_valid(&result));
        assert_eq!(result.len(), 2);
    }
}
