impl Solution {
    /// Build the lexicographically smallest valid number by flushing reversed blocks.
    ///
    /// # Intuition
    /// The smallest prefix comes from assigning the next smallest unused digit as early as possible.
    /// A run of `D` constraints requires a descending segment, so we temporarily store digits and
    /// emit them in reverse once we hit an `I` (or the end).
    ///
    /// # Approach
    /// 1. Iterate `i` from `0` to `n` and push digit `i + 1` onto a stack.
    /// 2. If `i == n` or `pattern[i] == 'I'`, flush the stack in reverse into the answer.
    /// 3. Convert collected ASCII digits to `String`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn smallest_number(pattern: String) -> String {
        let pattern_bytes = pattern.as_bytes();
        let n = pattern_bytes.len();

        let mut stack = Vec::with_capacity(n + 1);
        let mut answer = Vec::with_capacity(n + 1);

        (0..=n).for_each(|i| {
            stack.push(b'1' + i as u8);

            if i == n || pattern_bytes[i] == b'I' {
                answer.extend(stack.iter().rev().copied());
                stack.clear();
            }
        });

        answer.into_iter().map(char::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::smallest_number("IIIDIDDD".to_string()),
            "123549876".to_string()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::smallest_number("DDD".to_string()), "4321".to_string());
    }

    #[test]
    fn all_increasing() {
        assert_eq!(Solution::smallest_number("IIII".to_string()), "12345".to_string());
    }

    #[test]
    fn alternating_pattern() {
        assert_eq!(Solution::smallest_number("IDID".to_string()), "13254".to_string());
    }

    #[test]
    fn max_length_all_decreasing() {
        assert_eq!(
            Solution::smallest_number("DDDDDDDD".to_string()),
            "987654321".to_string()
        );
    }
}
