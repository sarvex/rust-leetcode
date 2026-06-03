use std::collections::HashSet;

impl Solution {
    /// Find the length of the longest common prefix between any pair from arr1 and arr2.
    ///
    /// # Intuition
    /// Store all numeric prefixes of arr1 in a HashSet. For each arr2 number, strip
    /// digits from the right (shortest prefix last) while tracking the digit count,
    /// so the first HashSet hit immediately gives both the match and its length —
    /// no second pass needed.
    ///
    /// # Approach
    /// 1. Compute the digit count of each arr1 number once, then insert all its
    ///    prefixes into a HashSet<i32>.
    /// 2. For each arr2 number, compute its digit count, then scan from the full
    ///    number down to a single digit. On the first HashSet hit, record the current
    ///    digit count as the prefix length and break.
    /// 3. Return the maximum length seen.
    ///
    /// # Complexity
    /// - Time: O((m + n) × d) where d ≤ 9 (max digits for values ≤ 10^8)
    /// - Space: O(m × d) for the prefix set
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        // Pre-size: each number contributes at most 9 prefixes.
        let mut prefixes = HashSet::with_capacity(arr1.len() * 9);

        for mut num in arr1 {
            while num > 0 {
                prefixes.insert(num);
                num /= 10;
            }
        }

        arr2.into_iter()
            .map(|num| {
                // Compute digit count once via ilog10 (no loop).
                let mut digits = num.ilog10() as i32 + 1;
                let mut cur = num;
                while cur > 0 {
                    if prefixes.contains(&cur) {
                        return digits;
                    }
                    cur /= 10;
                    digits -= 1;
                }
                0
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]),
            0
        );
    }

    #[test]
    fn test_no_overlap() {
        assert_eq!(
            Solution::longest_common_prefix(vec![123, 456], vec![789, 321]),
            0
        );
    }

    #[test]
    fn test_single_digit_match() {
        assert_eq!(Solution::longest_common_prefix(vec![5], vec![5678]), 1);
    }

    #[test]
    fn test_full_number_match() {
        assert_eq!(
            Solution::longest_common_prefix(vec![12345678], vec![12345678]),
            8
        );
    }

    #[test]
    fn test_multiple_pairs_longest_wins() {
        // (12, 1200) -> prefix 12 (len 2), (9, 9999) -> prefix 9 (len 1)
        assert_eq!(
            Solution::longest_common_prefix(vec![12, 9], vec![1200, 9999]),
            2
        );
    }

    #[test]
    fn test_max_value() {
        // arr1[i], arr2[i] <= 10^8 = 100_000_000 (9 digits)
        assert_eq!(
            Solution::longest_common_prefix(vec![100_000_000], vec![100_000_001]),
            9
        );
    }
}
